# Project log

Short audit trail: key actions, decisions, reasons. Newest first. Milestone entries include the metrics listed in `docs/plan/07-ai-workflow.md`.

## 2026-10-20 — Planning session

- Surveyed Python `aud` @ `bb01db7`: ~4.5k lines of source across 50 modules, 119 tracked files, 30 test files, conformance corpus v1 (cases/media/filename), JSON schema v1, 12 contract docs. Python itself prepares for a port (ROADMAP: "reference implementation for a language port").
- **Decision:** port the observable contract (plans, safe writes, FFmpeg conversion, chains, naming, filename parsing, album packaging, CLI, JSON v1). Exclude pydub-era effects and the fluent `Dir` convenience API. Reason: Python is deleting pydub; the rest is Python-idiom API, not contract.
- **Decision:** FFmpeg subprocess engine behind a trait (ADR-0001). Reason: parity is checkable; no codec work adds portfolio value.
- **Decision:** `forbid(unsafe_code)`, hard-link exclusive publish (ADR-0003). Reason: Rust `fs::rename` replaces on all platforms; a zero-unsafe claim is worth a documented Windows divergence (SG-3).
- **Decision:** differential testing against the pinned Python as the primary equivalence evidence (ADR-0005). Reason: the corpus authors state it is representative, not a full port gate.
- Vendored `spec/`, `conformance/v1/`, `schemas/v1.json` as a frozen snapshot. Left out the 26 MB `mock_assets/`; tests synthesize audio.
- Rust not installed on the dev machine; M0 task 1.
- Created public repo `zdhoward/aud-rust` from the planning tree.
