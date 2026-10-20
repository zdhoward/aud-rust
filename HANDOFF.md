# HANDOFF — aud-rust

Last updated: 2026-10-20 (planning session; no Rust code written yet).

## State

Planning is complete. The repo holds the plan, a frozen spec snapshot, the conformance corpus, and agent instructions. **No code, no CI, no Cargo workspace yet.** Rust is not installed on the dev machine.

- Branch `main`, one commit of planning artifacts.
- GitHub repo `zdhoward/aud-rust` (public) created from this tree.
- Reference: Python `aud` @ `bb01db7` (v2.0.2 + unreleased readiness work, merged on `master`). Local checkout: `S:\ZX\Programming\aud`.

## Next action

Execute **M0 — Bootstrap**: [docs/plan/milestones/M0-bootstrap.md](docs/plan/milestones/M0-bootstrap.md). First step is installing the Rust toolchain (`winget install Rustlang.Rustup`, plus MSVC build tools if `link.exe` is missing), then following TODO.md top to bottom.

Kickoff prompt for a fresh session: [prompts/kickoff.md](prompts/kickoff.md).

## Read order for a cold session

1. [AGENTS.md](AGENTS.md)
2. [docs/plan/README.md](docs/plan/README.md) (index) → `01-scope.md`, `02-architecture.md`
3. The active milestone file
4. [docs/plan/05-porting-traps.md](docs/plan/05-porting-traps.md) before porting any module

## Open decisions (none block M0)

| # | Question | Default if unanswered | Decided by |
| --- | --- | --- | --- |
| 1 | crates.io names (`aud` is probably taken) | `aud-core`, `aud-cli` on crates.io; binary stays `aud` | M9 |
| 2 | Pure-Rust media engine as stretch | Not started; FFmpeg engine only ([ADR-0001](docs/adr/0001-ffmpeg-subprocess-engine.md)) | after M8 |
| 3 | Binary distribution (`cargo-dist`) | Yes, GitHub Releases | M9 |

## Blockers

None. M0 needs the user only to confirm `winget` installs if prompted.
