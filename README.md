# aud-rust

A Rust rewrite of [`aud`](https://github.com/zdhoward/aud), a safety-first batch audio processor, built with AI assistance and verified against the original.

> **Status: planning complete, implementation not started.** See [ROADMAP.md](ROADMAP.md) and [TODO.md](TODO.md). Nothing below describes shipped behavior yet.

## What this project demonstrates

- **A disciplined port.** Behavior is defined by the original's published contract ([`spec/`](spec/)), its JSON Schema ([`schemas/v1.json`](schemas/v1.json)) and its portable conformance corpus ([`conformance/v1/`](conformance/v1/)), not by transliterating code.
- **Falsifiable equivalence.** A differential suite will run the Python original and this port on identical generated inputs and compare their JSON output, resulting file trees and decoded audio.
- **Rust where it earns its keep.** `#![forbid(unsafe_code)]`, RAII cleanup of staged files, a closed `Operation` enum with exhaustive dispatch, typed errors, and property and fuzz tests on the parsers.
- **An honest AI-assisted process.** Every PR states what the AI drafted, what a human decided and how it was verified. [`docs/AI_LEDGER.md`](docs/AI_LEDGER.md) records where the AI was wrong and what caught it.

## The planned tool

```
aud list masters --extensions wav flac --json
aud convert masters --extensions wav --format flac --profile flac-preserve --dry-run
aud effects masters --extensions wav --config effects.json --format flac --output-directory processed
aud rename masters --extensions wav --config naming.json --output-directory organized --copy
aud album masters --extensions wav --config album.json --target delivery/amuse.zip --service amuse
```

Existing files are never overwritten unless asked; writes are staged and verified before publication; every batch reports per-file results. FFmpeg and ffprobe are required for media work.

## Where to look

| | |
| --- | --- |
| Plan | [docs/plan/](docs/plan/README.md) |
| Decisions | [docs/adr/](docs/adr/README.md) |
| Behavior spec (from the Python original) | [spec/](spec/README.md) |
| Agent / contributor rules | [AGENTS.md](AGENTS.md) |
| Where we are | [HANDOFF.md](HANDOFF.md) |

## License

MIT. The behavior specification in `spec/` and the corpus in `conformance/` derive from the MIT-licensed [zdhoward/aud](https://github.com/zdhoward/aud).
