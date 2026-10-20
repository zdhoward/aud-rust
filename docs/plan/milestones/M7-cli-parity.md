# M7 — CLI parity and polish (v0.7.0)

**Goal:** the Rust CLI matches `spec/CLI.md` exactly where it is a contract, and is pleasant where it is not.

**Spec refs:** `spec/CLI.md` (entire), `spec/JSON_CONTRACTS.md`, `conformance/v1/cases.json` (`progress`, `error`).
**Python refs:** `aud/cli.py` (all), `tests/test_cli.py`, `tests/test_progress.py`.

## Tasks

1. **Audit** each command and flag against CLI.md; list gaps as TODO items. Verify help text covers every flag.
2. **Config loader hardening** (T-14, T-12): regular-file check, 1 MiB cap, BOM, duplicate keys, non-finite, overflow, unknown top-level fields, unknown effect types, invalid metadata types. One test per rejection reason, ported from `test_cli.py`.
3. **Progress (`--progress`)**: stderr lines for file start/finish, stdout unchanged including with `--json`; semantics in CLI.md ("counts cover terminal decisions"; album `staged` vs final `success`).
4. **Exit codes**: 0/1/2/130 table. Ctrl+C: `ctrlc` handler sets `CancelToken`; running FFmpeg is stopped; staging cleaned; exit 130; earlier committed files reported. Windows and POSIX tests (spawn the CLI, send CTRL_BREAK/SIGINT).
5. **JSON error documents** for every failure after parsing; argument-parser errors keep the legacy shape; `--json-version` requires `--json`; help/version stay plain text. Mirror `cli.run` ordering exactly.
6. **Legacy `aud DIRECTORY` count form** — decide: port (preferred, it is documented) or record a spec gap.
7. **Quality-of-life beyond Python (optional, flagged as additions, not parity):** shell completions (`clap_complete`), `--color` handling for human-facing errors, `-v/--verbose` tracing. Keep behind features that cannot alter contract output.
8. **Library docs**: `cargo doc` clean with `#![warn(missing_docs)]`; examples as doc tests for `Dir`, `Plan`, `execute`.

## Exit criteria

- [ ] All of `test_cli.py` ported (TODO ticked) and passing on three OSes.
- [ ] Every command: `--json --json-version 1` output validates; dry-run and execute paths covered.
- [ ] `cargo doc --no-deps` warning-free. CHANGELOG `0.7.0`; PROJECT_LOG; tag `v0.7.0`.
