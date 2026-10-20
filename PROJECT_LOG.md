# Project log

Short audit trail: key actions, decisions, reasons. Newest first. Milestone entries include the metrics listed in `docs/plan/07-ai-workflow.md`.

## 2026-10-20 — M1 exit (v0.1.0)

- Delivered: `aud list` end to end, `Dir` selection (extension/allow/deny/pattern), scanner, `fnmatchcase`-compatible glob, v1 envelope + embedded schema, legacy count form, differential harness (20 scenarios identical to Python 3.13.1 on Windows).
- Metrics: 3 PRs (#4, #5 and this exit PR); ~1.4k Rust lines (src + tests) vs ~0.6k Python lines read (cli.py, aud.py selection half, selection/*, contracts, models); 51 tests; Python tests ported: test_dir_selection, test_dir_config, test_dir_api in full, list parts of test_cli and test_contracts (the rest of those files belong to later milestones, so their TODO boxes stay open).
- Defects by layer: plan (2 wrong traps, caught by reading source + running Python), compiler/clippy (several lints), differential (JSON key order, my normalizer), CI (cargo-deny Zlib licence only visible on Linux/macOS; exec bit in M0), code-review (magic number, over-broad masking).
- **Decisions:** legacy count form ported in M1 (spec promises it; 3 lines). Allow/deny regex stay library-only (Python CLI has no flag), covered by unit tests. Zlib added to the licence allow list. Case-collision scenario skips on case-insensitive filesystems. Windows sort is case-insensitive like `pathlib` (SG-4).
- Deferred: Ctrl+C / exit 130 and `--output-bytes` default to M7 / M3; branch protection did not need changes.
- Wall-clock: roughly one session.

## 2026-10-20 — M0 exit (v0.0.1)

- Tool versions: rustc/cargo 1.99.1 (rustup 1.29.1, stable-x86_64-pc-windows-msvc, MSVC build tools already present), cargo-deny 0.20.2, cargo-audit 0.22.2, cargo-llvm-cov 0.9.1, FFmpeg/ffprobe 8.0.1.
- **Decision:** `rust-version = 1.99` and `rust-toolchain.toml` = 1.99.1, per the plan rule "the stable you install in M0". The `msrv` CI job reads the value from `Cargo.toml`.
- **Decision:** coverage gate (aud-core ≥ 85%) is on from day one; the smoke crate is at 100%.
- **Decision:** branch protection requires `lint`, the three `test` jobs and `supply-chain`; PR required, 0 approvals (solo repo), admins not enforced, no force-push. `coverage` and `msrv` run but are not required yet.
- Deviation: first CI run failed on ubuntu/macOS because the FFmpeg install script lacked the exec bit (committed from Windows). Fixed; recorded in the AI ledger.
- Repo settings applied: description, topics, private vulnerability reporting.
- PR #1 merged after all CI jobs passed; CHANGELOG 0.0.1 and tag `v0.0.1` added in a follow-up PR. M0 complete.

## 2026-10-20 — Planning session

- Surveyed Python `aud` @ `bb01db7`: ~4.5k lines of source across 50 modules, 119 tracked files, 30 test files, conformance corpus v1 (cases/media/filename), JSON schema v1, 12 contract docs. Python itself prepares for a port (ROADMAP: "reference implementation for a language port").
- **Decision:** port the observable contract (plans, safe writes, FFmpeg conversion, chains, naming, filename parsing, album packaging, CLI, JSON v1). Exclude pydub-era effects and the fluent `Dir` convenience API. Reason: Python is deleting pydub; the rest is Python-idiom API, not contract.
- **Decision:** FFmpeg subprocess engine behind a trait (ADR-0001). Reason: parity is checkable; no codec work adds portfolio value.
- **Decision:** `forbid(unsafe_code)`, hard-link exclusive publish (ADR-0003). Reason: Rust `fs::rename` replaces on all platforms; a zero-unsafe claim is worth a documented Windows divergence (SG-3).
- **Decision:** differential testing against the pinned Python as the primary equivalence evidence (ADR-0005). Reason: the corpus authors state it is representative, not a full port gate.
- Vendored `spec/`, `conformance/v1/`, `schemas/v1.json` as a frozen snapshot. Left out the 26 MB `mock_assets/`; tests synthesize audio.
- Rust not installed on the dev machine; M0 task 1.
- Created public repo `zdhoward/aud-rust` from the planning tree.
