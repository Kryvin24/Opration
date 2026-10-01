//! Lists Teleport resources (SSH nodes, applications, Kubernetes clusters) using
//! the non-interactive `tsh ... ls --format=json` commands. These require an
//! active login but no TTY, so they are plain subprocess calls.
//!
//! tsh's JSON keys differ slightly across versions and resource kinds, so the
//! normalizers try several candidate keys and always keep the raw object.

use std::process::Command;

use serde_json::{json, Map, Value};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Kind {
    Node,
    App,
    Kube,
}

/// List all three resource kinds and return one normalized array.
/// `failures` collects per-kind errors so a single empty/missing kind does not
/// wipe out the whole result.
pub fn list_all(tsh: &str) -> (Vec<Value>, Vec<(Kind, String)>) {
    let mut out = Vec::new();
    let mut failures = Vec::new();

    match list_kind(tsh, Kind::Node) {
        Ok(items) => out.extend(items),
        Err(e) => failures.push((Kind::Node, e)),
    }
    match list_kind(tsh, Kind::App) {
        Ok(items) => out.extend(items),
        Err(e) => failures.push((Kind::App, e)),
    }
    match list_kind(tsh, Kind::Kube) {
        Ok(items) => out.extend(items),
        Err(e) => failures.push((Kind::Kube, e)),
    }

    (out, failures)
}

fn list_kind(tsh: &str, kind: Kind) -> Result<Vec<Value>, String> {
    let args = match kind {
        Kind::Node => vec!["ls", "--format=json"],
        Kind::App => vec!["apps", "ls", "--format=json"],
        Kind::Kube => vec!["kube", "ls", "--format=json"],
    };

    let output = Command::new(tsh)
        .args(args)
        .output()
        .map_err(|e| format!("failed to run tsh: {e}"))?;

    dbx_plugin_sdk::trace(&format!(
        "listKind {:?}: exit={:?} stdout_len={} stderr={}",
        kind,
        output.status.code(),
        output.stdout.len(),
        String::from_utf8_lossy(&output.stderr).trim()
    ));

    if !output.status.success() {
        let err = String::from_utf8_lossy(&output.stderr);
        let trimmed = err.trim();
        return Err(if trimmed.is_empty() {
            "tsh returned no resources (are you logged in?)".to_string()
        } else {
            trimmed.to_string()
        });
    }

    let parsed: Value = serde_json::from_slice(&output.stdout)
        .map_err(|e| format!("could not parse tsh JSON: {e}"))?;

    let rows: Vec<Value> = match parsed {
        Value::Array(a) => a,
        Value::Object(_) => vec![parsed],
        _ => return Ok(Vec::new()),
    };

    Ok(rows
        .into_iter()
        .filter(|v| v.is_object())
        .map(|v| normalize(kind, v))
        .collect())
}

fn normalize(kind: Kind, raw: Value) -> Value {
    let obj = match &raw {
        Value::Object(m) => m.clone(),
        _ => Map::new(),
    };

    let kind_str = match kind {
        Kind::Node => "node",
        Kind::App => "app",
        Kind::Kube => "kube",
    };

    let name = match kind {
        Kind::Node => {
            // v18 resource objects: metadata.name is the UUID id, spec.hostname
            // is the human-readable node name — the hostname must win.
            deep_str(&obj, &["spec", "hostname"])
                .or_else(|| first_str(&obj, &["hostname", "node_name", "name"]))
                .or_else(|| deep_str(&obj, &["spec", "node_name"]))
                .or_else(|| deep_str(&obj, &["metadata", "name"]))
        }
        Kind::App => first_str(&obj, &["name", "app_name"])
            .or_else(|| deep_str(&obj, &["metadata", "name"])),
        Kind::Kube => first_str(&obj, &["cluster_name", "name", "kube_cluster"])
            .or_else(|| deep_str(&obj, &["metadata", "name"])),
    };

    // Secondary line: for nodes show the stable id (UUID) beneath the name.
    let address = match kind {
        Kind::Node => deep_str(&obj, &["metadata", "name"])
            .or_else(|| first_str(&obj, &["addr", "address", "public_addr"]))
            .or_else(|| deep_str(&obj, &["spec", "addr"])),
        Kind::App => first_str(&obj, &["public_addr", "uri", "fqdn", "url"])
            .or_else(|| deep_str(&obj, &["spec", "public_addr"]))
            .or_else(|| deep_str(&obj, &["status", "public_addr"])),
        Kind::Kube => first_str(&obj, &["addr", "address"])
            .or_else(|| deep_str(&obj, &["spec", "addr"])),
    };

    let description = match kind {
        Kind::App => first_str(&obj, &["description", "desc"]),
        _ => None,
    };

    let labels = obj
        .get("labels")
        .or_else(|| obj.get("metadata").and_then(|m| m.get("labels")))
        .cloned()
        .unwrap_or(Value::Null);
    let id = format!("{kind_str}:{}", name.clone().unwrap_or_default());

    json!({
        "id": id,
        "kind": kind_str,
        "name": name,
        "address": address,
        "description": description,
        "labels": labels,
        "raw": raw
    })
}

fn first_str(obj: &Map<String, Value>, keys: &[&str]) -> Option<String> {
    for k in keys {
        if let Some(v) = obj.get(*k) {
            match v.as_str() {
                Some(s) if !s.is_empty() => return Some(s.to_string()),
                _ => {}
            }
        }
    }
    None
}

/// Read a string nested along a dotted path, e.g. ["metadata", "name"] for
/// `{ "metadata": { "name": "..." } }`. Returns None on any non-object step.
fn deep_str(obj: &Map<String, Value>, path: &[&str]) -> Option<String> {
    let mut current = Value::Object(obj.clone());
    for (i, key) in path.iter().enumerate() {
        current = current.get(*key)?.clone();
        if i == path.len() - 1 {
            return match current {
                Value::String(s) if !s.is_empty() => Some(s),
                _ => None,
            };
        }
    }
    None
}
