# AI assistance ledger

Evidence log for the case study. One row per event, written when it happens. Types: **Catch** (defect in AI output found by a named layer), **Override** (human redirected the AI, with reason), **Win** (non-obvious AI finding, with evidence), **Gap** (spec or tooling shortfall that cost time). Rules: [docs/plan/07-ai-workflow.md](plan/07-ai-workflow.md).

| Date | M | Type | What happened | Caught by / evidence | Link |
| --- | --- | --- | --- | --- | --- |
| 2026-10-20 | plan | Win | Planning pass flagged that Rust `fs::rename` replaces existing targets on every OS, which breaks the Python exclusive-publish guarantee; led to ADR-0003 | Reading `safety.py` against `std` docs | [ADR-0003](adr/0003-no-unsafe-exclusive-publish.md) |
| 2026-10-20 | M0 | Catch | First draft of the smoke crates failed `clippy -D warnings` (`single_match_else` in `main.rs`, `assert_is_empty` in the core test) and `cargo fmt --check` | `cargo clippy` / `cargo fmt` run before first commit; fixed before it reached a PR | [PR](https://github.com/zdhoward/aud-rust/pulls) |
| 2026-10-20 | M0 | Gap | `scripts/fetch-reference.ps1` documented `pwsh`, which is not installed here; script itself runs fine on Windows PowerShell 5.1 | Ran the script; usage comment corrected | n/a |
