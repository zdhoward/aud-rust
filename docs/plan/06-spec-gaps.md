# Spec gaps and intentional divergences

Append-only. One entry per finding. Format:

```
## SG-N: short title
- Found: date, milestone, who (human / AI)
- Python behavior: …
- Spec says: … (file:section) or "silent"
- Rust behavior: …
- Decision: match | diverge | ask upstream — with reason
- Evidence: test name / commit
```

The differential suite's `allowed-diffs.toml` may reference only entries listed here.

## Seeded entries

## SG-1: Non-UTF-8 file names
- Found: 2026-10-20, planning
- Python behavior: `Path` carries undecodable bytes via surrogateescape; most operations proceed.
- Spec says: silent.
- Rust behavior: scan fails with a typed error naming the lossy path ([ADR-0004](../adr/0004-non-utf8-paths.md)).
- Decision: diverge. JSON v1 cannot represent such names faithfully, and sorting/collision keys are defined on text.

## SG-2: Allow/deny regex dialect
- Found: 2026-10-20, planning
- Python behavior: `re.match` with Python `re` syntax (backreferences, lookaround allowed; ReDoS-prone, flagged in Python ROADMAP).
- Spec says: prefix matching, no guard.
- Rust behavior: `regex` crate (linear time), start-anchored.
- Decision: diverge. Safer and the Python roadmap already plans bounding regex cost. Library-only; CLI exposes globs.

## SG-3: Exclusive publish mechanism on Windows
- Found: 2026-10-20, planning
- Python behavior: Windows uses exclusive `os.rename`; POSIX uses a hard link.
- Rust behavior: hard link on every platform ([ADR-0003](../adr/0003-no-unsafe-exclusive-publish.md)). Filesystems without hard links fail safely instead of falling back.
- Decision: diverge. Keeps `forbid(unsafe_code)`. Non-NTFS Windows targets lose support; revisit if a user hits it.

## SG-4: Scan order is case-insensitive on Windows (trap T-2 was incomplete)
- Found: 2026-10-20, M1, AI (confirmed by running Python 3.13.1 on Windows)
- Python behavior: `sorted(Path)` compares `PurePath` normalized parts. On Windows these are lowercased, so `A.wav, a2.wav, b.wav, Z.wav`; on POSIX plain code point order. Non-BMP names sort by code point, not UTF-16.
- Spec says: "Files are processed in sorted order" (`spec/ARCHITECTURE.md`), no detail.
- Rust behavior: scanner sorts by `to_lowercase()` on Windows and by code point elsewhere, with the raw name as tie-break.
- Decision: match, per platform. The differential suite runs Python and Rust on the same OS.
- Evidence: `scanner::tests::windows_sort_ignores_case_like_pathlib`, `posix_sort_is_case_sensitive`, differential scenario `sort-order`.

## SG-5: `--pattern` is case-sensitive on every OS (trap T-16 was wrong)
- Found: 2026-10-20, M1, AI
- Python behavior: `Dir.update` calls `fnmatch.fnmatchcase`, which skips `os.path.normcase`. Matching is case-sensitive on Windows too.
- Spec says: silent ("filename glob").
- Rust behavior: case-sensitive everywhere; no Windows case folding.
- Decision: match.
- Evidence: `dir::tests::pattern_is_case_sensitive_everywhere`; `fnmatch::tests::matches_python_fnmatchcase` (expected values from Python 3.13.1).

## SG-6: Usage-error wording and the legacy count form
- Found: 2026-10-20, M1, AI
- Python behavior: usage errors carry `argparse` messages. `aud DIRECTORY` (one non-option, non-command argument) prints `Found N files` using the default empty selection (always 0), exit 0; a missing directory prints `Directory does not exist: "<path>"` on stdout, exit 1; no arguments prints a usage line, exit 1.
- Spec says: `spec/CLI.md` keeps the count form "available".
- Rust behavior: clap generates the usage messages, so message text differs (exit code 2 and document shape match). The legacy form is ported exactly, because it is three lines and the spec promises it. `--help` text is clap's.
- Decision: match behavior and shapes, diverge on `argparse` wording. The differential harness masks `error` text only in scenarios that set `mask_error` (usage errors); all other error text is compared after `$ROOT` normalization.
- Evidence: `tests/list.rs::legacy_count_form`, differential scenarios `legacy-count`, `legacy-missing`, `usage-error-json`, `version-without-json`.
