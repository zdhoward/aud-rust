# Scope and success criteria

## Goal

Re-implement `aud`'s v1 observable contract in Rust: library plus `aud` CLI. Demonstrate disciplined AI-assisted engineering: spec-first, test-first, differentially verified against the original, with an honest record of where the AI helped and erred.

Audience: hiring managers and engineers reading the repo. Every choice optimizes for *reviewability and evidence*, not feature count.

## In scope (1.0.0)

Everything the Python repo documents as contract, minus the exclusions below:

| Area | Spec |
| --- | --- |
| Selection: extensions, allow/deny lists, glob `--pattern`, link skipping, sorted scan | `spec/SAFE_PROCESSING.md`, `spec/ARCHITECTURE.md` |
| Plans, previews, per-file results, cancellation, continue-on-error | `spec/SAFE_PROCESSING.md` |
| Collision policies `error`/`skip`/`overwrite`/`auto-rename`; staged, exclusive publish | `spec/SAFE_PROCESSING.md` |
| File ops: copy, move, backup, zip; name ops (upper, lower, prepend, append, replace, replace-spaces, iterate) | `spec/API.md` |
| FFmpeg conversion: formats wav/raw/flac/mp3/ogg, 8/16/24/32-bit PCM, profiles, tags, artwork, limits | `spec/FFMPEG_CONVERSION.md` |
| Audio chain: gain, fade, pad, low/high-pass, invert-phase, one final encode | `spec/AUDIO_CHAINS.md` |
| Canonical metadata, naming profiles, tree validation, filename grammar | `spec/NAMING_PROFILES.md`, `spec/FILENAME_PARSING.md` |
| Album packaging, 8 delivery profiles, manifest, checksums | `spec/ALBUM_PACKAGING.md` |
| CLI: `list`, `inspect`, `convert`, `export`, `rename`, `effects`, `album`; config files; exit codes 0/1/2/130; progress | `spec/CLI.md` |
| JSON v1 envelopes validated against `schemas/v1.json`; the conformance corpus | `spec/JSON_CONTRACTS.md`, `conformance/v1/` |

## Out of scope

| Excluded | Why |
| --- | --- |
| pydub-era effects: `afx_normalize`, `afx_strip_silence`, `afx_watermark`, `afx_join`, `afx_prepend/append` | Python itself plans to delete pydub (ROADMAP v3.0.0). Not part of the v1 contract. Normalization is a candidate stretch effect. |
| Python fluent `Dir` convenience methods (`name_upper()`, `convert_to_mp3()` …) | Python-idiom API. Rust exposes the same operations through `Plan` + `Operation`. |
| Optional web UI (v2.3.0) | Separate exposure review needed; adds nothing to the port evidence. |
| Recursive scan, plugins, registry dispatch | Unimplemented in Python too. |
| Python 3.x packaging, PyPI | n/a |

Divergences from Python are allowed only when written up as an ADR or a [spec gap](06-spec-gaps.md) entry, with a reason.

## Success criteria for 1.0.0

Each is checkable:

1. All conformance corpus groups (`cases.json`, `media.json`, `filename.json`) pass in Rust CI on Linux, macOS, Windows.
2. Every document the CLI emits with `--json --json-version 1` validates against `schemas/v1.json`.
3. The differential suite runs Python `aud` @ the pinned SHA and Rust `aud` on the same generated inputs and finds no unexplained differences (explained ones are listed in `06-spec-gaps.md`).
4. Ports of the Python tests' behaviors exist for each module (tracked in `03-port-map.md`).
5. `cargo clippy -D warnings`, `cargo fmt --check`, `cargo deny check`, `cargo audit` clean. `#![forbid(unsafe_code)]` everywhere.
6. Line coverage ≥ 85% on `aud-core` (matches the Python gate).
7. Benchmarks against Python exist and are reported with method and hardware.
8. `docs/CASE_STUDY.md` tells the AI-assisted story using the ledger as evidence.

## Decisions already made

| Decision | Choice | Record |
| --- | --- | --- |
| Media engine | FFmpeg/ffprobe subprocess behind a `MediaEngine` trait, as Python does | [ADR-0001](../adr/0001-ffmpeg-subprocess-engine.md) |
| Repo shape | Cargo workspace: `aud-core` (lib) + `aud-cli` (bin `aud`) | [ADR-0002](../adr/0002-workspace-and-api-shape.md) |
| Unsafe code | Forbidden; exclusive publish via `hard_link` | [ADR-0003](../adr/0003-no-unsafe-exclusive-publish.md) |
| Non-UTF-8 file names | Typed error naming the path; documented divergence | [ADR-0004](../adr/0004-non-utf8-paths.md) |
| Verification | Conformance corpus + ported tests + differential run vs pinned Python | [ADR-0005](../adr/0005-verification-strategy.md) |
| Operations | Closed `enum Operation`, exhaustive `match` replaces Python `isinstance` dispatch | [ADR-0002](../adr/0002-workspace-and-api-shape.md) |
| Git flow | PR per slice, merge commits, annotated tags at milestone exit | `AGENTS.md` |
