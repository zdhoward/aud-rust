# Changelog

Format: [Keep a Changelog](https://keepachangelog.com/en/1.1.0/). Versioning: [SemVer](https://semver.org/). Pre-1.0 minor versions mark milestone exits.

## [Unreleased]

## [0.1.0] - 2026-10-20

### Added

- `aud list` with `--extensions`, `--pattern`, `--json`, `--json-version 1`; JSON v1 documents (schema-validated), error documents and exit codes 0/2; legacy `aud DIRECTORY` count form.
- `aud-core`: selection policy, single-directory scanner, `fnmatchcase`-compatible glob, `Dir` facade, v1 envelope with embedded schema.
- `scripts/differential.py` (+ `.ps1`/`.sh`): Python-vs-Rust comparison, 20 scenarios identical.

### Changed

- Plan traps T-2 and T-16 corrected (SG-4, SG-5); SG-6 records usage-message wording divergence.

## [0.0.1] - 2026-10-20

### Added

- Cargo workspace (`aud-core`, `aud-cli`) with shared lints and pinned toolchain 1.99.1; `aud --version`.
- CI: lint, three-OS test with FFmpeg, coverage gate, supply-chain (deny, audit), msrv.
- Repo hygiene: SECURITY, CONTRIBUTING, dependabot, `deny.toml`, branch protection on `main`.

### Added

- Planning artifacts: implementation plan, ADRs, spec snapshot of Python `aud` @ `bb01db7`, conformance corpus v1, JSON schema v1, agent instructions.
