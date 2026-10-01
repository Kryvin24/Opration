// DBX Teleport Plugin — native sidecar
// Built on the vendored dbx-plugin-sdk (upstream, with built-in HostClient).
//
// Phase 1–2:
//   · connection/test          — locate tsh.exe and report its version
//   · connection/connect       — `tsh login` in ConPTY; password from the form,
//                                OTP collected fresh via host/requestUserInput
//   · connection/disconnect    — `tsh logout`
//   · connection/action        — login / logout / refresh metadata
//   · teleport/listResources   — nodes, apps, and kube clusters (Phase 2)

mod batch;
mod diag;
mod forward;
mod inspect;
mod login;
mod pool;
mod resources;
mod ssh;
mod transfer;

use std::process::Command;
use std::sync::Arc;
use std::time::Duration;
use std::thread;

use serde_json::{json, Value};

use dbx_plugin_sdk::{
    host_client, trace, PluginEmitter, PluginError, PluginHandler, PluginMetadata, PluginServer,
    PluginTransport, RequestContext, HOST_REQUEST_USER_INPUT_METHOD,
};

use login::LoginParams;

/// Settings remembered per DBX connection at login time, so the context-menu
/// SSH handler can resolve both the tsh binary and the remote login when DBX
/// passes only `{connectionId, name}` rather than the full connection object.
#[derive(Clone)]
struct ConnSetting {
    tsh: String,
    ssh_login: String,
}

static CONN_SETTINGS: std::sync::Mutex<Option<std::collections::HashMap<String, ConnSetting>>> =
    std::sync::Mutex::new(None);

fn remember_conn(connection_id: &str, setting: ConnSetting) {
    if connection_id.is_empty() {
        return;
    }
    if let Ok(mut slot) = CONN_SETTINGS.lock() {
        slot.get_or_insert_with(std::collections::HashMap::new)
            .insert(connection_id.to_string(), setting);
    }
}

fn conn_setting(connection_id: &str) -> Option<ConnSetting> {
    CONN_SETTINGS
        .lock()
        .ok()
        .and_then(|g| g.as_ref().and_then(|m| m.get(connection_id).cloned()))
}

struct TeleportPlugin {
    ssh: ssh::SshServer,
    forward: Arc<forward::ForwardManager>,
}

impl PluginHandler for TeleportPlugin {
    fn handle(
        &self,
        _context: RequestContext,
        method: &str,
        params: Value,
        emitter: &PluginEmitter,
    ) -> Result<Value, PluginError> {
        trace(&format!("rpc: {method}"));
        match method {
            "connection/test" => Ok(handle_test(&params)),

            "connection/connect" => self.handle_login(&params),

            "connection/disconnect" => Ok(handle_logout(&params)),

            "connection/action" => {
                let action = params
                    .pointer("/action/id")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                match action {
                    "login" => self.handle_login(&params),
                    "logout" => Ok(handle_logout(&params)),
                    "refresh" => Ok(handle_refresh(&params)),
                    other => {
                        Err(PluginError::new(-32602, format!("Unknown action: {other}")))
                    }
                }
            }

            "teleport/listResources" => Ok(handle_list_resources(&params)),

            "teleport/batchExec" => self.handle_batch_exec(&params, emitter),

            "teleport/probe" => self.handle_probe(&params, emitter),

            // Port forwarding + kube context injection.
            "teleport/forwardStart" => self.handle_forward_start(&params, emitter),
            "teleport/forwardStop" => self.handle_forward_stop(&params),
            "teleport/forwardList" => Ok(self.handle_forward_list()),
            "teleport/kubeLogin" => self.handle_kube_login(&params),

            // File transfer.
            "teleport/transferUpload" => self.handle_transfer_upload(&params, emitter),
            "teleport/transferDownload" => self.handle_transfer_download(&params),

            // Fleet inspection (metrics heatmap).
            "teleport/inspect" => self.handle_inspect(&params, emitter),

            // Diagnostics bundle (tsh version/status + sidecar log tail).
            "teleport/diagBundle" => self.handle_diag_bundle(&params),

            // Terminal lifecycle (fire-and-forget notifications from the UI).
            "ssh/terminal/resize" => {
                let sid = params.get("sessionId").and_then(|v| v.as_str()).unwrap_or("");
                let cols = params.get("cols").and_then(|v| v.as_u64()).unwrap_or(120);
                let rows = params.get("rows").and_then(|v| v.as_u64()).unwrap_or(30);
                self.ssh
                    .resize(sid, cols as u16, rows as u16)
                    .map_err(|e| PluginError::new(-32000, e))?;
                Ok(json!({}))
            }
            "ssh/terminal/close" => {
                if let Some(sid) = params.get("sessionId").and_then(|v| v.as_str()) {
                    self.ssh.close(sid);
                }
                Ok(json!({}))
            }

            // Context-menu contributions: only SSH is live. Unimplemented
            // methods fall through to method_not_found.
            "contextMenu/io.zdiai.teleport.ssh" => self.handle_ssh(&params, emitter),

            _ => Err(PluginError::method_not_found(method)),
        }
    }

    fn handle_binary(&self, channel: &str, data: Vec<u8>, _emitter: &PluginEmitter) -> Result<(), PluginError> {
        if let Some(sid) = channel.strip_prefix("ssh/terminal/in/") {
            trace(&format!("ssh: binary input sid={sid} len={}", data.len()));
            self.ssh
                .write(sid, &data)
                .map_err(|e| PluginError::new(-32000, e))?;
            return Ok(());
        }
        Err(PluginError::new(-32601, format!("Unsupported binary channel: {channel}")))
    }
}

impl TeleportPlugin {
    fn handle_login(&self, params: &Value) -> Result<Value, PluginError> {
        if let Some(client) = host_client() {
            trace(&format!(
                "login: host api_version={:?} supports_requestUserInput={}",
                client.host_api_version(),
                client.supports(HOST_REQUEST_USER_INPUT_METHOD)
            ));
        } else {
            trace("login: no host client installed");
        }
        let login_params = build_login_params(params)?;
        let tsh = login_params.tsh.clone();
        let conn_id = params
            .pointer("/connection/id")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        // Recorded on every connect path (including the valid-session skip),
        // so a restored workbench tab can still resolve tsh + ssh_login.
        remember_conn(
            conn_id,
            ConnSetting {
                tsh: tsh.clone(),
                ssh_login: login_params.ssh_login.clone(),
            },
        );

        // Idempotent login: if a valid cert for this proxy+user already exists,
        // don't force a new authentication (each TOTP code is single-use, so a
        // second forced login within the same 30s window would reject the same
        // code the user just typed).
        if has_valid_session(&tsh, &login_params.proxy, login_params.port, &login_params.user) {
            trace("login: valid session already exists, skipping re-auth");
            return Ok(build_connect_result(&tsh));
        }

        login::run_login(&login_params).map_err(|message| PluginError::new(-32000, message))?;
        Ok(build_connect_result(&tsh))
    }

    /// Open an SSH terminal for a node. `params` carries `name` (the node
    /// hostname from the resource list) and optionally a full `connection`
    /// object; the tsh binary falls back to the one used at login time.
    fn handle_ssh(&self, params: &Value, emitter: &PluginEmitter) -> Result<Value, PluginError> {
        let hostname = params
            .get("name")
            .and_then(|v| v.as_str())
            .filter(|s| !s.is_empty())
            .ok_or_else(|| PluginError::new(-32602, "node name is required"))?;

        let explicit_conn = params.get("connection").filter(|v| v.is_object());
        let conn_id = params
            .get("connectionId")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let remembered = conn_setting(conn_id);

        // tsh resolution: explicit form value → what this connection logged in
        // with → PATH default.
        let tsh = explicit_conn
            .and_then(|conn| cfg_str(conn, "tsh_path"))
            .map(|p| tsh_bin(&Some(p)))
            .or_else(|| remembered.as_ref().map(|s| s.tsh.clone()))
            .unwrap_or_else(|| tsh_bin(&None));

        // Remote login: explicit form value → remembered ssh_login → root.
        // Pinning it is mandatory — without --login tsh falls back to the local
        // OS user and the cluster rejects the session with access denied.
        let login = explicit_conn
            .and_then(|conn| cfg_str(conn, "ssh_login"))
            .filter(|s| !s.is_empty())
            .or_else(|| remembered.as_ref().map(|s| s.ssh_login.clone()))
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| "root".to_string());

        trace(&format!("ssh: open terminal for {hostname} (login={login})"));
        let session_id = self
            .ssh
            .open(&tsh, hostname, &login, emitter.clone())
            .map_err(|e| PluginError::new(-32000, e))?;
        trace(&format!("ssh: session {session_id} started (live={})", self.ssh.count()));
        Ok(json!({ "sessionId": session_id, "ok": true }))
    }

    /// Run one shell command on many nodes concurrently. Blocking: returns the
    /// full summary once every node finished; per-node rows also stream out as
    /// `batch/item` events so the UI can render progress live.
    fn handle_batch_exec(
        &self,
        params: &Value,
        emitter: &PluginEmitter,
    ) -> Result<Value, PluginError> {
        let conn_id = params
            .get("connectionId")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let command = params
            .get("command")
            .and_then(|v| v.as_str())
            .filter(|s| !s.is_empty())
            .ok_or_else(|| PluginError::new(-32602, "command is required"))?;
        let hosts: Vec<String> = params
            .get("hosts")
            .and_then(|v| v.as_array())
            .map(|a| {
                a.iter()
                    .filter_map(|h| h.as_str().map(|s| s.to_string()))
                    .filter(|s| !s.is_empty())
                    .collect()
            })
            .unwrap_or_default();
        if hosts.is_empty() {
            return Err(PluginError::new(-32602, "hosts must not be empty"));
        }
        let setting = conn_setting(conn_id).ok_or_else(|| {
            PluginError::new(-32000, "connection settings unavailable, please login first")
        })?;
        let concurrency = params
            .get("concurrency")
            .and_then(|v| v.as_u64())
            .unwrap_or(10) as usize;
        let timeout_secs = params
            .get("timeoutSecs")
            .and_then(|v| v.as_u64())
            .unwrap_or(60);
        let batch_id = gen_batch_id(params);

        let opts = batch::BatchOptions {
            tsh: setting.tsh,
            login: setting.ssh_login,
            hosts,
            command: command.to_string(),
            concurrency,
            timeout_secs,
            batch_id,
            kind: "exec".to_string(),
            phase: 0,
        };
        Ok(batch::execute(opts, emitter.clone()))
    }

    /// Liveness probe. Hosts default to every node `tsh ls` knows about; each
    /// node gets a short run of `echo`. The returned `online` object maps
    /// hostname → reachable.
    fn handle_probe(
        &self,
        params: &Value,
        emitter: &PluginEmitter,
    ) -> Result<Value, PluginError> {
        let conn_id = params
            .get("connectionId")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let setting = conn_setting(conn_id).ok_or_else(|| {
            PluginError::new(-32000, "connection settings unavailable, please login first")
        })?;

        let hosts: Vec<String> = match params.get("hosts").and_then(|v| v.as_array()) {
            Some(a) => a
                .iter()
                .filter_map(|h| h.as_str().map(|s| s.to_string()))
                .filter(|s| !s.is_empty())
                .collect(),
            None => {
                let (rows, _failures) = resources::list_all(&setting.tsh);
                rows.into_iter()
                    .filter(|r| r.get("kind").and_then(|v| v.as_str()) == Some("node"))
                    .filter_map(|r| r.get("name").and_then(|v| v.as_str()).map(|s| s.to_string()))
                    .collect()
            }
        };
        if hosts.is_empty() {
            return Err(PluginError::new(-32000, "no nodes to probe"));
        }

        let concurrency = params
            .get("concurrency")
            .and_then(|v| v.as_u64())
            .unwrap_or(30) as usize;
        let retry_concurrency = params
            .get("retryConcurrency")
            .and_then(|v| v.as_u64())
            .unwrap_or(5) as usize;
        // Satellite links add seconds of latency to both dial and response, so
        // the first pass must be patient and a failure is only *suspected*.
        let timeout_secs = params
            .get("timeoutSecs")
            .and_then(|v| v.as_u64())
            .unwrap_or(20);
        let retry_timeout_secs = params
            .get("retryTimeoutSecs")
            .and_then(|v| v.as_u64())
            .unwrap_or(45);
        let batch_id = gen_batch_id(params);

        // ---- Pass 1: broad sweep at high concurrency ----
        let phase1_opts = batch::BatchOptions {
            tsh: setting.tsh.clone(),
            login: setting.ssh_login.clone(),
            hosts: hosts.clone(),
            command: "echo __tp_probe_ok__".to_string(),
            concurrency,
            timeout_secs,
            batch_id: batch_id.clone(),
            kind: "probe".to_string(),
            phase: 1,
        };
        let phase1 = batch::execute(phase1_opts, emitter.clone());

        let p1_rows: Vec<Value> = phase1
            .get("results")
            .and_then(|r| r.as_array())
            .cloned()
            .unwrap_or_default();
        let ok_hosts: Vec<String> = p1_rows
            .iter()
            .filter(|r| r.get("status").and_then(|s| s.as_str()) == Some("ok"))
            .filter_map(|r| r.get("host").and_then(|h| h.as_str()).map(|s| s.to_string()))
            .collect();
        let suspects: Vec<String> = p1_rows
            .iter()
            .filter(|r| r.get("status").and_then(|s| s.as_str()) != Some("ok"))
            .filter_map(|r| r.get("host").and_then(|h| h.as_str()).map(|s| s.to_string()))
            .collect();

        trace(&format!(
            "probe[{batch_id}] pass1 ok={} suspects={}; confirming suspects with {}s timeout",
            ok_hosts.len(), suspects.len(), retry_timeout_secs
        ));

        // ---- Pass 2: re-confirm every suspected-offline node, low concurrency
        // and a long timeout so satellite jitter can never mislabel a node ----
        let mut p2_rows: Vec<Value> = Vec::new();
        if !suspects.is_empty() {
            let phase2_opts = batch::BatchOptions {
                tsh: setting.tsh.clone(),
                login: setting.ssh_login.clone(),
                hosts: suspects,
                command: "echo __tp_probe_ok__".to_string(),
                concurrency: retry_concurrency,
                timeout_secs: retry_timeout_secs,
                batch_id: batch_id.clone(),
                kind: "probe".to_string(),
                phase: 2,
            };
            let phase2 = batch::execute(phase2_opts, emitter.clone());
            p2_rows = phase2
                .get("results")
                .and_then(|r| r.as_array())
                .cloned()
                .unwrap_or_default();
        }

        // Merge: a node is reachable if EITHER pass succeeded. Remember the
        // best (shortest) successful duration for the slow-link classification.
        const SLOW_LINK_MS: u64 = 8_000;
        let mut best_ok: std::collections::HashMap<String, u64> =
            std::collections::HashMap::new();
        for row in p1_rows.iter().chain(p2_rows.iter()) {
            if row.get("status").and_then(|s| s.as_str()) == Some("ok") {
                let host = row.get("host").and_then(|h| h.as_str()).unwrap_or("");
                let dur = row.get("duration_ms").and_then(|v| v.as_u64()).unwrap_or(0);
                match best_ok.get(host) {
                    Some(prev) if *prev <= dur => {}
                    _ => {
                        best_ok.insert(host.to_string(), dur);
                    }
                }
            }
        }

        let mut online = serde_json::Map::new();
        let mut slow = serde_json::Map::new();
        for host in &hosts {
            match best_ok.get(host) {
                Some(dur) => {
                    online.insert(host.clone(), json!(true));
                    if *dur > SLOW_LINK_MS {
                        slow.insert(host.clone(), json!(*dur));
                    }
                }
                None => {
                    online.insert(host.clone(), json!(false));
                }
            }
        }

        let total = hosts.len();
        let ok_count = best_ok.len();
        let summary = json!({
            "batch_id": batch_id,
            "kind": "probe",
            "total": total,
            "ok": ok_count,
            "failed": total - ok_count,
            "suspects": phase1.get("failed").and_then(|v| v.as_u64()).unwrap_or(0),
            "slow": Value::Object(slow.clone()),
            "online": Value::Object(online),
            "phase1_timeout_secs": timeout_secs,
            "phase2_timeout_secs": retry_timeout_secs,
        });
        Ok(summary)
    }

    /// Start an SSH local port forward to a node.
    fn handle_forward_start(
        &self,
        params: &Value,
        emitter: &PluginEmitter,
    ) -> Result<Value, PluginError> {
        let conn_id = params
            .get("connectionId")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let setting = conn_setting(conn_id).ok_or_else(|| {
            PluginError::new(-32000, "connection settings unavailable, please login first")
        })?;
        let node = params
            .get("node")
            .and_then(|v| v.as_str())
            .filter(|s| !s.is_empty())
            .ok_or_else(|| PluginError::new(-32602, "node is required"))?;
        let local_port = params
            .get("localPort")
            .and_then(|v| v.as_u64())
            .ok_or_else(|| PluginError::new(-32602, "localPort is required"))?;
        let target_host = params
            .get("targetHost")
            .and_then(|v| v.as_str())
            .unwrap_or("127.0.0.1");
        let target_port = params
            .get("targetPort")
            .and_then(|v| v.as_u64())
            .ok_or_else(|| PluginError::new(-32602, "targetPort is required"))?;
        let bind = params
            .get("bind")
            .and_then(|v| v.as_str())
            .unwrap_or("127.0.0.1");

        self.forward
            .start(
                &setting.tsh,
                &setting.ssh_login,
                node,
                bind,
                local_port,
                target_host,
                target_port,
                emitter.clone(),
                self.forward.clone(),
            )
            .map_err(|e| PluginError::new(-32000, e))
    }

    fn handle_forward_stop(&self, params: &Value) -> Result<Value, PluginError> {
        let id = params
            .get("id")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        Ok(json!({ "ok": self.forward.stop(id) }))
    }

    fn handle_forward_list(&self) -> Value {
        json!({ "forwards": self.forward.list() })
    }

    /// Fan the metrics command out to every requested node (fleet heatmap).
    fn handle_inspect(
        &self,
        params: &Value,
        emitter: &PluginEmitter,
    ) -> Result<Value, PluginError> {
        let conn_id = params
            .get("connectionId")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let setting = conn_setting(conn_id).ok_or_else(|| {
            PluginError::new(-32000, "connection settings unavailable, please login first")
        })?;
        let hosts: Vec<String> = params
            .get("hosts")
            .and_then(|v| v.as_array())
            .map(|a| {
                a.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .filter(|s| !s.is_empty())
                    .collect()
            })
            .unwrap_or_default();
        if hosts.is_empty() {
            return Err(PluginError::new(-32602, "hosts must contain at least one node"));
        }
        let concurrency = params
            .get("concurrency")
            .and_then(|v| v.as_u64())
            .unwrap_or(20) as usize;
        // Satellite links: default 60s per node is generous but bounded.
        let timeout_secs = params
            .get("timeoutSecs")
            .and_then(|v| v.as_u64())
            .unwrap_or(60);

        let opts = inspect::InspectOptions {
            tsh: setting.tsh.clone(),
            login: setting.ssh_login.clone(),
            hosts,
            concurrency,
            timeout_secs,
            inspect_id: gen_batch_id(params),
        };
        Ok(inspect::run(opts, emitter.clone()))
    }

    /// Write a diagnostics bundle (tsh version/status + sidecar log tail) to
    /// %TEMP% and return its path for the UI to display.
    fn handle_diag_bundle(&self, params: &Value) -> Result<Value, PluginError> {
        let conn_id = params
            .get("connectionId")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let tsh = conn_setting(conn_id)
            .map(|s| s.tsh)
            .unwrap_or_else(|| "tsh.exe".to_string());
        diag::run(&tsh, conn_id, env!("CARGO_PKG_VERSION"))
            .map_err(|e| PluginError::new(-32000, e))
    }

    /// Inject a Kubernetes cluster's credentials into the local kubeconfig via
    /// `tsh kube login <cluster>`. Satellite links make this slow, so allow up
    /// to 90 seconds; the tsh output is returned for the UI to display.
    fn handle_kube_login(
        &self,
        params: &Value,
    ) -> Result<Value, PluginError> {
        let conn_id = params
            .get("connectionId")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let setting = conn_setting(conn_id).ok_or_else(|| {
            PluginError::new(-32000, "connection settings unavailable, please login first")
        })?;
        let cluster = params
            .get("cluster")
            .and_then(|v| v.as_str())
            .filter(|s| !s.is_empty())
            .ok_or_else(|| PluginError::new(-32602, "cluster is required"))?;

        let (tx, rx) = std::sync::mpsc::channel();
        let tsh = setting.tsh.clone();
        let kc = cluster.to_string();
        thread::spawn(move || {
            let out = Command::new(tsh)
                .args(["kube", "login", &kc])
                .output();
            let _ = tx.send(out);
        });
        let output = rx
            .recv_timeout(Duration::from_secs(90))
            .map_err(|_| PluginError::new(-32000, "kube login timed out after 90s"))?
            .map_err(|e| PluginError::new(-32000, format!("failed to run tsh: {e}")))?;

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        let ok = output.status.success();
        if !ok {
            return Err(PluginError::new(-32000, stderr.trim().to_string()));
        }
        // tsh writes the current-context entry into %USERPROFILE%\.kube\config.
        Ok(json!({
            "ok": true,
            "cluster": cluster,
            "context": cluster,
            "output": format!("{stdout}{stderr}").trim().to_string(),
            "kubeconfig": "%USERPROFILE%\\.kube\\config",
        }))
    }

    /// Upload one local path to many nodes concurrently.
    fn handle_transfer_upload(
        &self,
        params: &Value,
        emitter: &PluginEmitter,
    ) -> Result<Value, PluginError> {
        let conn_id = params
            .get("connectionId")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let setting = conn_setting(conn_id).ok_or_else(|| {
            PluginError::new(-32000, "connection settings unavailable, please login first")
        })?;
        let local_path = params
            .get("localPath")
            .and_then(|v| v.as_str())
            .filter(|s| !s.is_empty())
            .ok_or_else(|| PluginError::new(-32602, "localPath is required"))?;
        let remote_path = params
            .get("remotePath")
            .and_then(|v| v.as_str())
            .filter(|s| !s.is_empty())
            .ok_or_else(|| PluginError::new(-32602, "remotePath is required"))?;
        let nodes: Vec<String> = params
            .get("nodes")
            .and_then(|v| v.as_array())
            .map(|a| {
                a.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .filter(|s| !s.is_empty())
                    .collect()
            })
            .unwrap_or_default();
        if nodes.is_empty() {
            return Err(PluginError::new(-32602, "nodes must contain at least one node"));
        }
        let recursive = params
            .get("recursive")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        let concurrency = params
            .get("concurrency")
            .and_then(|v| v.as_u64())
            .unwrap_or(10) as usize;
        let timeout_secs = params
            .get("timeoutSecs")
            .and_then(|v| v.as_u64())
            .unwrap_or(300);

        let transfer_id = format!(
            "t{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_nanos())
                .unwrap_or_default()
        );

        let opts = transfer::UploadOptions {
            tsh: setting.tsh.clone(),
            login: setting.ssh_login.clone(),
            nodes,
            local_path: local_path.to_string(),
            remote_path: remote_path.to_string(),
            recursive,
            concurrency,
            timeout_secs,
            transfer_id,
        };
        Ok(transfer::upload(opts, emitter.clone()))
    }

    /// Download one remote path from a node to a local path.
    fn handle_transfer_download(&self, params: &Value) -> Result<Value, PluginError> {
        let conn_id = params
            .get("connectionId")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let setting = conn_setting(conn_id).ok_or_else(|| {
            PluginError::new(-32000, "connection settings unavailable, please login first")
        })?;
        let node = params
            .get("node")
            .and_then(|v| v.as_str())
            .filter(|s| !s.is_empty())
            .ok_or_else(|| PluginError::new(-32602, "node is required"))?;
        let remote_path = params
            .get("remotePath")
            .and_then(|v| v.as_str())
            .filter(|s| !s.is_empty())
            .ok_or_else(|| PluginError::new(-32602, "remotePath is required"))?;
        let local_path = params
            .get("localPath")
            .and_then(|v| v.as_str())
            .filter(|s| !s.is_empty())
            .ok_or_else(|| PluginError::new(-32602, "localPath is required"))?;
        let recursive = params
            .get("recursive")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        let timeout_secs = params
            .get("timeoutSecs")
            .and_then(|v| v.as_u64())
            .unwrap_or(300);

        let transfer_id = format!(
            "t{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_nanos())
                .unwrap_or_default()
        );

        let item = transfer::download(
            &setting.tsh,
            &setting.ssh_login,
            node,
            remote_path,
            local_path,
            recursive,
            timeout_secs,
            &transfer_id,
        );
        if item.get("status").and_then(|v| v.as_str()) == Some("ok") {
            Ok(item)
        } else {
            let reason = item
                .get("stderr")
                .and_then(|v| v.as_str())
                .unwrap_or("download failed");
            Err(PluginError::new(-32000, reason.to_string()))
        }
    }
}

/// Batch correlation id: use the client-supplied one, else synthesize one from
/// pid + timestamp (no uuid dependency).
fn gen_batch_id(params: &Value) -> String {
    match params.get("batchId").and_then(|v| v.as_str()) {
        Some(s) if !s.is_empty() => s.to_string(),
        _ => {
            let millis = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_millis())
                .unwrap_or_default();
            format!("b{}-{}", std::process::id(), millis)
        }
    }
}

/// True when tsh can list resources, i.e. it holds a currently valid cert, and
/// that cert belongs to `proxy:port` as `user`.
fn has_valid_session(tsh: &str, proxy: &str, port: u64, user: &str) -> bool {
    // `tsh ls` succeeds only with a live, non-expired certificate.
    let out = Command::new(tsh)
        .args(["ls", "--format=json"])
        .output();
    let Ok(out) = out else {
        return false;
    };
    if !out.status.success() {
        return false;
    }
    let Some(status) = run_status(tsh) else {
        return false;
    };
    let proxy_url = status
        .get("proxy_url")
        .or_else(|| status.get("profile_url"))
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .replace("https://", "")
        .replace("http://", "");
    let status_user = status
        .get("user")
        .or_else(|| status.get("username"))
        .and_then(|v| v.as_str())
        .unwrap_or("");
    proxy_url == format!("{proxy}:{port}") && status_user == user
}

fn main() -> std::io::Result<()> {
    // The metadata id/version must exactly match manifest.json.
    let metadata = PluginMetadata::new("io.zdiai.teleport", env!("CARGO_PKG_VERSION"))
        .with_capability("connections");

    let plugin = TeleportPlugin {
        ssh: ssh::SshServer::new(),
        forward: Arc::new(forward::ForwardManager::new()),
    };
    // Framed transport is required for the binary terminal channels
    // (sendBinary in, emitter.binary out) used by the SSH overlay.
    PluginServer::new(metadata, plugin)
        .transport(PluginTransport::Framed)
        .serve()
}

// ---------------------------------------------------------------------------
// Login parameter extraction + connect result
// ---------------------------------------------------------------------------

fn build_login_params(params: &Value) -> Result<LoginParams, PluginError> {
    let conn = params.get("connection").ok_or_else(|| {
        PluginError::new(-32602, "missing connection in lifecycle params")
    })?;

    let proxy = conn
        .get("host")
        .and_then(|v| v.as_str())
        .filter(|s| !s.is_empty())
        .ok_or_else(|| PluginError::new(-32602, "proxy host is required"))?
        .to_string();
    let port = conn.get("port").and_then(|v| v.as_u64()).unwrap_or(443);
    let user = conn
        .get("username")
        .and_then(|v| v.as_str())
        .filter(|s| !s.is_empty())
        .ok_or_else(|| PluginError::new(-32602, "Teleport username is required"))?
        .to_string();
    let password = conn
        .get("password")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

    let auth = cfg_str(conn, "auth_connector").unwrap_or_else(|| "local".to_string());
    if auth != "local" {
        return Err(PluginError::new(
            -32602,
            format!(
                "Phase 1 supports local (username + password) login only; '{auth}' SSO login comes in a later phase."
            ),
        ));
    }
    let mfa = cfg_str(conn, "mfa_type").unwrap_or_else(|| "otp".to_string());
    let cluster = cfg_str(conn, "cluster");
    // Remote OS login for SSH sessions; root matches this deployment.
    let ssh_login = cfg_str(conn, "ssh_login")
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "root".to_string());
    let tsh = tsh_bin(&cfg_str(conn, "tsh_path"));

    Ok(LoginParams {
        tsh,
        proxy,
        port,
        user,
        password,
        auth,
        mfa,
        cluster,
        ssh_login,
    })
}

fn build_connect_result(tsh: &str) -> Value {
    match run_status(tsh) {
        Some(status) => {
            let cluster = status.get("cluster").and_then(|v| v.as_str()).unwrap_or("");
            let proxy_url = status
                .get("proxy_url")
                .or_else(|| status.get("profile_url"))
                .and_then(|v| v.as_str())
                .unwrap_or("");
            // v18 exposes valid_until as RFC3339; older tsh used epoch seconds.
            let expires_ms = status
                .get("valid_until")
                .or_else(|| status.get("expires"))
                .and_then(|v| v.as_str())
                .and_then(rfc3339_to_epoch_ms)
                .or_else(|| status.get("valid_until").and_then(|v| v.as_u64()))
                .unwrap_or(0);
            let kube_enabled = status
                .get("kubernetes_enabled")
                .or_else(|| status.get("kube_enabled"))
                .and_then(|v| v.as_bool())
                .unwrap_or(false);
            json!({
                "success": true,
                "connected": true,
                "message": format!("Logged in to {cluster}"),
                "cluster": cluster,
                "proxy_url": proxy_url,
                "expires": expires_ms,
                "kube_enabled": kube_enabled
            })
        }
        None => json!({
            "success": true,
            "connected": true,
            "message": "tsh login succeeded, but `tsh status` returned no parseable profile."
        }),
    }
}

// ---------------------------------------------------------------------------
// test / logout / refresh
// ---------------------------------------------------------------------------

fn handle_test(params: &Value) -> Value {
    let conn = params.get("connection").unwrap_or(&Value::Null);
    let host = conn.get("host").and_then(|v| v.as_str()).unwrap_or("");
    let port = conn.get("port").and_then(|v| v.as_u64()).unwrap_or(443);
    let tsh = tsh_bin(&cfg_str(conn, "tsh_path"));

    match Command::new(&tsh).arg("version").output() {
        Ok(out) if out.status.success() => {
            let first_line = String::from_utf8_lossy(&out.stdout)
                .lines()
                .next()
                .unwrap_or("tsh")
                .trim()
                .to_string();
            json!({
                "success": true,
                "message": format!("{first_line} — tsh found. Proxy {host}:{port} ready for login.")
            })
        }
        Ok(out) => json!({
            "success": false,
            "message": format!("tsh command failed: {}", String::from_utf8_lossy(&out.stderr).trim())
        }),
        Err(_) => json!({
            "success": false,
            "message": "tsh not found on PATH. Install the Teleport CLI or set the 'tsh binary path' field."
        }),
    }
}

fn handle_logout(params: &Value) -> Value {
    let conn = params.get("connection").unwrap_or(&Value::Null);
    let tsh = tsh_bin(&cfg_str(conn, "tsh_path"));

    match Command::new(&tsh).arg("logout").output() {
        Ok(out) if out.status.success() => {
            json!({ "success": true, "message": "Logged out of Teleport." })
        }
        Ok(out) => json!({
            "success": false,
            "message": format!("tsh logout failed: {}", String::from_utf8_lossy(&out.stderr).trim())
        }),
        Err(e) => {
            json!({ "success": false, "message": format!("failed to run tsh logout: {e}") })
        }
    }
}

fn handle_refresh(params: &Value) -> Value {
    let conn = params.get("connection").unwrap_or(&Value::Null);
    let tsh = tsh_bin(&cfg_str(conn, "tsh_path"));

    match run_status(&tsh) {
        Some(status) => json!({
            "success": true,
            "message": "Refreshed Teleport profile.",
            "status": status
        }),
        None => json!({
            "success": false,
            "message": "No active Teleport session. Run tsh login first."
        }),
    }
}

/// Custom RPC `teleport/listResources` — return all nodes, apps, and kube
/// clusters for the active session. Tolerates params being the connection
/// object itself.
fn handle_list_resources(params: &Value) -> Value {
    let conn = params
        .get("connection")
        .filter(|v| v.is_object())
        .unwrap_or(params);
    let tsh = tsh_bin(&cfg_str(conn, "tsh_path"));

    let (items, failures) = resources::list_all(&tsh);
    let logged_in = run_status(&tsh).is_some();
    trace(&format!(
        "listResources: tsh={tsh} items={} logged_in={logged_in} failures={}",
        items.len(),
        failures
            .iter()
            .map(|(k, m)| format!("{:?}:{m}", k))
            .collect::<Vec<_>>()
            .join(" | ")
    ));
    let errors: Vec<Value> = failures
        .iter()
        .map(|(kind, message)| {
            let k = match kind {
                resources::Kind::Node => "node",
                resources::Kind::App => "app",
                resources::Kind::Kube => "kube",
            };
            json!({ "kind": k, "message": message })
        })
        .collect();

    if items.is_empty() && !logged_in {
        return json!({
            "success": false,
            "resources": [],
            "message": "No active Teleport session. Run tsh login first (cert may have expired)."
        });
    }

    json!({
        "success": true,
        "resources": items,
        "count": items.len(),
        "errors": errors,
        "message": format!("Found {} resource(s).", items.len())
    })
}

/// Run `tsh status --format=json`. tsh may print either a single profile object
/// or an array of profiles (newer versions); return the active/first one.
fn run_status(tsh: &str) -> Option<Value> {
    let out = Command::new(tsh)
        .args(["status", "--format=json"])
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    let parsed: Value = serde_json::from_slice(&out.stdout).ok()?;
    let obj = match parsed {
        Value::Array(list) => list
            .iter()
            .find(|v| v.get("active").and_then(|a| a.as_bool()).unwrap_or(false))
            .or_else(|| list.first())
            .cloned()?,
        // Newer tsh wraps the profile in `active`.
        Value::Object(ref m) if m.contains_key("active") => {
            m.get("active").cloned()?
        }
        obj @ Value::Object(_) => obj,
        _ => return None,
    };
    Some(obj)
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Read a form value bound to config from a connection object (runtime objects
/// expose it as external_config).
fn cfg_str(conn: &Value, key: &str) -> Option<String> {
    // Runtime connection objects carry form values under external_config; keep
    // config as a fallback for other entry shapes.
    conn.get("external_config")
        .or_else(|| conn.get("config"))
        .and_then(|c| c.get(key))
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
}

/// Resolve the tsh binary, honouring an explicit path and ensuring .exe on Windows.
fn tsh_bin(configured: &Option<String>) -> String {
    let raw = configured
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .unwrap_or("tsh");
    if cfg!(windows) {
        let base = raw.trim_end_matches(".exe");
        format!("{base}.exe")
    } else {
        raw.to_string()
    }
}

/// Parse an RFC3339 timestamp (e.g. "2026-10-01T13:03:50+08:00") into epoch
/// milliseconds. Matches YYYY-MM-DDTHH:MM:SS with optional fraction and the
/// common timezone shapes (+08:00, Z).
fn rfc3339_to_epoch_ms(s: &str) -> Option<u64> {
    let s = s.trim();
    let (rest, tz) = if let Some(idx) = s.rfind('+') {
        (&s[..idx], &s[idx..])
    } else if let Some(idx) = s.rfind('-') {
        // "-" must belong to a timezone, not the date separator at index 10.
        if idx > 10 {
            (&s[..idx], &s[idx..])
        } else {
            (s, "Z")
        }
    } else if s.ends_with('Z') {
        (&s[..s.len() - 1], "Z")
    } else {
        (s, "Z")
    };
    let tz_num: i32 = match tz {
        "Z" => 0,
        _ => {
            let digits: String = tz.chars().filter(|c| c.is_ascii_digit()).collect();
            if digits.len() != 4 {
                return None;
            }
            let hh: i32 = digits.get(0..2)?.parse().ok()?;
            let mm: i32 = digits.get(2..4)?.parse().ok()?;
            if tz.contains('-') {
                -(hh * 60 + mm)
            } else {
                hh * 60 + mm
            }
        }
    };
    let date = rest.split('T').next()?;
    let time = rest.split('T').nth(1)?;
    let time = time.split('.').collect::<Vec<_>>()[0];
    let (y, mo, d): (i64, u32, u32) = {
        let mut it = date.split('-');
        (it.next()?.parse().ok()?, it.next()?.parse().ok()?, it.next()?.parse().ok()?)
    };
    let (h, mi, s2): (u32, u32, u32) = {
        let mut it = time.split(':');
        (it.next()?.trim().parse().ok()?, it.next()?.trim().parse().ok()?, it.next()?.trim().parse().ok()?)
    };
    // Days of elapsed months before each month (non-leap baseline).
    const CUM: [i64; 12] = [0,31,59,90,120,151,181,212,243,273,304,334];
    let is_leap = (y % 4 == 0 && y % 100 != 0) || y % 400 == 0;
    let yadj = y - 1;
    // Days since 0001-01-01 (proleptic Gregorian).
    let days_from_ce = yadj * 365 + yadj / 4 - yadj / 100 + yadj / 400 + CUM[(mo - 1) as usize] + if mo > 2 && is_leap { 1 } else { 0 } + (d as i64 - 1);
    // Days from 0001-01-01 to 1970-01-01.
    const DAYS_TO_EPOCH: i64 = 719162;
    let secs_utc = (days_from_ce - DAYS_TO_EPOCH) * 86400 + (h as i64) * 3600 + (mi as i64) * 60 + s2 as i64 - (tz_num as i64) * 60;
    Some((secs_utc as u64).checked_mul(1000)?)
}
