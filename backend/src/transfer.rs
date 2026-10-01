//! File transfers over Teleport (`tsh scp`).
//!
//! Upload fans one local path out to many nodes with a fixed worker pool;
//! each finished node is streamed to the UI as a `transfer/item` event and
//! the blocking call returns a summary once every node is done. Download is a
//! single-node pull that runs synchronously. Both honor a per-item timeout
//! (satellite links are slow, especially pulling large logs).
//!
//! Remote arguments use the bare `node:path` form together with `--login`,
//! matching what `tsh scp` accepts for nodes in the current profile.
//!
//! Item payload shape:
//! ```json
//! { "transfer_id": "...", "direction": "upload|download", "node": "...",
//!   "status": "ok|fail|timeout|error", "duration_ms": 123, "stderr": "..." }
//! ```

use std::io::Read;
use std::process::{Command, Stdio};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use dbx_plugin_sdk::{trace, PluginEmitter};
use serde_json::{json, Value};

const STDERR_CAP: usize = 16_384;

#[derive(Clone)]
pub struct UploadOptions {
    pub tsh: String,
    pub login: String,
    pub nodes: Vec<String>,
    pub local_path: String,
    pub remote_path: String,
    pub recursive: bool,
    pub concurrency: usize,
    pub timeout_secs: u64,
    pub transfer_id: String,
}

/// Push one local path to every node in `opts`. Returns a summary Value.
pub fn upload(opts: UploadOptions, emitter: PluginEmitter) -> Value {
    let total = opts.nodes.len();
    let started = Instant::now();
    trace(&format!(
        "transfer[{}] upload nodes={} local={} remote={} recursive={}",
        opts.transfer_id, total, opts.local_path, opts.remote_path, opts.recursive
    ));

    let (task_tx, task_rx) = mpsc::channel::<String>();
    let (res_tx, res_rx) = mpsc::channel::<Value>();
    let task_rx = Arc::new(Mutex::new(task_rx));

    let workers = opts
        .concurrency
        .clamp(1, 20)
        .min(total.max(1));

    let mut handles = Vec::new();
    for _ in 0..workers {
        let rx = task_rx.clone();
        let tx = res_tx.clone();
        let em = emitter.clone();
        let tsh = opts.tsh.clone();
        let login = opts.login.clone();
        let local = opts.local_path.clone();
        let remote = opts.remote_path.clone();
        let recursive = opts.recursive;
        let transfer_id = opts.transfer_id.clone();
        let timeout_secs = opts.timeout_secs;
        handles.push(thread::spawn(move || loop {
            let node = match rx.lock() {
                Ok(guard) => match guard.recv() {
                    Ok(n) => n,
                    Err(_) => break,
                },
                Err(_) => break,
            };
            let item = run_scp(
                &tsh, &login, &node, &local, &remote,
                recursive, timeout_secs, &transfer_id, "upload",
            );
            let _ = em.event("transfer/item", item.clone());
            let _ = tx.send(item);
        }));
    }
    drop(res_tx);
    for node in opts.nodes {
        let _ = task_tx.send(node);
    }
    drop(task_tx);

    let mut results: Vec<Value> = Vec::with_capacity(total);
    for item in res_rx {
        results.push(item);
    }
    for h in handles {
        let _ = h.join();
    }

    let ok_count = results
        .iter()
        .filter(|r| r.get("status").and_then(|v| v.as_str()) == Some("ok"))
        .count();
    let summary = json!({
        "transfer_id": opts.transfer_id,
        "direction": "upload",
        "total": total,
        "ok": ok_count,
        "failed": total - ok_count,
        "duration_ms": started.elapsed().as_millis() as u64,
        "results": results,
    });
    trace(&format!(
        "transfer[{}] upload done total={} ok={} failed={}",
        opts.transfer_id, total, ok_count, total - ok_count
    ));
    summary
}

/// Pull one remote path from a single node into a local path.
pub fn download(
    tsh: &str,
    login: &str,
    node: &str,
    remote_path: &str,
    local_path: &str,
    recursive: bool,
    timeout_secs: u64,
    transfer_id: &str,
) -> Value {
    // Remote is the source, local the destination.
    run_scp(
        tsh, login, node, local_path, remote_path,
        recursive, timeout_secs, transfer_id, "download",
    )
}

/// Run one `tsh scp`. `local` and `remote` are ordered by the caller: for
/// upload local=source/remote=destination spec; for download reversed.
#[allow(clippy::too_many_arguments)]
fn run_scp(
    tsh: &str,
    login: &str,
    node: &str,
    local: &str,
    remote: &str,
    recursive: bool,
    timeout_secs: u64,
    transfer_id: &str,
    direction: &str,
) -> Value {
    let started = Instant::now();

    // Remote spec is node:path for upload destinations and node:path for
    // download sources.
    let remote_spec = format!("{node}:{remote}");
    let mut args: Vec<String> = vec![
        "scp".to_string(),
        "--login".to_string(), login.to_string(),
    ];
    if recursive {
        args.push("-r".to_string());
    }
    if direction == "upload" {
        args.push(local.to_string());
        args.push(remote_spec);
    } else {
        args.push(remote_spec);
        args.push(local.to_string());
    }

    let spawn_result = Command::new(tsh)
        .args(&args)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn();

    let mut child = match spawn_result {
        Ok(c) => c,
        Err(e) => {
            return json!({
                "transfer_id": transfer_id, "direction": direction, "node": node,
                "status": "error", "duration_ms": started.elapsed().as_millis() as u64,
                "stderr": format!("spawn failed: {e}"),
            });
        }
    };

    let stderr_handle = child.stderr.take().map(|r| {
        thread::spawn(move || drain_capped(r, STDERR_CAP))
    });
    let stdout_handle = child.stdout.take().map(|r| {
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

    let stderr_bytes = stderr_handle
        .and_then(|h| h.join().ok())
        .map(|(b, _)| b)
        .unwrap_or_default();
    let _ = stdout_handle.and_then(|h| h.join().ok());

    // tsh prints its progress bar to stderr using \r; on success that is all
    // it contains. Normalize and keep it for failure reporting.
    let raw = String::from_utf8_lossy(&stderr_bytes).into_owned();
    let stderr = clean_scp_stderr(&raw);

    let status = if timed_out {
        "timeout"
    } else if exit_code == Some(0) {
        "ok"
    } else {
        "fail"
    };

    json!({
        "transfer_id": transfer_id,
        "direction": direction,
        "node": node,
        "status": status,
        "exit_code": exit_code,
        "duration_ms": started.elapsed().as_millis() as u64,
        // Success carries only the progress bar noise; drop it to keep the
        // report clean.
        "stderr": if status == "ok" { String::new() } else { stderr },
    })
}

/// Turn tsh's carriage-return progress output into a compact message. A line
/// is treated as progress-bar noise only when it contains a block character or
/// is a bare percentage; real error text (it has no block glyphs) is kept.
fn clean_scp_stderr(raw: &str) -> String {
    let mut lines: Vec<String> = Vec::new();
    for part in raw.split(['\r', '\n']) {
        let p = part.trim();
        if p.is_empty() {
            continue;
        }
        let is_bar = p.contains('█')
            || p.chars().all(|c| c.is_ascii_digit() || matches!(c, '%' | ' '));
        if is_bar {
            continue;
        }
        lines.push(p.to_string());
    }
    // Errors often repeat as the bar redraws; dedup consecutive duplicates.
    let mut out: Vec<String> = Vec::new();
    for l in lines {
        if out.last() != Some(&l) {
            out.push(l);
        }
    }
    out.join("\n")
}

/// Read a stream until EOF, storing at most `cap` bytes.
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
