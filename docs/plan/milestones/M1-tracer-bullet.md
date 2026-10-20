# M1 — Tracer bullet (v0.1.0)

**Goal:** one thin behavior through *every* layer: `aud list DIR --extensions wav flac --json --json-version 1` produces a schema-valid envelope identical to Python's. Proves the workspace, selection logic, contracts, CLI, schema validation, and the differential harness skeleton before any hard work starts.

**Spec refs:** `spec/CLI.md` (list, selection), `spec/JSON_CONTRACTS.md`, `spec/SAFE_PROCESSING.md` (scanning, links), `schemas/v1.json`, `conformance/v1/cases.json` (`progress`/`error` fixtures as schema inputs).
**Python refs:** `aud/aud.py` (`Dir`: `config_set_*`, `include`, selection building), `aud/core/models.py`, `aud/core/selection/*`, `aud/contracts.py`, `aud/cli.py` (`_parser`, `list` path, `_emit`), `tests/test_dir_selection.py`, `test_dir_config.py`, `test_dir_api.py`, `test_contracts.py`.

## Tasks

1. `AudioFile`, `SelectionPolicy` trait or enum (`Extension`, `Allowlist`, `Denylist`, `Composite`, glob narrowing). Selection rule: `(valid_extension AND NOT denylisted) OR allowlisted`; no extensions ⇒ only allowlisted files. Re-read `Dir` to confirm how `pattern` narrows *after* the allowlist override.
2. `DirectoryScanner`: single directory, sorted by code point (T-2), skip links unless `follow_symlinks` (T-6), regular files only, non-UTF-8 name ⇒ typed error (ADR-0004).
3. `fnmatch` matcher (T-16) with Windows-only case folding. Ported tests from Python's `test_dir_selection.py`.
4. `contracts.rs`: `json_document(kind, payload)`, `KINDS`, embedded schema, `json_schema()`. Reject non-finite numbers.
5. `aud-cli`: clap definition for `list` and global flags used by it (`--extensions`, `--pattern`, `--json`, `--json-version`, `--follow-symlinks` if Python has it). Human output = indented JSON with escaped strings (see CLI.md). Exit codes per CLI.md. Error documents per `JSON_CONTRACTS.md` (`error` kind with `succeeded:false`, `error`, `exit_code`, `result`). `aud DIRECTORY` legacy count form: read `cli.py` and decide in the PR (port or record as spec gap).
6. Test helper crate module (`tests/common`): temp tree builder, `$ROOT` normalizer, schema validator using the `jsonschema` crate (Draft 2020-12).
7. Differential harness skeleton: `scripts/differential.ps1` + `scripts/differential.sh` run Python and Rust `list` on a generated tree and diff normalized JSON. Wired but not yet in CI.
8. Validate each `progress` and `error` fixture in `cases.json` against the embedded schema (these are document fixtures, no engine yet).

## Traps

T-2, T-6, T-14 (config comes later but the loader belongs here if `list` takes `--config`; check), T-16, T-22.

## Exit criteria

- [ ] `aud list` JSON output equals Python's for ≥ 10 scenarios (extensions, allow/deny, pattern, links, Unicode and case-colliding names, empty directory, missing directory error) in the differential script.
- [ ] Every emitted document passes schema validation in tests.
- [ ] All ported selection/contract tests green; TODO.md test-file checkboxes ticked for those files.
- [ ] CI green on three OSes. CHANGELOG `0.1.0`; PROJECT_LOG milestone entry with metrics; tag `v0.1.0`.

## Why this slice first

If the harness, schema validation, or CLI contract is wrong, finding out here costs an afternoon; finding out after M5 costs a week.
