//! One-click diagnostics bundle.
//!
//! Gathers everything useful when reporting a plugin problem and writes it to
//! a single text file under %TEMP%:
//!   - tsh version + tsh status output
//!   - plugin version, connection id, timestamp
//!   - the tail of the sidecar log (passwords/OTPs already masked by the SDK)
//!
//! The UI shows the resulting path so it can be attached to an issue.

use std::io::{Read, Seek};
use std::process::{Command, Stdio};
use std::time::Duration;

use serde_json::Value;

/// Run `tsh <args>` with a hard timeout; returns combined stdout+stderr.
fn run_tsh(tsh: &str, args: &[&str], timeout_secs: u64) -> String {
    let (tx, rx) = std::sync::mpsc::channel();
    let prog = tsh.to_string();
    let argv: Vec<String> = args.iter().map(|s| s.to_string()).collect();
    std::thread::spawn(move || {
        let out = Command::new(prog)
            .args(&argv)
            .stdin(Stdio::null())
            .output();
        let _ = tx.send(out);
    });
    match rx.recv_timeout(Duration::from_secs(timeout_secs)) {
        Ok(Ok(out)) => {
            let mut s = String::from_utf8_lossy(&out.stdout).to_string();
            let err = String::from_utf8_lossy(&out.stderr).to_string();
            if !err.trim().is_empty() {
                s.push_str("\n[stderr]\n");
                s.push_str(&err);
            }
            s
        }
        Ok(Err(e)) => format!("failed to run tsh: {e}"),
        Err(_) => format!("tsh timed out after {timeout_secs}s"),
    }
}

/// Read the last `max_bytes` of the sidecar log, if present.
fn log_tail(max_bytes: usize) -> String {
    let Some(dir) = std::env::var_os("TEMP").or_else(|| std::env::var_os("TMP")) else {
        return String::from("(no TEMP dir)");
    };
    let path = std::path::Path::new(&dir).join("dbx-teleport-sidecar.log");
    let Ok(mut f) = std::fs::File::open(&path) else {
        return String::from("(no sidecar log found)");
    };
    let len = f.metadata().map(|m| m.len() as usize).unwrap_or(0);
    if len > max_bytes {
        let _ = f.seek(std::io::SeekFrom::End(-(max_bytes as i64)));
    }
    let mut buf = Vec::new();
    let _ = f.take((max_bytes + 64) as u64).read_to_end(&mut buf);
    String::from_utf8_lossy(&buf).to_string()
}

pub fn run(tsh: &str, conn_id: &str, plugin_version: &str) -> Result<Value, String> {
    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or_default();

    let mut doc = String::new();
    doc.push_str("DBX Teleport plugin diagnostics\n");
    doc.push_str(&format!("generated_unix={ts}\n"));
    doc.push_str(&format!("plugin_version={plugin_version}\n"));
    doc.push_str(&format!("connection_id={conn_id}\n"));
    doc.push_str(&format!("tsh_path={tsh}\n"));

    doc.push_str("\n===== tsh version =====\n");
    doc.push_str(&run_tsh(tsh, &["version"], 20));

    doc.push_str("\n===== tsh status =====\n");
    doc.push_str(&run_tsh(tsh, &["status"], 20));

    doc.push_str("\n===== sidecar log (tail) =====\n");
    doc.push_str(&log_tail(64 * 1024));

    let dir = std::env::var_os("TEMP")
        .or_else(|| std::env::var_os("TMP"))
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| std::path::PathBuf::from("."));
    let path = dir.join(format!("dbx-teleport-diag-{ts}.txt"));
    std::fs::write(&path, doc).map_err(|e| format!("failed to write diag file: {e}"))?;

    Ok(serde_json::json!({ "path": path.to_string_lossy() }))
}
