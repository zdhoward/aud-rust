# M5 — Naming profiles and filename parsing (v0.5.0)

**Goal:** metadata-driven destination trees with portable validation; explicit filename grammar; CLI `rename`.

**Spec refs:** `spec/NAMING_PROFILES.md`, `spec/FILENAME_PARSING.md`, `spec/CLI.md` (`naming.json`, `filename_pattern`), `conformance/v1/cases.json` (`naming`), `conformance/v1/filename.json` (`valid`, `invalid`).
**Python refs:** `core/naming.py` (read fully; it is the densest file for traps), `core/filename.py`, `core/operations/naming.py`, `aud.py` (`plan_naming`, `name_profile`, `parse_filenames`); tests `test_naming_profiles.py`, `test_filename_parsing.py`.

## Tasks

1. **Template parser** (T-1): literal text, `{field}`, `{field:spec}`, `{{`/`}}`, conversion rejection. Share between naming and filename pattern; the two have different spec rules.
2. **`NamingProfile`** with `validate()` (every check in `naming.py` lines 51–92) and `render()` (T-7, T-8, T-9, T-18): NFC, optional NFKD→ASCII, case, illegal char replacement, whitespace collapse, trim ` .`, device-name guard, 255-unit limits in UTF-8 and UTF-16, extension rules, missing-field policy `error|skip`, defaults.
3. **`validate_naming_tree`** — port with filesystem checks: root aliasing, file-vs-directory collisions, case/Unicode directory collisions, existing alias detection (T-3, T-19).
4. **`NameWithProfile` operation**: copy vs move; skipped entries (`missing: skip`) carry through preview and execution.
5. **`FilenamePattern`**: left-to-right parse with first-separator behavior, bounded inputs, numeric field rules (`isascii` + digits, ≤ 9 digits, width, positive), number ≤ total.
6. **Canonical metadata probe path** for files with no supplied metadata (uses M3 probe, M4 `from_tags`); supplied-metadata path from config; `metadata` and `filename_pattern` mutually exclusive.
7. **CLI `rename`**: `naming.json`, `--copy`, `--output-directory`, dry-run.
8. **Conformance**: `naming` (exact relative POSIX paths, Unicode/ASCII), `filename` (`valid` fields, `invalid` rejected).
9. **Differential**: template-rendering fuzz via the harness. Generate random profiles and metadata (Unicode-heavy), render in both implementations, compare. This is where T-1/T-7/T-8 will bite.

## Exit criteria

- [ ] Conformance `naming` and `filename` green.
- [ ] 10k generated profile×metadata cases render identically in Python and Rust, or each difference is an SG entry.
- [ ] CHANGELOG `0.5.0`; PROJECT_LOG; tag `v0.5.0`.
