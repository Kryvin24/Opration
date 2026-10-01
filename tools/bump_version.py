#!/usr/bin/env python3
"""Synchronize the plugin version across manifest.json and backend/Cargo.toml.

Cargo.lock picks up the new version on the next `cargo build`, so it is not
edited by hand (manual edits drift and conflict with cargo's own resolver).

Usage:
    python tools/bump_version.py 0.1.22
    python tools/bump_version.py --patch     # 0.1.21 -> 0.1.22
    python tools/bump_version.py --minor     # 0.1.21 -> 0.2.0
"""
import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
MANIFEST = ROOT / "manifest.json"
CARGO_TOML = ROOT / "backend" / "Cargo.toml"
SEMVER = re.compile(r"^(\d+)\.(\d+)\.(\d+)$")


def current_version() -> str:
    data = json.loads(MANIFEST.read_text(encoding="utf-8"))
    return data["version"]


def bump(ver: str, kind: str) -> str:
    m = SEMVER.match(ver)
    if not m:
        sys.exit(f"current version is not semver: {ver!r}")
    major, minor, patch = (int(x) for x in m.groups())
    if kind == "patch":
        patch += 1
    elif kind == "minor":
        minor += 1
        patch = 0
    elif kind == "major":
        major += 1
        minor = patch = 0
    else:
        sys.exit(f"unknown bump kind: {kind}")
    return f"{major}.{minor}.{patch}"


def set_manifest(ver: str) -> None:
    text = MANIFEST.read_text(encoding="utf-8")
    data = json.loads(text)
    data["version"] = ver
    # Preserve the file's existing indent style (2 spaces across the project).
    MANIFEST.write_text(json.dumps(data, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")


def set_cargo_toml(ver: str) -> None:
    text = CARGO_TOML.read_text(encoding="utf-8")
    new_text, n = re.subn(
        r'^(version\s*=\s*")\d+\.\d+\.\d+(")',
        rf"\g<1>{ver}\g<2>",
        text,
        count=1,
        flags=re.MULTILINE,
    )
    if n != 1:
        sys.exit("could not find the package version line in backend/Cargo.toml")
    CARGO_TOML.write_text(new_text, encoding="utf-8")


def main() -> None:
    args = sys.argv[1:]
    if not args:
        sys.exit(__doc__)

    old = current_version()
    if args[0] in ("--patch", "--minor", "--major"):
        new = bump(old, args[0].lstrip("-"))
    else:
        new = args[0]
        if not SEMVER.match(new):
            sys.exit(f"target version must look like 1.2.3, got {new!r}")

    if new == old:
        sys.exit(f"already at {old}")

    set_manifest(new)
    set_cargo_toml(new)
    print(f"version: {old} -> {new}")
    print("note: run a cargo build so Cargo.lock refreshes.")


if __name__ == "__main__":
    main()
