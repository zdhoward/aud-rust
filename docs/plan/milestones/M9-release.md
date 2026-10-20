# M9 — Release and case study (v1.0.0)

**Goal:** a repository a stranger can understand in five minutes, trust in thirty, and download in one.

## Tasks

1. **README final form.** One-paragraph pitch; demo (terminal recording via `asciinema`/`vhs`, committed as a GIF or SVG); install (`cargo install`, release binaries); quick tour of each command; "Verified against the original" section with the differential and conformance numbers; link to the case study.
2. **Docs.** `docs/ARCHITECTURE.md` (Rust-specific, evolved from `plan/02`), CLI reference generated from clap, library guide with doc-tested examples. Keep `spec/` as the behavior reference and say so.
3. **Release engineering.** `cargo-dist` (or equivalent) building binaries for Linux x86_64, macOS arm64/x86_64, Windows x86_64; checksums; GitHub Release notes from CHANGELOG. Publish to crates.io only if the crate names are available and the user confirms; otherwise state "install from source or release binaries".
4. **Case study.** `docs/CASE_STUDY.md` per [07-ai-workflow.md](07-ai-workflow.md): built from `AI_LEDGER.md` and PROJECT_LOG metrics; include the real misses. Link PRs and commits as evidence.
5. **Final audit.** Clean-clone test: fresh checkout builds and passes on all three OSes from the README instructions alone. Spell-check docs. Remove stale TODOs. Confirm no secrets or local paths (`S:\…`) remain in published docs outside `AGENTS.md` environment notes.
6. **Repo presentation.** Description, topics, social preview image, pinned on the user's GitHub profile (user action; list it in the handoff).
7. **Tag `v1.0.0`** with release notes; update ROADMAP to "post-1.0 ideas" (pure-Rust read-only probe, recursive scan, normalization effect, `ffmpeg` version policy).

## Exit criteria

- [ ] Every success criterion in [01-scope.md](../01-scope.md) is ticked with a link to its evidence.
- [ ] Clean-clone check recorded in PROJECT_LOG.
- [ ] Release published with binaries and checksums; CHANGELOG `1.0.0`; tag `v1.0.0`.
