#!/usr/bin/env bash
set -euo pipefail

command -v node >/dev/null || { echo "Node.js is required"; exit 1; }
command -v cargo >/dev/null || { echo "Rust/Cargo is required"; exit 1; }
command -v python3 >/dev/null || { echo "Python 3 is required"; exit 1; }

echo "VoicePilot development prerequisites look available."
echo "Next: npm install && cd inference && python3 -m venv .venv && . .venv/bin/activate && pip install -e ."
