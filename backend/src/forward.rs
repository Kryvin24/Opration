//! Long-lived SSH local port forwards.
//!
//! Each forward is a `tsh ssh --login <login> -N -L <bind>:<lport>:<host>:<rport> <node>`
//! child process. `-N` means no remote command; `-L` makes tsh listen locally
//! and tunnel every connection to the remote target (typically a service bound
//! to 127.0.0.1 on the ship, or another address reachable from the node).
//!
//! Lifecycle events are emitted as `forward/state`:
//! `{ "id", "node", "state": "running|stopped|error", "reason", "local_url" }`.
//! A monitor thread owns the child and detects both natural exits (satellite
//! link drops) and user-initiated kills.

use std::io::Read;
use std::process::{Child, Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use dbx_plugin_sdk::{trace, PluginEmitter};
use serde_json::{json, Value};

struct EntryStatus {
    state: String, // "running" | "stopped" | "error"
    reason: String,
}

struct Entry {
    id: String,
    node: String,
    bind: String,
    local_port: u64,
    target_host: String,
    target_port: u64,
    started_at: u128,
    status: Mutex<EntryStatus>,
    child: Mutex<Option<Child>>,
    stderr: Arc<Mutex<Vec<u8>>>,
}

#[derive(Default)]
pub struct ForwardManager {
    entries: Mutex<std::collections::HashMap<String, Arc<Entry>>>,
}

impl ForwardManager {
    pub fn new() -> Self {
        Self::default()
    }

    /// Start a forward. Returns an error if the child dies within the first two
    /// seconds (local port already in use, node access denied, …).
    pub fn start(
        &self,
        tsh: &str,
        login: &str,
        node: &str,
        bind: &str,
        local_port: u64,
        target_host: &str,
        target_port: u64,
        emitter: PluginEmitter,
        manager: Arc<ForwardManager>,
    ) -> Result<Value, String> {
        let bind = if bind.trim().is_empty() {
            "127.0.0.1"
        } else {
            bind.trim()
        };
        let spec = format!("{bind}:{local_port}:{target_host}:{target_port}");
        let id = format!(
            "f{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_nanos())
                .unwrap_or_default()
        );

        let mut child = Command::new(tsh)
            .args(["ssh", "--login", login, "-N", "-L", &spec, node])
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| format!("failed to spawn tsh: {e}"))?;

        // Drain tsh stderr for the whole lifetime of the forward so the pipe
        // never blocks; the buffer is read when the child exits.
        let stderr_buf: Arc<Mutex<Vec<u8>>> = Arc::new(Mutex::new(Vec::new()));
        if let Some(mut err_reader) = child.stderr.take() {
            let buf = stderr_buf.clone();
            thread::spawn(move || {
                let mut chunk = [0u8; 2048];
                loop {
                    match err_reader.read(&mut chunk) {
                        Ok(0) | Err(_) => break,
                        Ok(n) => {
                            if let Ok(mut b) = buf.lock() {
                                if b.len() < 16_384 {
                                    let take = n.min(16_384 - b.len());
                                    b.extend_from_slice(&chunk[..take]);
                                }
                            }
                        }
                    }
                }
            });
        }

        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_millis())
            .unwrap_or_default();
        let entry = Arc::new(Entry {
            id: id.clone(),
            node: node.to_string(),
            bind: bind.to_string(),
            local_port,
            target_host: target_host.to_string(),
            target_port,
            started_at: now,
            status: Mutex::new(EntryStatus {
                state: "running".to_string(),
                reason: String::new(),
            }),
            child: Mutex::new(Some(child)),
            stderr: stderr_buf,
        });
        self.entries
            .lock()
            .unwrap()
            .insert(id.clone(), entry.clone());

        trace(&format!(
            "forward[{id}] start node={node} {bind}:{local_port} -> {target_host}:{target_port}"
        ));

        // Monitor thread: watch the child until it exits for any reason.
        let weak_entry: Arc<Entry> = entry.clone();
        thread::spawn(move || loop {
            let exited = {
                let mut child_g = match weak_entry.child.lock() {
                    Ok(g) => g,
                    Err(_) => break,
                };
                match child_g.as_mut().and_then(|c| c.try_wait().ok()) {
                    Some(Some(status)) => {
                        *child_g = None;
                        Some(status)
                    }
                    _ => None,
                }
            };
            if let Some(status) = exited {
                let code = status.code().unwrap_or(-1);
                // tsh's stderr output (may explain why the tunnel dropped).
                let err_text = weak_entry
                    .stderr
                    .lock()
                    .map(|b| String::from_utf8_lossy(&b).trim().to_string())
                    .unwrap_or_default();
                let mut state_g = weak_entry.status.lock().unwrap();
                if state_g.state == "running" {
                    state_g.state = if code == 0 {
                        "stopped".to_string()
                    } else {
                        "error".to_string()
                    };
                    state_g.reason = if err_text.is_empty() {
                        format!("tsh exited with code {code}")
                    } else {
                        err_text
                    };
                }
                let ev = json!({
                    "id": weak_entry.id,
                    "node": weak_entry.node,
                    "state": state_g.state,
                    "reason": state_g.reason,
                    "local_url": format!("http://{}:{}", weak_entry.bind, weak_entry.local_port),
                });
                drop(state_g);
                let _ = emitter.event("forward/state", ev);
                manager.entries.lock().unwrap().remove(&weak_entry.id);
                trace(&format!("forward[{}] ended", weak_entry.id));
                break;
            }
            thread::sleep(Duration::from_millis(300));
        });

        // Give the child two seconds to fail loudly (port in use, bad node).
        thread::sleep(Duration::from_millis(2000));
        // Read the state and DROP the guard here: metadata() locks `status`
        // again, and std::sync::Mutex is NOT reentrant — keeping the guard
        // across the metadata() call deadlocked the handler forever (the
        // forwardStart 30s timeout reported over satellite nodes).
        let state = entry.status.lock().unwrap().state.clone();
        if state == "running" {
            Ok(metadata(&entry))
        } else {
            Err(entry.status.lock().unwrap().reason.clone())
        }
    }

    /// Kill a forward's child; the monitor thread emits the state event.
    pub fn stop(&self, id: &str) -> bool {
        let entry = match self.entries.lock().unwrap().get(id).cloned() {
            Some(e) => e,
            None => return false,
        };
        let mut child_g = entry.child.lock().unwrap();
        if let Some(child) = child_g.as_mut() {
            let _ = child.kill();
            let _ = child.wait();
        }
        *child_g = None;
        let mut state_g = entry.status.lock().unwrap();
        state_g.state = "stopped".to_string();
        state_g.reason = "stopped by user".to_string();
        true
    }

    pub fn list(&self) -> Vec<Value> {
        self.entries
            .lock()
            .unwrap()
            .values()
            .map(|e| metadata(e))
            .collect()
    }
}

fn metadata(e: &Entry) -> Value {
    let status = e.status.lock().unwrap();
    json!({
        "id": e.id,
        "node": e.node,
        "bind": e.bind,
        "local_port": e.local_port,
        "target_host": e.target_host,
        "target_port": e.target_port,
        "started_at": e.started_at,
        "state": status.state,
        "local_url": format!("http://{}:{}", e.bind, e.local_port),
    })
}
