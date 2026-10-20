# Verification strategy

Claim to defend: *the Rust port behaves like the Python original wherever the contract says so, and differs only where recorded.* Evidence comes in layers; each catches what the previous cannot.

| Layer | Catches | Lives in | Runs |
| --- | --- | --- | --- |
| 1. Unit + doc tests | local logic errors | `crates/*/src`, `tests/` | every PR |
| 2. Ported Python tests | behaviors the docs omit | `crates/aud-core/tests/` | every PR |
| 3. Conformance corpus | contract drift from the shared fixtures | `crates/aud-core/tests/conformance_*.rs` | every PR, 3 OSes |
| 4. Schema validation | malformed JSON documents | every CLI/JSON test also validates against `schemas/v1.json` | every PR |
| 5. Property tests (`proptest`) | edge cases in pure cores: template parsing, `path_key`, `auto-rename`, filename grammar, `from_tags` | next to the module | every PR |
| 6. Differential suite | places where Rust and Python disagree on identical input | `crates/aud-cli/tests/differential/` + `scripts/differential.*` | CI job + nightly (M8) |
| 7. Fuzzing (`cargo-fuzz`) | panics in parsers: config JSON, templates, filename patterns, ffprobe JSON | `fuzz/` | M8, short CI smoke |
| 8. Failure injection (fake `MediaEngine`) | staging/cleanup bugs on error and cancel | `aud-core/tests/` | every PR |

## Test audio

Generate in test code with `hound` (WAV): sine and ramp signals with known sample values, 16/24-bit, mono/stereo, ~0.1 s (and ~4 s where a fade or watermark needs time). FLAC/MP3 inputs come from running real FFmpeg once per test run into a temp dir. No committed binary fixtures except the one-pixel RGBA PNG the corpus defines.

## Differential suite (M8; scaffolding from M1)

Inputs: a generator creates a temp tree (synthetic audio, tagged files, Unicode and case-colliding names, spaces). For each scenario `S`:

1. Copy the tree twice. Run `python -m aud.cli … --json --json-version 1` on one, `aud … --json --json-version 1` on the other.
2. Normalize: replace the temp-root prefix with `$ROOT`, drop timings, drop encoder-version strings. Do not sort arrays or rewrite metadata.
3. Compare decoded JSON documents and exit codes.
4. Compare the resulting directory trees: names, file sizes for non-audio bytes, SHA-256 for byte-preserving outputs, and decoded-PCM hashes (via `ffmpeg -f s32le -`) for audio. Lossy encoded bytes are never compared.
5. Any diff fails the job unless the scenario ID is listed in `differential/allowed-diffs.toml` with a link to a `06-spec-gaps.md` entry.

Python side: CI checks out `zdhoward/aud` at the pinned SHA (recorded in `spec/README.md`), `pip install .`, installs FFmpeg. Locally: `scripts/fetch-reference.ps1`, then a venv.

Scenarios mirror the CLI commands × overwrite policies × continue-on-error × dry-run, plus each album service.

## Performance claims

Benchmark only what the contract permits both sides to do: `list` on 10k files, `convert` batch of N synthetic WAVs to FLAC (FFmpeg-bound, so expect parity; report it honestly), `rename` dry-run planning over 10k files, `album` dry-run. Record machine, OS, FFmpeg version, Python version, command lines, and raw numbers in `docs/BENCHMARKS.md`. Report process-startup time separately. Do not claim a speedup the data does not show.

## CI jobs (built in M0, extended per milestone)

| Job | Content |
| --- | --- |
| `lint` | `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings` |
| `test` | matrix `ubuntu`, `macos`, `windows`; installs FFmpeg (apt / brew / choco); `cargo test --workspace` |
| `coverage` | `cargo llvm-cov`, fail under 85% for `aud-core` |
| `supply-chain` | `cargo deny check`, `cargo audit` |
| `msrv` | build with the declared `rust-version` |
| `differential` | M8: pinned Python checkout + comparison harness |
| `fuzz-smoke` | M8: 60 s per target |

FFmpeg versions differ across runners; assert on contract properties (sample values, tags, settings), never on encoder bytes (`conformance/v1/README.md`).
