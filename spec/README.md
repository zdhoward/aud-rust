# Behavior specification (frozen snapshot)

These documents are copied unmodified from the Python `aud` repository. They define the behavior this port reproduces. **Do not edit them.** Rust documentation lives in `docs/`.

- Source: https://github.com/zdhoward/aud
- Pinned commit: `bb01db7cad9795f37a3fbe14cc8c59da4d9b8eb0` (master after PRs #74–92)
- Snapshot date: 2026-10-20
- Also snapshotted from the same commit: `../conformance/v1/*` (corpus), `../schemas/v1.json`

Wording like "unreleased" in these files refers to the Python package state at the pin (version 2.0.2 plus unreleased work). It does not describe this repository.

| File | Contract |
| --- | --- |
| [ARCHITECTURE.md](ARCHITECTURE.md) | layers, invariants |
| [SAFE_PROCESSING.md](SAFE_PROCESSING.md) | selection, collisions, staged writes, cancellation |
| [FFMPEG_CONVERSION.md](FFMPEG_CONVERSION.md) | conversion, profiles, tags, artwork, limits |
| [AUDIO_CHAINS.md](AUDIO_CHAINS.md) | effects chain, canonical metadata |
| [NAMING_PROFILES.md](NAMING_PROFILES.md) | templates, portability, tree validation |
| [FILENAME_PARSING.md](FILENAME_PARSING.md) | explicit filename grammar |
| [ALBUM_PACKAGING.md](ALBUM_PACKAGING.md) | ZIP packages, delivery profiles |
| [CLI.md](CLI.md) | commands, config files, exit codes, progress |
| [JSON_CONTRACTS.md](JSON_CONTRACTS.md) | v1 envelopes and schema |
| [API.md](API.md) | Python API reference (use for operation semantics, not for Rust shape) |
| [SECURITY_REVIEW.md](SECURITY_REVIEW.md) | threat model and findings |
| [READINESS_REPORT.md](READINESS_REPORT.md) | rationale behind the contract |

## Re-snapshotting

Only deliberately, in a dedicated PR: copy the same files from a new Python commit, update the pin above and in `scripts/fetch-reference.ps1`, rerun conformance and the differential suite, record changes in `docs/plan/06-spec-gaps.md` and PROJECT_LOG.
