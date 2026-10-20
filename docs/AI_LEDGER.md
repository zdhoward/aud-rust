# AI assistance ledger

Evidence log for the case study. One row per event, written when it happens. Types: **Catch** (defect in AI output found by a named layer), **Override** (human redirected the AI, with reason), **Win** (non-obvious AI finding, with evidence), **Gap** (spec or tooling shortfall that cost time). Rules: [docs/plan/07-ai-workflow.md](plan/07-ai-workflow.md).

| Date | M | Type | What happened | Caught by / evidence | Link |
| --- | --- | --- | --- | --- | --- |
| 2026-10-20 | plan | Win | Planning pass flagged that Rust `fs::rename` replaces existing targets on every OS, which breaks the Python exclusive-publish guarantee; led to ADR-0003 | Reading `safety.py` against `std` docs | [ADR-0003](adr/0003-no-unsafe-exclusive-publish.md) |
