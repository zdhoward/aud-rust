# HANDOFF — aud-rust

Last updated: 2026-10-20 (M1 complete, tagged v0.1.0).

## State

M0 (v0.0.1) and M1 (v0.1.0) are complete and merged with CI green on three OSes. `aud list` works end to end (JSON v1, schema-validated) and `scripts/differential.py` (+ `.ps1`/`.sh`) shows 20 scenarios identical to the Python reference. Core pieces in `aud-core`: `selection` (policy, scanner, `Glob`), `Dir`, `contracts`. Spec gaps SG-4 to SG-6 recorded.

How to check: `cargo test --workspace` then `powershell -File scripts/differential.ps1` (or `scripts/differential.sh`; needs Python 3.10+ and the reference checkout).

- Reference: Python `aud` @ `bb01db7`. Local checkout `S:\ZX\Programming\aud`; `.reference/aud` also cloned by `scripts/fetch-reference.ps1` (gitignored).
- Not yet wired into CI: the differential script (M8-1).
- Gotcha: the file-write tool turns double backslashes into single ones in file content; avoid backslash escapes in written source (see AI_LEDGER).

## Next action

Start **M2 — Safety core**: [docs/plan/milestones/M2-safety-core.md](docs/plan/milestones/M2-safety-core.md). Read `safety.py`, `plan.py`, `execution.py` and traps T-3, T-4, T-5, T-19, T-20, T-24 first.

Kickoff prompt for a fresh session: [prompts/kickoff.md](prompts/kickoff.md).

## Read order for a cold session

1. [AGENTS.md](AGENTS.md)
2. [docs/plan/README.md](docs/plan/README.md) (index) → `01-scope.md`, `02-architecture.md`
3. The active milestone file
4. [docs/plan/05-porting-traps.md](docs/plan/05-porting-traps.md) before porting any module

## Open decisions (none block M2)

| # | Question | Default if unanswered | Decided by |
| --- | --- | --- | --- |
| 1 | crates.io names (`aud` is probably taken) | `aud-core`, `aud-cli` on crates.io; binary stays `aud` | M9 |
| 2 | Pure-Rust media engine as stretch | Not started; FFmpeg engine only ([ADR-0001](docs/adr/0001-ffmpeg-subprocess-engine.md)) | after M8 |
| 3 | Binary distribution (`cargo-dist`) | Yes, GitHub Releases | M9 |

## Blockers

None.
