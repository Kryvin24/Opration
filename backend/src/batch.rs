//! Batch command execution across many SSH nodes.
//!
//! One request fans out `tsh ssh --login <login> <host> <command>` to many
//! nodes using a fixed worker pool (see pool.rs), with a per-node timeout and
//! capped output. Each finished node is streamed to the UI as a `batch/item`
//! event and the blocking call returns the full summary once every node is
//! done.
//!
//! The same engine powers liveness probes (`teleport/probe`): `tsh ls` JSON is
//! only the cluster's *desired* state and carries no realtime liveness flag, so
//! the only reliable way to tell whether a reverse-tunnel node is alive is to
//! actually run a command on it.
//!
//! Item payload shape (event params and summary rows):
//! ```json
//! { "batch_id": "...", "kind": "exec|probe", "host": "...",
//!   "status": "ok|fail|timeout|error", "exit_code": 0,
//!   "duration_ms": 123, "stdout": "...", "stderr": "..." }
//! ```

use std::time::Instant;

use dbx_plugin_sdk::{trace, PluginEmitter};
use serde_json::{json, Value};

use crate::pool;

const STDOUT_CAP: usize = 131_072; // 128 KiB
const STDERR_CAP: usize = 65_536; // 64 KiB

#[derive(Clone)]
pub struct BatchOptions {
    pub tsh: String,
    pub login: String,
    pub hosts: Vec<String>,
    pub command: String,
    pub concurrency: usize,
    pub timeout_secs: u64,
    pub batch_id: String,
    /// "exec" for user commands, "probe" for liveness checks.
    pub kind: String,
    /// Probe pass: 1 = broad first pass, 2 = slow/retried confirmation.
    pub phase: u8,
}

/// Run one command on every host in `opts`. Returns a summary Value.
pub fn execute(opts: BatchOptions, emitter: PluginEmitter) -> Value {
    let total = opts.hosts.len();
    let started = Instant::now();
    trace(&format!(
        "batch[{}] start kind={} hosts={} concurrency={} timeout={}s cmd={}",
        opts.batch_id, opts.kind, total, opts.concurrency, opts.timeout_secs, opts.command
    ));

    let tsh = opts.tsh.clone();
    let login = opts.login.clone();
    let command = opts.command.clone();
    let batch_id = opts.batch_id.clone();
    let kind = opts.kind.clone();
    let phase = opts.phase;
    let timeout_secs = opts.timeout_secs;

    let results = pool::run_pool(
        opts.hosts.clone(),
        opts.concurrency,
        50,
        emitter,
        "batch/item",
        move |host| run_one(&tsh, &login, host, &command, timeout_secs, &batch_id, &kind, phase),
    );

    let ok_count = results
        .iter()
        .filter(|r| r.get("status").and_then(|v| v.as_str()) == Some("ok"))
        .count();
    let summary = json!({
        "batch_id": opts.batch_id,
        "kind": opts.kind,
        "total": total,
        "ok": ok_count,
        "failed": total - ok_count,
        "duration_ms": started.elapsed().as_millis() as u64,
        "results": results,
    });
    trace(&format!(
        "batch[{}] done total={} ok={} failed={} in {}ms",
        opts.batch_id, ok_count, total,
        total - ok_count,
        summary.get("duration_ms").and_then(|v| v.as_u64()).unwrap_or(0)
    ));
    summary
}

/// Execute the command on a single node with timeout and output caps.
fn run_one(
    tsh: &str,
    login: &str,
    host: &str,
    command: &str,
    timeout_secs: u64,
    batch_id: &str,
    kind: &str,
    phase: u8,
) -> Value {
    let started = Instant::now();
    let out = match pool::run_tsh_ssh(tsh, login, host, command, timeout_secs, STDOUT_CAP, STDERR_CAP) {
        Ok(o) => o,
        Err(e) => {
            return json!({
                "batch_id": batch_id, "kind": kind, "phase": phase, "host": host,
                "status": "error", "exit_code": Value::Null,
                "duration_ms": started.elapsed().as_millis() as u64,
                "stdout": "", "stderr": format!("spawn failed: {e}"),
            });
        }
    };

    let mut stdout = String::from_utf8_lossy(&out.stdout).into_owned();
    let mut stderr = String::from_utf8_lossy(&out.stderr).into_owned();
    if out.stdout_truncated {
        stdout.push_str("\n…[输出已截断]");
    }
    if out.stderr_truncated {
        stderr.push_str("\n…[输出已截断]");
    }

    let status = if out.timed_out {
        "timeout"
    } else if out.exit_code == Some(0) {
        "ok"
    } else {
        "fail"
    };

    json!({
        "batch_id": batch_id,
        "kind": kind,
        "phase": phase,
        "host": host,
        "status": status,
        "exit_code": out.exit_code,
        "duration_ms": out.duration_ms,
        "stdout": stdout,
        "stderr": stderr,
    })
}
