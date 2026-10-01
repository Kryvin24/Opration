//! Fleet inspection: one combined metrics command per node, parsed into
//! structured values the UI renders as a health heatmap wall.
//!
//! A single `tsh ssh` round trip runs INSPECT_CMD, which prints tagged
//! sections (==DF==, ==MEM==, ...) so parsing never depends on locale or
//! column order of the original tools. Each finished node is streamed to the
//! UI as an `inspect/item` event; the blocking call returns a summary.
//!
//! Item payload shape:
//! ```json
//! { "inspect_id": "...", "host": "...", "status": "ok|fail|timeout|error",
//!   "disk_pct": 90, "disk_avail": "10G", "mem_pct": 61, "load1": 0.8,
//!   "containers": 8, "duration_ms": 1234, "stderr": "" }
//! ```
//! `containers` is null when docker is not installed on the node.

use std::io::Read;
use std::process::{Command, Stdio};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use dbx_plugin_sdk::{trace, PluginEmitter};
use serde_json::{json, Value};

const STDERR_CAP: usize = 16_384;

/// Tagged-section command. `free -m` and `df -h` output is localized on some
/// distros, so we only rely on token positions, never on header names.
pub const INSPECT_CMD: &str = "echo ==DF==; df -h / | tail -1; echo ==MEM==; free -m; echo ==LOAD==; cat /proc/loadavg; echo ==DKR==; if command -v docker >/dev/null 2>&1; then docker ps -q 2>/dev/null | wc -l; else echo na; fi";

pub struct InspectOptions {
    pub tsh: String,
    pub login: String,
    pub hosts: Vec<String>,
    pub concurrency: usize,
    pub timeout_secs: u64,
    pub inspect_id: String,
}

/// Run the metrics command on every host. Returns a summary Value.
pub fn run(opts: InspectOptions, emitter: PluginEmitter) -> Value {
    let total = opts.hosts.len();
    let started = Instant::now();
    trace(&format!(
        "inspect[{}] start hosts={} concurrency={} timeout={}s",
        opts.inspect_id, total, opts.concurrency, opts.timeout_secs
    ));

    let (task_tx, task_rx) = mpsc::channel::<String>();
    let (res_tx, res_rx) = mpsc::channel::<Value>();
    let task_rx = Arc::new(Mutex::new(task_rx));

    let workers = opts.concurrency.clamp(1, 30).min(total.max(1));

    let mut handles = Vec::new();
    for _ in 0..workers {
        let rx = task_rx.clone();
        let tx = res_tx.clone();
        let em = emitter.clone();
        let tsh = opts.tsh.clone();
        let login = opts.login.clone();
        let inspect_id = opts.inspect_id.clone();
        let timeout_secs = opts.timeout_secs;
        handles.push(thread::spawn(move || loop {
            let host = match rx.lock() {
                Ok(guard) => match guard.recv() {
                    Ok(h) => h,
                    Err(_) => break,
                },
                Err(_) => break,
            };
            let item = run_one(&tsh, &login, &host, timeout_secs, &inspect_id);
            let _ = em.event("inspect/item", item.clone());
            let _ = tx.send(item);
        }));
    }
    drop(res_tx);
    for host in opts.hosts {
        let _ = task_tx.send(host);
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
    trace(&format!(
        "inspect[{}] done total={} ok={} failed={} in {}ms",
        opts.inspect_id,
        total,
        ok_count,
        total - ok_count,
        started.elapsed().as_millis()
    ));
    json!({
        "inspect_id": opts.inspect_id,
        "total": total,
        "ok": ok_count,
        "failed": total - ok_count,
        "duration_ms": started.elapsed().as_millis() as u64,
        "results": results,
    })
}

fn run_one(tsh: &str, login: &str, host: &str, timeout_secs: u64, inspect_id: &str) -> Value {
    let started = Instant::now();
    let spawn_result = Command::new(tsh)
        .args(["ssh", "--login", login, host, INSPECT_CMD])
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn();

    let mut child = match spawn_result {
        Ok(c) => c,
        Err(e) => {
            return json!({
                "inspect_id": inspect_id, "host": host, "status": "error",
                "duration_ms": started.elapsed().as_millis() as u64,
                "stderr": format!("spawn failed: {e}"),
            });
        }
    };

    let stdout_handle = child.stdout.take().map(|r| {
        thread::spawn(move || drain_capped(r, 65_536))
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

    let stdout_bytes = stdout_handle
        .and_then(|h| h.join().ok())
        .map(|(b, _)| b)
        .unwrap_or_default();
    let stderr_bytes = stderr_handle
        .and_then(|h| h.join().ok())
        .map(|(b, _)| b)
        .unwrap_or_default();
    let stdout = String::from_utf8_lossy(&stdout_bytes).into_owned();
    let stderr = String::from_utf8_lossy(&stderr_bytes).trim().to_string();

    let status = if timed_out {
        "timeout"
    } else if exit_code == Some(0) {
        "ok"
    } else {
        "fail"
    };

    let mut item = json!({
        "inspect_id": inspect_id,
        "host": host,
        "status": status,
        "duration_ms": started.elapsed().as_millis() as u64,
        "stderr": if status == "ok" { String::new() } else { stderr },
    });
    if status == "ok" {
        if let Value::Object(ref mut map) = parse_metrics(&stdout) {
            if let Value::Object(ref mut item_map) = item {
                item_map.append(map);
            }
        }
    }
    item
}

/// Parse the tagged sections into structured metrics.
pub fn parse_metrics(stdout: &str) -> Value {
    let section = |tag: &str| -> Option<&str> {
        let marker = format!("=={tag}==");
        let start = stdout.find(&marker)? + marker.len();
        let rest = &stdout[start..];
        let end = rest.find("==").unwrap_or(rest.len());
        Some(rest[..end].trim())
    };

    // df -h / | tail -1  →  "/dev/sda1  100G  90G  10G  90% /"
    let mut disk_pct = Value::Null;
    let mut disk_avail = Value::Null;
    if let Some(df) = section("DF") {
        let toks: Vec<&str> = df.split_whitespace().collect();
        if let Some(pos) = toks.iter().position(|t| t.ends_with('%')) {
            if let Ok(p) = toks[pos].trim_end_matches('%').parse::<i64>() {
                disk_pct = json!(p);
            }
            if pos > 0 {
                disk_avail = json!(toks[pos - 1]);
            }
        }
    }

    // free -m → "Mem:  total used free shared buff/cache available"
    let mut mem_pct = Value::Null;
    if let Some(mem) = section("MEM") {
        for line in mem.lines() {
            let l = line.trim_start();
            if l.starts_with("Mem:") {
                let toks: Vec<&str> = l.split_whitespace().collect();
                if toks.len() >= 3 {
                    if let (Ok(total), Ok(used)) = (
                        toks[1].parse::<f64>(),
                        toks[2].parse::<f64>(),
                    ) {
                        if total > 0.0 {
                            mem_pct = json!(((used / total) * 100.0).round() as i64);
                        }
                    }
                }
                break;
            }
        }
    }

    // /proc/loadavg → "0.08 0.03 0.01 1/234 12345"
    let mut load1 = Value::Null;
    if let Some(load) = section("LOAD") {
        if let Some(first) = load.split_whitespace().next() {
            if let Ok(v) = first.parse::<f64>() {
                load1 = json!(v);
            }
        }
    }

    // docker count, or null when docker is absent ("na")
    let mut containers = Value::Null;
    if let Some(dkr) = section("DKR") {
        let v = dkr.trim();
        if v != "na" {
            if let Ok(n) = v.parse::<i64>() {
                containers = json!(n);
            }
        }
    }

    json!({
        "disk_pct": disk_pct,
        "disk_avail": disk_avail,
        "mem_pct": mem_pct,
        "load1": load1,
        "containers": containers,
    })
}

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

#[cfg(test)]
mod tests {
    use super::parse_metrics;

    #[test]
    fn parses_typical_output() {
        let out = "==DF==\n/dev/sda1  100G  90G  10G  90% /\n==MEM==\n              total        used        free      shared  buff/cache   available\nMem:           7973        4861         512         112        2599        2789\nSwap:             0           0           0\n==LOAD==\n0.08 0.03 0.01 1/234 12345\n==DKR==\n8\n";
        let m = parse_metrics(out);
        assert_eq!(m["disk_pct"], 90);
        assert_eq!(m["disk_avail"], "10G");
        assert_eq!(m["mem_pct"], 61);
        assert_eq!(m["load1"], 0.08);
        assert_eq!(m["containers"], 8);
    }

    #[test]
    fn handles_missing_docker() {
        let out = "==DF==\noverlay  50G  20G  30G  40% /\n==MEM==\nMem:           4096        1024        2048          10        1024        2900\n==LOAD==\n1.50 0.90 0.40 2/100 999\n==DKR==\nna\n";
        let m = parse_metrics(out);
        assert_eq!(m["disk_pct"], 40);
        assert_eq!(m["mem_pct"], 25);
        assert_eq!(m["load1"], 1.5);
        assert!(m["containers"].is_null());
    }
}
