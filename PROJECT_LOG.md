# Project log

Short audit trail: key actions, decisions, reasons. Newest first. Milestone entries include the metrics listed in `docs/plan/07-ai-workflow.md`.

## 2026-10-20 — M0 bootstrap (PR #1, awaiting merge)

- Tool versions: rustc/cargo 1.99.1 (rustup 1.29.1, stable-x86_64-pc-windows-msvc, MSVC build tools already present), cargo-deny 0.20.2, cargo-audit 0.22.2, cargo-llvm-cov 0.9.1, FFmpeg/ffprobe 8.0.1.
- **Decision:** `rust-version = 1.99` and `rust-toolchain.toml` = 1.99.1, per the plan rule "the stable you install in M0". The `msrv` CI job reads the value from `Cargo.toml`.
- **Decision:** coverage gate (aud-core ≥ 85%) is on from day one; the smoke crate is at 100%.
- **Decision:** branch protection requires `lint`, the three `test` jobs and `supply-chain`; PR required, 0 approvals (solo repo), admins not enforced, no force-push. `coverage` and `msrv` run but are not required yet.
- Deviation: first CI run failed on ubuntu/macOS because the FFmpeg install script lacked the exec bit (committed from Windows). Fixed; recorded in the AI ledger.
- Repo settings applied: description, topics, private vulnerability reporting.
- Open at time of writing: merge PR #1, then CHANGELOG `0.0.1`, final milestone-exit log entry, tag `v0.0.1`, and CI green on `main`.

## 2026-10-20 — Planning session

- Surveyed Python `aud` @ `bb01db7`: ~4.5k lines of source across 50 modules, 119 tracked files, 30 test files, conformance corpus v1 (cases/media/filename), JSON schema v1, 12 contract docs. Python itself prepares for a port (ROADMAP: "reference implementation for a language port").
- **Decision:** port the observable contract (plans, safe writes, FFmpeg conversion, chains, naming, filename parsing, album packaging, CLI, JSON v1). Exclude pydub-era effects and the fluent `Dir` convenience API. Reason: Python is deleting pydub; the rest is Python-idiom API, not contract.
- **Decision:** FFmpeg subprocess engine behind a trait (ADR-0001). Reason: parity is checkable; no codec work adds portfolio value.
- **Decision:** `forbid(unsafe_code)`, hard-link exclusive publish (ADR-0003). Reason: Rust `fs::rename` replaces on all platforms; a zero-unsafe claim is worth a documented Windows divergence (SG-3).
- **Decision:** differential testing against the pinned Python as the primary equivalence evidence (ADR-0005). Reason: the corpus authors state it is representative, not a full port gate.
- Vendored `spec/`, `conformance/v1/`, `schemas/v1.json` as a frozen snapshot. Left out the 26 MB `mock_assets/`; tests synthesize audio.
- Rust not installed on the dev machine; M0 task 1.
- Created public repo `zdhoward/aud-rust` from the planning tree.
