# M8 — Differential suite, hardening, benchmarks (v0.8.0)

**Goal:** turn "it behaves like Python" from a belief into a CI-enforced, falsifiable claim; harden against hostile input; produce honest performance numbers.

**Plan refs:** [04-verification.md](../04-verification.md). **Spec refs:** `spec/SECURITY_REVIEW.md` (threat model to re-apply to Rust).

## Tasks

1. **Differential CI job.** Check out `zdhoward/aud` at the pinned SHA, install, run the full scenario matrix from 04-verification.md. Failure output: scenario ID, both documents, tree diff. `differential/allowed-diffs.toml` entries must cite an SG.
2. **Scenario generator.** Deterministic seeds; Unicode, case collisions, spaces, long names, empty files, corrupt audio, zero-length audio, link files (skipped on Windows if unprivileged), read-only targets. Seed printed on failure for replay.
3. **Fuzzing.** `cargo-fuzz` targets: config JSON loader, template parser, filename pattern parser, ffprobe JSON parser, `from_tags`, `fnmatch`. Run each for at least 10 minutes locally once, commit minimized corpora, 60 s smoke in CI. Triage every crash into a regression test.
4. **Panic audit.** `grep` for `unwrap|expect|panic!|unreachable!|\[.*\]` indexing in `aud-core`; justify or remove. Clippy `unwrap_used`, `expect_used`, `indexing_slicing` as warnings in `aud-core` non-test code.
5. **Rust security review.** Re-run the Python `SECURITY_REVIEW.md` threat list against the Rust code: path traversal in archive members and naming, symlink/reparse handling, argv injection (filenames starting with `-` must be passed as `./name` or after `--`; check what Python does), process env, resource limits, TOCTOU windows. Write `docs/SECURITY_REVIEW.md` with findings, fixes and residual risk. Fix high/critical issues before the milestone exits.
6. **Benchmarks** (`criterion` for in-process planning; `hyperfine` for end-to-end CLI). Methodology in 04-verification.md. Output `docs/BENCHMARKS.md` with raw numbers and caveats.
7. **Coverage.** Raise to ≥ 85% on `aud-core`; investigate uncovered branches rather than adding vacuous tests.

## Exit criteria

- [ ] Differential job green on `main` with every allowed diff justified.
- [ ] Fuzz corpora committed; no open crashes.
- [ ] `docs/SECURITY_REVIEW.md` and `docs/BENCHMARKS.md` published.
- [ ] CHANGELOG `0.8.0`; PROJECT_LOG with defects-caught-by-layer table; tag `v0.8.0`.
