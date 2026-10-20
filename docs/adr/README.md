# Architecture decision records

Short, dated, immutable once Accepted. Supersede with a new ADR rather than editing. Status: Proposed → Accepted → Superseded.

| ADR | Decision | Status |
| --- | --- | --- |
| [0001](0001-ffmpeg-subprocess-engine.md) | Media engine is FFmpeg/ffprobe subprocesses behind a trait | Accepted |
| [0002](0002-workspace-and-api-shape.md) | Two-crate workspace; closed `Operation` enum; thin `Dir` | Accepted |
| [0003](0003-no-unsafe-exclusive-publish.md) | `forbid(unsafe_code)`; exclusive publish via hard link | Accepted |
| [0004](0004-non-utf8-paths.md) | Non-UTF-8 file names are a typed error | Accepted |
| [0005](0005-verification-strategy.md) | Conformance + ported tests + differential run against pinned Python | Accepted |

Template: Context · Decision · Consequences · Alternatives considered.
