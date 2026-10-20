# aud-rust — agent instructions

A Rust port of [`aud`](https://github.com/zdhoward/aud) (Python batch audio processor), built as a public portfolio project. The port reproduces the Python product's **observable contract**, not its code shape. The AI-assisted process is itself a deliverable: keep it visible and honest.

Start here: [HANDOFF.md](HANDOFF.md) (state, next action) → [TODO.md](TODO.md) (task list) → the active file in [docs/plan/milestones/](docs/plan/milestones/).

## Source of truth

- **Behavior** → [spec/](spec/) (Python contract docs, frozen snapshot of `zdhoward/aud` @ `bb01db7`) plus [conformance/v1/](conformance/v1/) and [schemas/v1.json](schemas/v1.json). When the spec is silent, read the Python source: `S:\ZX\Programming\aud` locally, otherwise `scripts/fetch-reference.ps1` (pinned clone into the gitignored `.reference/aud`).
- **Plan** → [docs/plan/](docs/plan/README.md). Decisions → [docs/adr/](docs/adr/). A decision that changes the plan gets an ADR first, then the plan edit.
- Rust-side docs describe Rust. Never edit `spec/`; a contradiction found in it goes in [docs/plan/06-spec-gaps.md](docs/plan/06-spec-gaps.md).

## Loop for every task

1. Read the milestone file's *Spec refs* and *Traps*, and [docs/plan/05-porting-traps.md](docs/plan/05-porting-traps.md) before touching a new module.
2. Name the Python module being ported and read it in full.
3. Red: write the failing test first (conformance case, ported Python test, or property test). `tdd` skill for the loop.
4. Green, then refactor. Rust idiom beats transliteration; the contract decides, the Python structure does not.
5. Definition of done for a task: `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace` all green, TODO.md ticked, and any surprise recorded.

Unsure of a design question: `grilling`, then `prototype`. Broken or failing: `diagnosing-bugs`. Before every PR: `code-review`, then `pr` for the body.

## Conventions

- Branch per slice (`feat/m2-safety-core`), PR to `main`, merge commit. No direct pushes to `main`. One logical change per commit, imperative subject, body states why.
- `#![forbid(unsafe_code)]` in every crate. Needing `unsafe` means writing an ADR first.
- Library errors: typed `thiserror` enums. `anyhow` only in the CLI binary.
- No `unwrap`/`expect` outside tests, except for proven invariants with a comment stating the proof.
- Tests use synthetic audio generated in-test. The Python repo's 26 MB `mock_assets/` stay out of this repo.
- Versions: 0.N.0 at each milestone exit, 1.0.0 at M9. Tag (`vX.Y.Z`, annotated) only on a merged milestone-exit commit, with CHANGELOG, tag and PROJECT_LOG agreeing.

## Transparency duties (portfolio requirement)

- Every PR body carries an **AI assistance** section: what the AI drafted, what the human decided, what was verified and how. Template: [.github/PULL_REQUEST_TEMPLATE.md](.github/PULL_REQUEST_TEMPLATE.md).
- Append to [docs/AI_LEDGER.md](docs/AI_LEDGER.md) whenever the AI was wrong, a human overrode it, or a tool or test caught a defect. Truthful entries only: misses are the evidence that review works.
- Append to [PROJECT_LOG.md](PROJECT_LOG.md) on each milestone exit and each important decision.

## Session end

When the user says "wrap up": reconcile TODO/ROADMAP/HANDOFF with reality, merge ready PRs, tag if a milestone exited, and leave HANDOFF.md so a cold session can continue.

## Environment

Windows 11 primary; CI covers Linux, macOS, Windows. FFmpeg and ffprobe must be on PATH (installed locally: 8.0.1). Rust toolchain installation is task M0-1 in TODO.md.
