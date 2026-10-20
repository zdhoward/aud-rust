#!/usr/bin/env bash
# Build the Rust binary and run the Python-vs-Rust differential scenarios.
# Usage: scripts/differential.sh [--reference DIR] [--only ID]
set -euo pipefail
cd "$(dirname "$0")/.."
cargo build --quiet
exec "${PYTHON:-python3}" scripts/differential.py "$@"
