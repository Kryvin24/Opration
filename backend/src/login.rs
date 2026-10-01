//! Drives `tsh login` inside a ConPTY pseudo-console.
//!
//! Design notes:
//! - tsh refuses password login on a plain pipe, so it must run attached to a
//!   real terminal; ConPTY gives us one while keeping the sidecar windowless.
//! - `--force` makes tsh re-authenticate even if ~/.tsh still holds a valid
//!   cert, so this code path always runs the full password -> OTP exchange.
//!   (main.rs short-circuits when a valid session already exists.)
//! - OTP is collected fresh via host/requestUserInput and NEVER stored.
//! - ConPTY on Windows does not reliably deliver EOF on the master read, so the
//!   loop is driven by a reader thread plus polling child.try_wait().
//! - A strict Stage state machine (Password -> Otp -> Done) prevents the prompt
//!   matcher from firing twice: tsh re-renders screen text constantly, and a
//!   loose `contains("password")` also matches error strings like "invalid
//!   username, password or second factor".

use std::io::{Read, Write};
use std::sync::mpsc::{self, RecvTimeoutError};
use std::thread;
use std::time::Duration;

use dbx_plugin_sdk::{
    host_client, trace, PluginError, UserInputAnswer, UserInputPrompt,
    HOST_REQUEST_USER_INPUT_METHOD,
};
use portable_pty::{native_pty_system, CommandBuilder, PtySize};

pub struct LoginParams {
    pub tsh: String,
    pub proxy: String,
    pub port: u64,
    pub user: String,
    pub password: String,
    pub auth: String,
    pub mfa: String,
    pub cluster: Option<String>,
    /// Remote OS login pinned onto SSH sessions (defaults to "root").
    pub ssh_login: String,
}

/// Once we have fed a stage's input we move on irrevocably, so a redraw or an
/// error message containing prompt-like words can never re-trigger a feed.
#[derive(Clone, Copy, PartialEq)]
enum Stage {
    Password,
    Otp,
    Done,
}

/// Run the interactive login. Returns `Ok(())` only when tsh exits successfully.
pub fn run_login(p: &LoginParams) -> Result<(), String> {
    trace(&format!(
        "login: start proxy={}:{} user={} auth={} mfa={} cluster={:?} tsh={}",
        p.proxy, p.port, p.user, p.auth, p.mfa, p.cluster, p.tsh
    ));

    let pty_system = native_pty_system();
    let pair = pty_system
        .openpty(PtySize {
            rows: 30,
            cols: 120,
            pixel_width: 0,
            pixel_height: 0,
        })
        .map_err(|e| {
            trace(&format!("login: ConPTY open failed: {e}"));
            format!("ConPTY unavailable on this host: {e}")
        })?;
    trace("login: ConPTY opened");

    let mut cmd = CommandBuilder::new(&p.tsh);
    cmd.arg("login");
    cmd.arg(format!("--proxy={}:{}", p.proxy, p.port));
    cmd.arg(format!("--user={}", p.user));
    cmd.arg(format!("--auth={}", p.auth));
    cmd.arg(format!("--mfa-mode={}", p.mfa));
    // Force re-authentication: without this, a still-valid cert in ~/.tsh makes
    // tsh print the profile dashboard and skip the password/OTP flow entirely.
    cmd.arg("--force");
    if let Some(cluster) = &p.cluster {
        if !cluster.trim().is_empty() {
            cmd.arg(cluster.trim());
        }
    }

    let mut child = pair
        .slave
        .spawn_command(cmd)
        .map_err(|e| {
            trace(&format!("login: spawn failed: {e}"));
            format!("failed to spawn {}: {e}", p.tsh)
        })?;
    trace("login: tsh spawned");
    // Drop our slave handle so nothing keeps the pty open on our side.
    drop(pair.slave);

    let mut reader = pair
        .master
        .try_clone_reader()
        .map_err(|e| format!("PTY reader error: {e}"))?;
    let mut writer = pair
        .master
        .take_writer()
        .map_err(|e| format!("PTY writer error: {e}"))?;

    // Reader thread: sends Some(bytes) per chunk, None on EOF/error.
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let mut buf = [0u8; 4096];
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
    });

    let mut stage = Stage::Password;
    // Sliding window for prompt matching; cleared after each feed so stale text
    // is never re-matched.
    let mut match_buf = String::new();
    // Sliding window kept for error reporting (never cleared).
    let mut recent = String::new();

    let exit_status = loop {
        match rx.recv_timeout(Duration::from_millis(150)) {
            Ok(Some(bytes)) => {
                let chunk = String::from_utf8_lossy(&bytes).to_string();
                trace(&format!("login pty: {}", sanitize(&chunk, 300)));
                recent.push_str(&chunk);
                if recent.len() > 600 {
                    let cut = recent.len() - 600;
                    recent.drain(..cut);
                }
                match_buf.push_str(&chunk);
                if match_buf.len() > 512 {
                    let cut = match_buf.len() - 512;
                    match_buf.drain(..cut);
                }
                let low = match_buf.to_lowercase();

                if !matches!(stage, Stage::Done) && is_otp_prompt(&low) {
                    // Asking for OTP ends prompt matching: tsh re-renders the
                    // prompt after every answer, and a rejected code must NOT
                    // spawn another dialog (each TOTP code is single-use).
                    stage = Stage::Done;
                    match_buf.clear();
                    trace("login: OTP prompt matched, asking host");
                    match ask_otp() {
                        Ok(answer) => match answer.submitted() {
                            Some(code) if !code.trim().is_empty() => {
                                let code = code.trim().to_string();
                                trace(&format!(
                                    "login: OTP submitted (len={})",
                                    code.len()
                                ));
                                if let Err(e) = feed(&mut writer, &code) {
                                    let _ = child.kill();
                                    return Err(format!(
                                        "{e}. Last tsh output: {}",
                                        sanitize(&recent, 400)
                                    ));
                                }
                            }
                            Some(_) => {
                                let _ = child.kill();
                                return Err("Empty verification code".to_string());
                            }
                            None => {
                                let _ = child.kill();
                                return Err(if answer.is_cancelled() {
                                    "Verification code prompt was cancelled.".to_string()
                                } else if answer.is_timeout() {
                                    "Verification code prompt timed out.".to_string()
                                } else {
                                    "Unexpected MFA prompt response.".to_string()
                                });
                            }
                        },
                        Err(e) => {
                            let _ = child.kill();
                            trace(&format!("login: OTP ask failed: {e}"));
                            return Err(format!(
                                "Could not collect the verification code: {e}. Last tsh output: {}",
                                sanitize(&recent, 400)
                            ));
                        }
                    }
                } else if stage == Stage::Password && is_password_prompt(&low) {
                    stage = Stage::Otp;
                    match_buf.clear();
                    trace("login: password prompt matched");
                    if let Err(e) = feed(&mut writer, &p.password) {
                        let _ = child.kill();
                        return Err(format!(
                            "{e}. Last tsh output: {}",
                            sanitize(&recent, 400)
                        ));
                    }
                }
            }
            Ok(None) => {
                // EOF — tsh exited; collect its status.
                break child.wait().map_err(|e| format!("tsh wait failed: {e}"))?;
            }
            Err(RecvTimeoutError::Timeout) => {
                // No new output for a while; poll whether tsh has exited.
                match child.try_wait() {
                    Ok(Some(status)) => break status,
                    Ok(None) => {}
                    Err(e) => return Err(format!("tsh wait failed: {e}")),
                }
            }
            Err(RecvTimeoutError::Disconnected) => {
                break child.wait().map_err(|e| format!("tsh wait failed: {e}"))?;
            }
        }
    };

    trace(&format!("login: tsh exit status: {:?}", exit_status.success()));
    if exit_status.success() {
        Ok(())
    } else {
        let low_recent = recent.to_lowercase();
        let hint = if low_recent.contains("second factor")
            || low_recent.contains("invalid username")
        {
            " The OTP code changes every 30 seconds and each code can be used only once — enter the CURRENT code from your authenticator app when retrying."
        } else if low_recent.contains("password") {
            " Check that the saved password in the connection form is correct."
        } else {
            ""
        };
        Err(format!(
            "tsh login failed.{hint} Last tsh output: {}",
            sanitize(&recent, 400)
        ))
    }
}

/// Ask the user for the OTP through the DBX host UI. Bounded to 110s so a host
/// that never answers fails before the connection/action deadline.
fn ask_otp() -> Result<UserInputAnswer, String> {
    let client = host_client()
        .ok_or_else(|| "the plugin server is not running".to_string())?;
    let prompt = UserInputPrompt::secret("Enter the CURRENT 6-digit code from your authenticator app")
        .with_title("Teleport verification code")
        .with_timeout_secs(100);
    let params = serde_json::to_value(&prompt).map_err(|e| e.to_string())?;
    let answer = client
        .request_with_timeout(
            HOST_REQUEST_USER_INPUT_METHOD,
            params,
            Duration::from_secs(110),
        )
        .map_err(|e: PluginError| format!("{} (code {})", e.message, e.code))?;
    serde_json::from_value(answer).map_err(|e| format!("invalid host answer: {e}"))
}

fn feed(writer: &mut Box<dyn Write + Send>, secret: &str) -> Result<(), String> {
    // ConPTY expects CR as the Enter key.
    writer
        .write_all(format!("{secret}\r").as_bytes())
        .map_err(|e| format!("failed to send input to tsh: {e}"))?;
    writer
        .flush()
        .map_err(|e| format!("failed to flush input to tsh: {e}"))
}

/// Strip ANSI/control characters for logging and error messages.
fn sanitize(text: &str, max: usize) -> String {
    let mut out = String::with_capacity(max.min(text.len()));
    for c in text.chars() {
        if out.len() >= max {
            break;
        }
        out.push(if c.is_control() { ' ' } else { c });
    }
    out.trim().to_string()
}

/// Match the tsh local-auth password prompt. Deliberately narrow: bare
/// "password" also appears in "passwordless", help text, and error strings
/// like "invalid username, password or second factor".
fn is_password_prompt(low: &str) -> bool {
    low.contains("enter password") || low.contains("password:")
}

/// Match the TOTP / authenticator prompt ("Enter an OTP code from a device:").
fn is_otp_prompt(low: &str) -> bool {
    low.contains("otp code")
        || low.contains("verification code")
        || low.contains("totp")
        || (low.contains("authenticator") && low.contains("code"))
}
