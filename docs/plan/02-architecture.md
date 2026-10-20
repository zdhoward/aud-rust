# Architecture

Same layering as Python (`spec/ARCHITECTURE.md`): **intent** (operations) is separate from **execution** (adapters). Invariants carry over unchanged:

- Operations never perform I/O. Adapters hold no intent.
- `preview` has no side effects on disk or on operation state.
- Files are processed in sorted order. Naming is deterministic.
- Media work requires `ffmpeg` and `ffprobe` on PATH.

## Workspace

```
aud-rust/
├── Cargo.toml                 # [workspace], shared lints, edition, rust-version
├── crates/
│   ├── aud-core/              # library: all behavior, no terminal I/O
│   │   ├── src/
│   │   │   ├── lib.rs         # #![forbid(unsafe_code)], public re-exports
│   │   │   ├── error.rs       # AudError (thiserror), kinds mirror Python exceptions
│   │   │   ├── model.rs       # AudioFile
│   │   │   ├── selection/     # policy.rs, extensions.rs, listing.rs (allow/deny/glob), scanner.rs
│   │   │   ├── safety.rs      # path_key, link detection, Destination, collision resolve, staged_write
│   │   │   ├── plan.rs        # Plan, Operation enum, preview types
│   │   │   ├── operations/    # names.rs, files.rs, archive.rs, convert.rs, effects.rs, chain.rs, naming.rs
│   │   │   ├── execution.rs   # resolve(plan) -> Vec<Step>; execute; PlanPreview
│   │   │   ├── adapters/      # filesystem.rs, convert.rs (chain + conversion via MediaEngine)
│   │   │   ├── media/         # engine.rs (trait MediaEngine), ffmpeg.rs (ProcessRunner, probe, encode, verify), limits.rs
│   │   │   ├── metadata.rs    # AudioMetadata::from_tags
│   │   │   ├── naming.rs      # NamingProfile, template parser, validate_naming_tree
│   │   │   ├── filename.rs    # FilenamePattern
│   │   │   ├── effects.rs     # validate_effects, compile_effects (-> filter strings)
│   │   │   ├── packaging.rs   # AlbumPackage, manifest, checksums, staged ZIP
│   │   │   ├── delivery.rs    # 8 provider profiles (static data)
│   │   │   ├── progress.rs    # ProgressEvent, ProgressReporter
│   │   │   ├── results.rs     # FileResult, BatchResult, Status, CancelToken
│   │   │   └── contracts.rs   # json_document(), embedded schema, KINDS
│   │   └── tests/             # integration tests + conformance runners
│   └── aud-cli/               # binary `aud`: clap, config loading, formatting, exit codes, ctrl-c
├── conformance/v1/            # shared corpus (read by tests)
├── schemas/v1.json            # embedded via include_str! (copy lives in aud-core at build, see M1)
├── spec/                      # frozen Python contract docs
├── docs/ prompts/ scripts/
└── xtask/ (optional, M8)      # differential and bench drivers if shell scripts get unwieldy
```

`aud-core` embeds the schema with `include_str!("../../../schemas/v1.json")`. Keep a single copy of the file.

## Key design choices

**Operations as a closed enum.**
```rust
pub enum Operation {
    Name(NameOp),              // Uppercase, Lowercase, Prepend, Append, Replace, ReplaceSpaces, Iterate
    File(FileOp),              // Copy, Move, Backup
    Zip(Zip),
    Convert(ConvertFormat),    // includes mono/stereo as channel settings
    Chain(AudioChain),
    NameWithProfile(NameWithProfile),
}
```
`resolve()` is one exhaustive `match`; adding a variant is a compile error until handled. Python's "unknown operation raises TypeError" disappears by construction. Plugin registration stays a non-goal.

**Statefulness.** Python's `Iterate` carries a counter, so `resolve` deep-copies operations. In Rust, `Operation: Clone` and `resolve` clones; preview and execute never share counter state.

**Engine seam.** `trait MediaEngine { fn probe(..); fn encode(..); fn decode_verify(..); }`. `FfmpegEngine` is the production implementation. Unit tests use a fake engine to cover orchestration (staging, collision, cancellation, recovery) without spawning processes. A small set of integration tests runs the real engine. This is the one abstraction the port adds; it earns its place by making failure injection deterministic (the conformance `recovery` cases need it).

**Errors.** `AudError` variants: `Filename`, `File`, `AudioFx`, `Convert`, `Export`, `Cancelled`, `Usage`, plus `#[source]` chaining (Python's `raise … from e`). CLI maps kinds to exit codes: `Usage`/config → 2, execution failure → 1, `Cancelled` → 130.

**Cancellation.** `CancelToken(Arc<AtomicBool>)`, cheap to clone, checked between files and polled inside `ProcessRunner`. The CLI wires Ctrl+C with the `ctrlc` crate.

**Progress.** `Arc<dyn Fn(&ProgressEvent) + Send + Sync>` inside `ProgressReporter`. A panicking or failing callback disables further events and logs a warning (Python behavior), via `catch_unwind` — `catch_unwind` is safe code.

**Serialization.** Result, preview, progress and document types derive `serde::Serialize`; field names match the schema exactly. Comparison is on decoded JSON, never text. Output uses `serde_json` without NaN support (non-finite floats must fail).

**Logging.** `tracing` or `log` facade in the library, no subscriber installed by the library (Python: `NullHandler`). Choose `tracing`; the CLI installs a subscriber only with `-v`.

## Dependencies (starting set; each addition needs a reason in the PR)

| Need | Crate | Note |
| --- | --- | --- |
| CLI | `clap` (derive) | |
| JSON | `serde`, `serde_json` | duplicate-key rejection needs a custom check (trap T-14) |
| Errors | `thiserror` (core), `anyhow` (cli) | |
| Unicode | `unicode-normalization`, `caseless`, `unicode-general-category` | NFC/NFKD, casefold, `Cc/Cf/Cs` |
| Same-file detection | `same-file` | safe `samefile` (T-19) |
| Regex (library allow/deny only) | `regex` | linear-time, SG-2 |
| Hash / zip | `sha2`, `zip` | |
| Ctrl+C | `ctrlc` | |
| Process groups (POSIX) | `nix` (safe `killpg`) | `cfg(unix)` only; keeps `forbid(unsafe_code)` |
| Logging | `tracing` | |
| Dev | `tempfile`, `proptest`, `assert_cmd`, `predicates`, `jsonschema`, `hound` (WAV synth), `criterion` | |
| Supply chain | `cargo-deny`, `cargo-audit`, `cargo-llvm-cov` (tools, not deps) | |

Rejected: `glob`/`globset` (Python `fnmatch` semantics differ at the edges; write the ~60-line matcher and test it), `walkdir` (scan is a single non-recursive directory read), async runtimes (process polling is simple and matches Python).

## Rust baseline

Edition 2024, `rust-version` = the stable you install in M0, never lower than 1.85. APIs relied on: `f64::round_ties_even` (1.77), `std::path::absolute` (1.79), `CommandExt::process_group` (1.64). Pin `rust-toolchain.toml` to a stable version, and bump deliberately.

Workspace lints (in root `Cargo.toml`): `unsafe_code = "forbid"`, `clippy::all = "deny"`, `clippy::pedantic = "warn"` with named allows when justified, `missing_docs = "warn"` for `aud-core`.
