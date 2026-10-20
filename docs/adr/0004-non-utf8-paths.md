# ADR-0004: Non-UTF-8 file names are a typed error

Status: Accepted · 2026-10-20

## Context

Rust paths are `OsStr`. Python maps undecodable bytes to surrogates. The JSON v1 contract, sort order (T-2), `path_key` normalization (T-3), and naming limits (T-9) are all defined on Unicode text.

## Decision

The scanner converts each directory entry name to `String`. A name that is not valid Unicode yields `AudError::File` naming the entry (lossy display), and the scan stops. The library never silently skips or lossily renames a file. Internally, paths remain `PathBuf`; the check is at ingestion and at config/JSON boundaries.

## Consequences

- Deterministic ordering and JSON representation.
- Documented divergence (SG-1). Directories containing such names must be cleaned or narrowed with `--pattern` only if the scanner is changed to skip them, which would be a new ADR.

## Alternatives considered

- Skip with a warning: hides inputs from batch results, which violates "results identify every file".
- Preserve via `OsString` end to end: no faithful JSON form.
