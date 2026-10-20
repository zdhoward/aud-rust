# Build the Rust binary and run the Python-vs-Rust differential scenarios.
# Usage: powershell -File scripts/differential.ps1 [--reference DIR] [--only ID]
$ErrorActionPreference = 'Stop'
Set-Location (Split-Path -Parent $PSScriptRoot)
cargo build --quiet
if ($LASTEXITCODE -ne 0) { throw 'cargo build failed' }
$py = if ($env:PYTHON) { $env:PYTHON } else { 'python' }
& $py scripts/differential.py @args
exit $LASTEXITCODE
