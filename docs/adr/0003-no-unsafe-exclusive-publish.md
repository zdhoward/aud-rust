# ADR-0003: No unsafe code; exclusive publish via hard link

Status: Accepted · 2026-10-20

## Context

Python guarantees a destination that appears after preflight is never overwritten (`spec/SAFE_PROCESSING.md`): Windows uses exclusive `os.rename`, POSIX a hard link. Rust's `std::fs::rename` **replaces** an existing target on every platform, so it cannot publish exclusively. A Windows no-replace move needs `MoveFileExW` without `MOVEFILE_REPLACE_EXISTING` via FFI (`unsafe`).

## Decision

`#![forbid(unsafe_code)]` in every crate. Publish a new destination with `std::fs::hard_link(staging, dest)` followed by removing the staging name; it fails if `dest` exists, on NTFS and POSIX alike. Explicit replacement uses `fs::rename`. Filesystems without hard links fail safely with a typed error, as Python already does on POSIX.

## Consequences

- A concrete safety property is provable by test: create the destination between preflight and publish; it must survive.
- Windows on non-NTFS volumes (FAT/exFAT) cannot publish new files. Recorded as SG-3.
- Process-group termination on POSIX goes through `nix` (a safe API) rather than raw `libc`.

## Alternatives considered

- Isolated `unsafe` module with `windows-sys`: exact Python parity on Windows; rejected for the portfolio value of a zero-unsafe claim. Revisit only if SG-3 causes real failures.
- `rename` after an existence check: has a race; the property is lost.
- `OpenOptions::create_new` + copy: not atomic for readers; partial files visible.
