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
