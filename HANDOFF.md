# HANDOFF — aud-rust

Last updated: 2026-10-20 (M0 implemented; PR #1 open, CI green, not merged).

## State

M0 is implemented on branch `chore/m0-bootstrap` ([PR #1](https://github.com/zdhoward/aud-rust/pull/1), all CI jobs green, **not merged**). Done: Cargo workspace (`aud-core`, `aud-cli`), CI (lint, 3-OS test with FFmpeg, coverage, supply-chain, msrv), SECURITY/CONTRIBUTING/dependabot/deny.toml, README badges, GitHub description/topics, private vulnerability reporting, and branch protection on `main` (requires `lint`, the three `test` jobs, `supply-chain`; PR required; no force-push). Local toolchain: Rust 1.99.1, cargo-deny, cargo-audit, cargo-llvm-cov.

- Reference: Python `aud` @ `bb01db7`. Local checkout `S:\ZX\Programming\aud`; `.reference/aud` also cloned by `scripts/fetch-reference.ps1` (gitignored).
- Only `aud --version` works. No feature code yet.

## Next action

1. User merges PR #1 (merge commit). Then on `main`: confirm CI green, add CHANGELOG `0.0.1`, write the final PROJECT_LOG exit note, tick `M0-X` in TODO.md, tag annotated `v0.0.1`, and verify `gh api repos/zdhoward/aud-rust/branches/main/protection`.
2. Start **M1 — Tracer bullet**: [docs/plan/milestones/M1-tracer-bullet.md](docs/plan/milestones/M1-tracer-bullet.md).

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

PR #1 awaits the user's merge. `code-review` has not been run on the PR diff.
