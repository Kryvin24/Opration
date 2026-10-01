//! Shared fan-out plumbing for features that run one `tsh ssh` per node.
//!
//! Two pieces live here:
//!   - [`run_pool`]: fixed worker pool over a task list, streaming each
//!     finished task to the UI as an event and returning all results.
//!   - [`run_tsh_ssh`]: spawn one `tsh ssh` with drained/capped pipes and a
//!     hard timeout (kill + reap), returning the raw output.
//!
//! Used by batch.rs (exec/probe) and inspect.rs (metrics collection).

use std::io::Read;
use std::process::{Command, Stdio};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use dbx_plugin_sdk::PluginEmitter;
use serde_json::Value;

/// Fan `tasks` out over a fixed worker pool. `handle` produces one Value per
/// task; every Value is also emitted as `event`. Returns all Values in
/// completion order.
pub fn run_pool<F>(
    tasks: Vec<String>,
    concurrency: usize,
    max_concurrency: usize,
    emitter: PluginEmitter,
    event: &'static str,
    handle: F,
) -> Vec<Value>
where
    F: Fn(&str) -> Value + Send + Sync + 'static,
{
    let total = tasks.len();
    let (task_tx, task_rx) = mpsc::channel::<String>();
    let (res_tx, res_rx) = mpsc::channel::<Value>();
    let task_rx = Arc::new(Mutex::new(task_rx));
    let handle = Arc::new(handle);

    let workers = concurrency.clamp(1, max_concurrency).min(total.max(1));
    let mut handles = Vec::new();
    for _ in 0..workers {
        let rx = task_rx.clone();
        let tx = res_tx.clone();
        let em = emitter.clone();
        let f = handle.clone();
        handles.push(thread::spawn(move || loop {
            // The lock is held only for the recv call, never across execution.
            let task = match rx.lock() {
                Ok(guard) => match guard.recv() {
                    Ok(t) => t,
                    Err(_) => break,
                },
                Err(_) => break,
            };
            let item = f(&task);
            let _ = em.event(event, item.clone());
            let _ = tx.send(item);
        }));
    }
    // Original sender/receiver clones must be dropped so res_rx terminates.
    drop(res_tx);
    for t in tasks {
        let _ = task_tx.send(t);
    }
    drop(task_tx);

    let mut results = Vec::with_capacity(total);
    for item in res_rx {
        results.push(item);
    }
    for h in handles {
        let _ = h.join();
    }
    results
}

pub struct ChildOutput {
    pub timed_out: bool,
    pub exit_code: Option<i32>,
    pub stdout: Vec<u8>,
    pub stdout_truncated: bool,
    pub stderr: Vec<u8>,
    pub stderr_truncated: bool,
    pub duration_ms: u64,
}

/// Spawn `tsh ssh --login <login> <host> <command>` with both pipes drained on
/// dedicated threads (capped, so a chatty node can't exhaust memory) and a
/// hard timeout that kills AND reaps the child.
pub fn run_tsh_ssh(
    tsh: &str,
    login: &str,
    host: &str,
    command: &str,
    timeout_secs: u64,
    stdout_cap: usize,
    stderr_cap: usize,
) -> Result<ChildOutput, std::io::Error> {
    let started = Instant::now();
    let mut child = Command::new(tsh)
        .args(["ssh", "--login", login, host, command])
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let stdout_handle = child
        .stdout
        .take()
        .map(|r| thread::spawn(move || drain_capped(r, stdout_cap)));
    let stderr_handle = child
        .stderr
        .take()
        .map(|r| thread::spawn(move || drain_capped(r, stderr_cap)));

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

    let (stdout, stdout_truncated) = stdout_handle
        .and_then(|h| h.join().ok())
        .unwrap_or((Vec::new(), false));
    let (stderr, stderr_truncated) = stderr_handle
        .and_then(|h| h.join().ok())
        .unwrap_or((Vec::new(), false));

    Ok(ChildOutput {
        timed_out,
        exit_code,
        stdout,
        stdout_truncated,
        stderr,
        stderr_truncated,
        duration_ms: started.elapsed().as_millis() as u64,
    })
}

/// Read a stream until EOF, storing at most `cap` bytes. Returns the stored
/// bytes and whether more data followed.
pub fn drain_capped<R: Read>(mut reader: R, cap: usize) -> (Vec<u8>, bool) {
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
