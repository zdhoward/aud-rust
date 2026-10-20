# AI assistance ledger

Evidence log for the case study. One row per event, written when it happens. Types: **Catch** (defect in AI output found by a named layer), **Override** (human redirected the AI, with reason), **Win** (non-obvious AI finding, with evidence), **Gap** (spec or tooling shortfall that cost time). Rules: [docs/plan/07-ai-workflow.md](plan/07-ai-workflow.md).

| Date | M | Type | What happened | Caught by / evidence | Link |
| --- | --- | --- | --- | --- | --- |
| 2026-10-20 | plan | Win | Planning pass flagged that Rust `fs::rename` replaces existing targets on every OS, which breaks the Python exclusive-publish guarantee; led to ADR-0003 | Reading `safety.py` against `std` docs | [ADR-0003](adr/0003-no-unsafe-exclusive-publish.md) |
| 2026-10-20 | M0 | Catch | First draft of the smoke crates failed `clippy -D warnings` (`single_match_else` in `main.rs`, `assert_is_empty` in the core test) and `cargo fmt --check` | `cargo clippy` / `cargo fmt` run before first commit; fixed before it reached a PR | [PR](https://github.com/zdhoward/aud-rust/pulls) |
| 2026-10-20 | M0 | Gap | `scripts/fetch-reference.ps1` documented `pwsh`, which is not installed here; script itself runs fine on Windows PowerShell 5.1 | Ran the script; usage comment corrected | n/a |
| 2026-10-20 | M0 | Catch | CI install script was committed from Windows without the exec bit, so `test` failed on ubuntu and macOS (`Permission denied`) while Windows passed | First CI run of PR #1; fixed with `git update-index --chmod=+x` and `bash` invocation | [PR #1](https://github.com/zdhoward/aud-rust/pull/1) |
| 2026-10-20 | M1 | Catch | Plan trap T-16 said `--pattern` uses `fnmatch` with Windows case folding; Python source calls `fnmatchcase` (case-sensitive everywhere). Plan trap T-2 said scan order is code point; on Windows `pathlib` sorts case-insensitively | Reading `aud.py` / running Python 3.13.1 before writing the Rust; recorded as SG-4, SG-5 | [06-spec-gaps](plan/06-spec-gaps.md) |
