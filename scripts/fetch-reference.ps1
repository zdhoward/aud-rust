# Clone the pinned Python reference implementation into .reference/aud (gitignored).
# Usage: pwsh scripts/fetch-reference.ps1
$ErrorActionPreference = 'Stop'
$Pin = 'bb01db7cad9795f37a3fbe14cc8c59da4d9b8eb0'   # keep in sync with spec/README.md
$Root = Split-Path -Parent $PSScriptRoot
$Dest = Join-Path $Root '.reference/aud'

if (-not (Test-Path $Dest)) {
    git clone https://github.com/zdhoward/aud.git $Dest
    if ($LASTEXITCODE -ne 0) { throw 'git clone failed' }
}
git -C $Dest fetch --quiet origin
if ($LASTEXITCODE -ne 0) { throw 'git fetch failed' }
git -C $Dest checkout --quiet $Pin
if ($LASTEXITCODE -ne 0) { throw 'git checkout failed' }
Write-Host "Reference ready at $Dest ($Pin)"
