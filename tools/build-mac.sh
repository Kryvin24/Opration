#!/bin/sh
# Build the macOS .dbxp of the DBX Teleport plugin.
#
# Prerequisites on the Mac:
#   - Rust toolchain:  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
#   - Node.js:         https://nodejs.org  (or `brew install node`)
#   - tsh (Teleport client) installed, for the plugin to use at runtime
#
# The plugin-cli builds the native sidecar for the current host, so the
# produced artifact matches the Mac you run this on (Intel -> darwin-x64,
# Apple Silicon -> darwin-arm64).
set -e
cd "$(dirname "$0")/.."
npx -y @dbx-app/plugin-cli package . --output-dir dist
echo "Done. See dist/ for the .dbxp artifact."
