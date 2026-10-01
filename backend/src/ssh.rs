//! SSH terminal sessions over ConPTY, streaming to the DBX workbench UI over
//! the framed binary protocol used by the official io.dbx.ssh plugin.
//!
//! Frame layout (per out-channel byte stream, confirmed against io.dbx.ssh):
//!   byte[0]      stream flag (0 = pty data, 2 = session ended)
//!   byte[1..9]   sequence number, u64 big-endian
//!   byte[9..]    payload (pty output, or a short text reason on stream 2)
//!
//! The UI writes typed bytes via `sendBinary("ssh/terminal/in/<sid>", ...)`
//! and resizes via `notify("ssh/terminal/resize", {sessionId, cols, rows})`.
//!
//! ConPTY on Windows does not reliably deliver EOF on the master read, so the
//! pump loop is driven by a reader thread plus polling `child.try_wait()` —
//! the same pattern as login.rs.

use std::collections::HashMap;
use std::io::{Read, Write};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::mpsc::{self, RecvTimeoutError};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use dbx_plugin_sdk::{trace, trace_verbose, PluginEmitter};
use portable_pty::{native_pty_system, Child, CommandBuilder, MasterPty, PtySize};

const STREAM_DATA: u8 = 0;
const STREAM_END: u8 = 2;
const POLL_INTERVAL: Duration = Duration::from_millis(150);

/// Manages live SSH terminal sessions, one ConPTY per open terminal.
pub struct SshServer {
    sessions: Arc<Mutex<HashMap<String, Arc<Session>>>>,
    next_id: AtomicU64,
}

struct Session {
    writer: Mutex<Box<dyn Write + Send>>,
    master: Mutex<Box<dyn MasterPty + Send>>,
    child: Mutex<Box<dyn Child + Send>>,
    seq: AtomicU64,
    closed: AtomicBool,
}

impl SshServer {
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(Mutex::new(HashMap::new())),
            next_id: AtomicU64::new(1),
        }
    }

    /// Spawn `tsh ssh <hostname>` in a ConPTY and start streaming its output.
    /// Returns the session id used on the `ssh/terminal/in/<sid>` channel.
    pub fn open(&self, tsh: &str, hostname: &str, login: &str, emitter: PluginEmitter) -> Result<String, String> {
        trace(&format!("ssh: opening {hostname} via {}", tsh));

        let pty_system = native_pty_system();
        let pair = pty_system
            .openpty(PtySize {
                rows: 30,
                cols: 120,
                pixel_width: 0,
                pixel_height: 0,
            })
            .map_err(|e| format!("ConPTY unavailable: {e}"))?;

        let mut cmd = CommandBuilder::new(tsh);
        cmd.arg("ssh");
        // A login MUST be pinned explicitly. Without it tsh falls back to the
        // local OS user (e.g. "zdiai"), which the cluster rejects with
        // "access denied to <osuser>". The login list is exposed as
        // "Logins: root" at auth time, so root is the correct default here.
        cmd.arg("--login");
        cmd.arg(login);
        cmd.arg("--tty");
        // Keepalives detect a dead satellite link: with 30s interval and 3
        // missed probes, tsh exits ~90s after the link silently drops instead
        // of hanging on a half-open TCP forever. The pump then emits its end
        // frame and the UI can offer an immediate reconnect.
        cmd.arg("-o");
        cmd.arg("ServerAliveInterval=30");
        cmd.arg("-o");
        cmd.arg("ServerAliveCountMax=3");
        cmd.arg(hostname);

        let child = pair
            .slave
            .spawn_command(cmd)
            .map_err(|e| format!("failed to spawn {}: {e}", tsh))?;
        drop(pair.slave);

        let mut reader = pair
            .master
            .try_clone_reader()
            .map_err(|e| format!("PTY reader error: {e}"))?;
        let writer = pair
            .master
            .take_writer()
            .map_err(|e| format!("PTY writer error: {e}"))?;
        let master = pair.master;

        let id = format!("{}-{}", std::process::id(), self.next_id.fetch_add(1, Ordering::SeqCst));
        let channel = format!("ssh/terminal/out/{id}");
        let session = Arc::new(Session {
            writer: Mutex::new(writer),
            master: Mutex::new(master),
            child: Mutex::new(child),
            seq: AtomicU64::new(0),
            closed: AtomicBool::new(false),
        });
        self.sessions.lock().map_err(|_| "session map is poisoned".to_string())?.insert(id.clone(), session.clone());
        trace(&format!("ssh: session {id} registered"));

        // Reader thread: hand raw pty chunks (or EOF) to the pump.
        let (tx, rx) = mpsc::channel::<Option<Vec<u8>>>();
        thread::Builder::new()
            .name(format!("ssh-reader-{id}"))
            .spawn(move || {
                let mut buf = [0u8; 8192];
                loop {
                    match reader.read(&mut buf) {
                        Ok(0) | Err(_) => {
                            let _ = tx.send(None);
                            break;
                        }
                        Ok(n) => {
                            if tx.send(Some(buf[..n].to_vec())).is_err() {
                                break;
                            }
                        }
                    }
                }
            })
            .map_err(|e| format!("failed to spawn reader thread: {e}"))?;

        // Pump: frame each chunk as [stream][seq BE][data]; when the child
        // exits (or EOF arrives), emit a stream=2 end frame and clean up.
        let sessions = self.sessions.clone();
        let pump_id = id.clone();
        thread::Builder::new()
            .name(format!("ssh-pump-{pump_id}"))
            .spawn(move || {
                let mut exited = false;
                // Diagnose why a session may close early: capture the first pty
                // bytes so the log shows exactly what tsh ssh printed/errored.
                let mut diag: Vec<u8> = Vec::with_capacity(1024);
                loop {
                    match rx.recv_timeout(POLL_INTERVAL) {
                        Ok(Some(bytes)) => {
                            if diag.len() < 1024 {
                                let n = 1024 - diag.len();
                                diag.extend_from_slice(&bytes[..bytes.len().min(n)]);
                            }
                            if diag.len() >= 950 {
                                trace_verbose(&format!(
                                    "ssh: pty diag ({} bytes): {:?}",
                                    diag.len(),
                                    String::from_utf8_lossy(&diag)
                                ));
                                diag.clear();
                            }
                            let frame = make_frame(
                                session.seq.fetch_add(1, Ordering::SeqCst),
                                STREAM_DATA,
                                &bytes,
                            );
                            if emitter.binary(&channel, &frame).is_err() {
                                break;
                            }
                        }
                        Ok(None) => {
                            // Reader hit EOF.
                            exited = true;
                            break;
                        }
                        Err(RecvTimeoutError::Timeout) => {
                            // No new output; check whether tsh ssh already exited.
                            match session.child.lock() {
                                Ok(mut child) => match child.try_wait() {
                                    Ok(Some(_)) => {
                                        exited = true;
                                        break;
                                    }
                                    Ok(None) => {}
                                    Err(_) => break,
                                },
                                Err(_) => break,
                            }
                        }
                        Err(RecvTimeoutError::Disconnected) => break,
                    }
                }
                if !diag.is_empty() {
                    trace_verbose(&format!(
                        "ssh: pty diag final ({} bytes): {:?}",
                        diag.len(),
                        String::from_utf8_lossy(&diag)
                    ));
                }
                if !exited {
                    // Make sure the child is gone before declaring the session over.
                    if let Ok(mut child) = session.child.lock() {
                        let _ = child.kill();
                    }
                }
                let end = make_frame(
                    session.seq.fetch_add(1, Ordering::SeqCst),
                    STREAM_END,
                    b"ssh-transport-disconnected",
                );
                let _ = emitter.binary(&channel, &end);
                session.closed.store(true, Ordering::SeqCst);
                if let Ok(mut map) = sessions.lock() {
                    map.remove(&pump_id);
                }
                trace(&format!("ssh: session {pump_id} ended"));
            })
            .map_err(|e| format!("failed to spawn pump thread: {e}"))?;

        Ok(id)
    }

    /// Forward a chunk of typed bytes to the session's pty.
    pub fn write(&self, session_id: &str, data: &[u8]) -> Result<(), String> {
        let session = self
            .sessions
            .lock()
            .map_err(|_| "session map is poisoned".to_string())?
            .get(session_id)
            .cloned()
            .ok_or_else(|| format!("no such session: {session_id}"))?;
        if session.closed.load(Ordering::SeqCst) {
            return Err(format!("session {session_id} is closed"));
        }
        let mut writer = session.writer.lock().map_err(|_| "session writer is poisoned".to_string())?;
        writer
            .write_all(data)
            .map_err(|e| format!("failed to write to session {session_id}: {e}"))?;
        writer
            .flush()
            .map_err(|e| format!("failed to flush session {session_id}: {e}"))
    }

    /// Tell the pty that the UI terminal was resized.
    pub fn resize(&self, session_id: &str, cols: u16, rows: u16) -> Result<(), String> {
        let session = self
            .sessions
            .lock()
            .map_err(|_| "session map is poisoned".to_string())?
            .get(session_id)
            .cloned()
            .ok_or_else(|| format!("no such session: {session_id}"))?;
        let master = session.master.lock().map_err(|_| "session master is poisoned".to_string())?;
        master
            .resize(PtySize {
                rows,
                cols,
                pixel_width: 0,
                pixel_height: 0,
            })
            .map_err(|e| format!("failed to resize session {session_id}: {e}"))
    }

    /// Kill the child; the pump thread notices and emits the end frame.
    pub fn close(&self, session_id: &str) {
        trace(&format!("ssh: closing session {session_id}"));
        if let Ok(map) = self.sessions.lock() {
            if let Some(session) = map.get(session_id) {
                session.closed.store(true, Ordering::SeqCst);
                if let Ok(mut child) = session.child.lock() {
                    let _ = child.kill();
                }
            }
        }
    }

    /// Number of live sessions (diagnostics only).
    pub fn count(&self) -> usize {
        self.sessions.lock().map(|m| m.len()).unwrap_or(0)
    }
}

fn make_frame(seq: u64, stream: u8, data: &[u8]) -> Vec<u8> {
    let mut frame = Vec::with_capacity(9 + data.len());
    frame.push(stream);
    frame.extend_from_slice(&seq.to_be_bytes());
    frame.extend_from_slice(data);
    frame
}
