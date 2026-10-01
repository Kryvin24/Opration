//! Batch command execution across many SSH nodes.
//!
//! One request fans out `tsh ssh --login <login> <host> <command>` to many
//! nodes using a fixed worker pool, with a per-node timeout and capped output.
//! Each finished node is streamed to the UI as a `batch/item` event and the
//! blocking call returns the full summary once every node is done.
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

use std::io::Read;
use std::process::{Command, Stdio};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use dbx_plugin_sdk::{trace, PluginEmitter};
use serde_json::{json, Value};

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

    let (task_tx, task_rx) = mpsc::channel::<String>();
    let (res_tx, res_rx) = mpsc::channel::<Value>();
    let task_rx = Arc::new(Mutex::new(task_rx));

    let workers = opts
        .concurrency
        .clamp(1, 50)
        .min(total.max(1));

    let mut handles = Vec::new();
    for _ in 0..workers {
        let rx = task_rx.clone();
        let tx = res_tx.clone();
        let em = emitter.clone();
        let tsh = opts.tsh.clone();
        let login = opts.login.clone();
        let command = opts.command.clone();
        let batch_id = opts.batch_id.clone();
        let kind = opts.kind.clone();
        let phase = opts.phase;
        let timeout_secs = opts.timeout_secs;
        handles.push(thread::spawn(move || loop {
            // The lock is held only for the recv call, never across execution.
            let host = match rx.lock() {
                Ok(guard) => match guard.recv() {
                    Ok(h) => h,
                    Err(_) => break,
                },
                Err(_) => break,
            };
            let item = run_one(&tsh, &login, &host, &command, timeout_secs, &batch_id, &kind, phase);
            let _ = em.event("batch/item", item.clone());
            let _ = tx.send(item);
        }));
    }
    // Original sender/receiver clones must be dropped so res_rx terminates.
    drop(res_tx);
    for host in opts.hosts {
        let _ = task_tx.send(host);
    }
    drop(task_tx);

    let mut results: Vec<Value> = Vec::with_capacity(total);
    for item in res_rx {
        results.push(item);
    }
    for handle in handles {
        let _ = handle.join();
    }

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
    let spawn_result = Command::new(tsh)
        .args(["ssh", "--login", login, host, command])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn();

    let mut child = match spawn_result {
        Ok(c) => c,
        Err(e) => {
            return json!({
                "batch_id": batch_id, "kind": kind, "phase": phase, "host": host,
                "status": "error", "exit_code": Value::Null,
                "duration_ms": started.elapsed().as_millis() as u64,
                "stdout": "", "stderr": format!("spawn failed: {e}"),
            });
        }
    };

    // Drain both pipes on dedicated threads so a full pipe can never deadlock
    // the child, and keep reading past the cap so the stream stays drained.
    let stdout_handle = child.stdout.take().map(|r| {
        thread::spawn(move || drain_capped(r, STDOUT_CAP))
    });
    let stderr_handle = child.stderr.take().map(|r| {
        thread::spawn(move || drain_capped(r, STDERR_CAP))
    });

    let timeout = Duration::from_secs(timeout_secs.max(1));
    let mut timed_out = false;
    let mut exit_code: Option<i32> = None;
    loop {
        match child.try_wait() {
            Ok(Some(status)) => {
                exit_code = status.code();
                break;
            }
            Ok(None) => {
                if started.elapsed() >= timeout {
                    timed_out = true;
                    let _ = child.kill();
                    let _ = child.wait();
                    break;
                }
                thread::sleep(Duration::from_millis(120));
            }
            Err(_) => break,
        }
    }

    let (stdout_bytes, stdout_truncated) = stdout_handle
        .and_then(|h| h.join().ok())
        .unwrap_or((Vec::new(), false));
    let (stderr_bytes, stderr_truncated) = stderr_handle
        .and_then(|h| h.join().ok())
        .unwrap_or((Vec::new(), false));

    let mut stdout = String::from_utf8_lossy(&stdout_bytes).into_owned();
    let mut stderr = String::from_utf8_lossy(&stderr_bytes).into_owned();
    if stdout_truncated {
        stdout.push_str("\n…[输出已截断]");
    }
    if stderr_truncated {
        stderr.push_str("\n…[输出已截断]");
    }

    let status = if timed_out {
        "timeout"
    } else if exit_code == Some(0) {
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
        "exit_code": exit_code,
        "duration_ms": started.elapsed().as_millis() as u64,
        "stdout": stdout,
        "stderr": stderr,
    })
}

/// Read a stream until EOF, storing at most `cap` bytes. Returns the stored
/// bytes and whether more data followed.
fn drain_capped<R: Read>(mut reader: R, cap: usize) -> (Vec<u8>, bool) {
    let mut chunk = [0u8; 4096];
    let mut out = Vec::new();
    let mut truncated = false;
    loop {
        match reader.read(&mut chunk) {
            Ok(0) | Err(_) => break,
            Ok(n) => {
                if out.len() < cap {
                    let take = n.min(cap - out.len());
                    out.extend_from_slice(&chunk[..take]);
                    if take < n {
                        truncated = true;
                    }
                } else {
                    truncated = true;
                }
            }
        }
    }
    (out, truncated)
}
