# Contributing

`aud-rust` is a Rust port of [`zdhoward/aud`](https://github.com/zdhoward/aud). The authoritative rules for contributors, human or AI, are in [AGENTS.md](AGENTS.md); this is the short version.

## Where behavior comes from

- Behavior is defined by [`spec/`](spec/) (frozen), [`conformance/v1/`](conformance/v1/) and [`schemas/v1.json`](schemas/v1.json). Do not edit `spec/` or expected values in `conformance/`.
- Spec contradictions go in [`docs/plan/06-spec-gaps.md`](docs/plan/06-spec-gaps.md). A change to the plan gets an ADR in [`docs/adr/`](docs/adr/) first.

## Workflow

1. Branch per slice (`feat/m2-safety-core`), PR to `main`, merge commit. No direct pushes to `main`.
2. Write the failing test first, then the code.
3. Before committing, all of these must pass:
   ```
   cargo fmt --check
   cargo clippy --workspace --all-targets -- -D warnings
   cargo test --workspace
   ```
4. One logical change per commit; imperative subject; the body says why.

## Code rules

- `#![forbid(unsafe_code)]` in every crate. Needing `unsafe` requires an ADR first.
- Library errors are typed `thiserror` enums; `anyhow` only in the CLI binary.
- No `unwrap`/`expect` outside tests, except for proven invariants with a comment stating the proof.
- Tests generate synthetic audio in-test. FFmpeg and ffprobe must be on `PATH` for media tests.

## AI assistance

Every PR body has an **AI assistance** section (see the PR template). When the AI was wrong or a human overrode it, add an entry to [`docs/AI_LEDGER.md`](docs/AI_LEDGER.md).

## Security

See [SECURITY.md](SECURITY.md).
