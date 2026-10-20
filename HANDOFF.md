# HANDOFF — aud-rust

Last updated: 2026-10-20 (M0 complete, tagged v0.0.1).

## State

M0 is complete and tagged `v0.0.1` (PR #1 merged, all CI green). Done: Cargo workspace (`aud-core`, `aud-cli`), CI (lint, 3-OS test with FFmpeg, coverage, supply-chain, msrv), SECURITY/CONTRIBUTING/dependabot/deny.toml, README badges, GitHub description/topics, private vulnerability reporting, and branch protection on `main` (requires `lint`, the three `test` jobs, `supply-chain`; PR required; no force-push). Local toolchain: Rust 1.99.1, cargo-deny, cargo-audit, cargo-llvm-cov.

- Reference: Python `aud` @ `bb01db7`. Local checkout `S:\ZX\Programming\aud`; `.reference/aud` also cloned by `scripts/fetch-reference.ps1` (gitignored).
- Only `aud --version` works. No feature code yet.

## Next action

Start **M1 — Tracer bullet**: [docs/plan/milestones/M1-tracer-bullet.md](docs/plan/milestones/M1-tracer-bullet.md).

Kickoff prompt for a fresh session: [prompts/kickoff.md](prompts/kickoff.md).

## Read order for a cold session

1. [AGENTS.md](AGENTS.md)
2. [docs/plan/README.md](docs/plan/README.md) (index) → `01-scope.md`, `02-architecture.md`
3. The active milestone file
4. [docs/plan/05-porting-traps.md](docs/plan/05-porting-traps.md) before porting any module

## Open decisions (none block M1)

| # | Question | Default if unanswered | Decided by |
| --- | --- | --- | --- |
| 1 | crates.io names (`aud` is probably taken) | `aud-core`, `aud-cli` on crates.io; binary stays `aud` | M9 |
| 2 | Pure-Rust media engine as stretch | Not started; FFmpeg engine only ([ADR-0001](docs/adr/0001-ffmpeg-subprocess-engine.md)) | after M8 |
| 3 | Binary distribution (`cargo-dist`) | Yes, GitHub Releases | M9 |

## Blockers

None.
