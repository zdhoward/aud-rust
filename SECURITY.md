# Security policy

## Reporting a vulnerability

Report vulnerabilities privately through GitHub: **Security → Report a vulnerability** on
<https://github.com/zdhoward/aud-rust/security/advisories/new>. Do not open a public issue.

Expect an acknowledgement within a week. This is a pre-1.0 portfolio project; only the latest
release and `main` receive fixes.

## Scope

`aud` reads and writes files and runs `ffmpeg`/`ffprobe` as subprocesses. Reports about path
handling, overwrite protection, staged writes, archive output, or subprocess argument handling are
in scope.
