# TODO

Legend: `[ ]` open · `[x]` done · `[~]` consciously excluded (reason inline). Tick items in the same commit that completes them. Milestone detail lives in `docs/plan/milestones/`.

## Done: M0 — Bootstrap → [details](docs/plan/milestones/M0-bootstrap.md)

- [x] M0-1 Install Rust toolchain + components; record versions in PROJECT_LOG
- [x] M0-2 Install `cargo-deny`, `cargo-audit`, `cargo-llvm-cov`
- [x] M0-3 Workspace: root `Cargo.toml`, `aud-core`, `aud-cli`, `rust-toolchain.toml`, shared lints
- [x] M0-4 Smoke tests (`aud --version`), `forbid(unsafe_code)`
- [x] M0-5 CI: lint, test matrix (3 OS + FFmpeg), supply-chain, coverage, msrv; `deny.toml`
- [x] M0-6 Hygiene: dependabot, SECURITY.md, CONTRIBUTING.md, Rust-specific issue/PR templates
- [x] M0-7 GitHub: confirm `origin`, description/topics, branch protection, private vulnerability reporting
- [x] M0-8 README with real badges; verify `scripts/fetch-reference.ps1`
- [x] M0-X Exit: CHANGELOG 0.0.1, PROJECT_LOG, tag `v0.0.1`

## M1 — Tracer bullet → [details](docs/plan/milestones/M1-tracer-bullet.md)

- [x] M1-1 `AudioFile`, selection policies, `fnmatch`, scanner (links, sort, non-UTF-8)
- [x] M1-2 `contracts.rs` + embedded schema + doc validation helper
- [ ] M1-3 CLI `list` (+ error docs, exit codes, legacy count form decision)
- [ ] M1-4 Test helpers: temp tree, `$ROOT` normalizer, schema validator
- [ ] M1-5 Differential harness skeleton (`scripts/differential.*`), ≥ 10 `list` scenarios
- [x] M1-6 Schema-validate `progress`/`error` fixtures in `cases.json`
- [ ] M1-X Exit: tag `v0.1.0`

## M2 — Safety core → [details](docs/plan/milestones/M2-safety-core.md)

- [ ] M2-1 Results/status/errors · [ ] M2-2 Destination resolution + policies · [ ] M2-3 Staged write + exclusive publish
- [ ] M2-4 Operations + plan + preview · [ ] M2-5 Filesystem adapter + executor · [ ] M2-6 `Dir` facade
- [ ] M2-7 Conformance `copy`/`results`/`collisions` (fs-only) · [ ] M2-8 Progress types
- [ ] M2-X Exit: tag `v0.2.0`

## M3 — Media engine → [details](docs/plan/milestones/M3-media-engine.md)

- [ ] M3-1 `MediaLimits` · [ ] M3-2 `ProcessRunner` · [ ] M3-3 Tool discovery · [ ] M3-4 Probe
- [ ] M3-5 `ConvertFormat` + validation + profiles · [ ] M3-6 Encode/verify/publish + `MediaEngine`/fake
- [ ] M3-7 Export presets · [ ] M3-8 CLI `inspect`/`convert`/`export` · [ ] M3-9 Conformance media groups
- [ ] M3-X Exit: tag `v0.3.0`

## M4 — Audio chain → [details](docs/plan/milestones/M4-audio-chain.md)

- [ ] M4-1 `AudioMetadata` · [ ] M4-2 Effect validation · [ ] M4-3 Filter compiler + Python-derived fixtures
- [ ] M4-4 `AudioChain` op · [ ] M4-5 Chain result metadata · [ ] M4-6 CLI `effects` + shared config loader · [ ] M4-7 Decoded-sample tests
- [ ] M4-X Exit: tag `v0.4.0`

## M5 — Naming + parsing → [details](docs/plan/milestones/M5-naming-and-parsing.md)

- [ ] M5-1 Template parser · [ ] M5-2 `NamingProfile` · [ ] M5-3 Tree validation · [ ] M5-4 `NameWithProfile`
- [ ] M5-5 `FilenamePattern` · [ ] M5-6 Metadata sources · [ ] M5-7 CLI `rename` · [ ] M5-8 Conformance · [ ] M5-9 Render differential
- [ ] M5-X Exit: tag `v0.5.0`

## M6 — Album packaging → [details](docs/plan/milestones/M6-album-packaging.md)

- [ ] M6-1 Delivery profiles · [ ] M6-2 Duration rules · [ ] M6-3 Input snapshot + member checks · [ ] M6-4 Artwork + sidecars
- [ ] M6-5 Settings/consent · [ ] M6-6 Manifest/checksums/ZIP · [ ] M6-7 Preview/execute/progress · [ ] M6-8 CLI `album` · [ ] M6-9 Conformance + differential
- [ ] M6-X Exit: tag `v0.6.0`

## M7 — CLI parity → [details](docs/plan/milestones/M7-cli-parity.md)

- [ ] M7-1 CLI audit · [ ] M7-2 Config hardening · [ ] M7-3 Progress · [ ] M7-4 Exit codes + Ctrl+C · [ ] M7-5 JSON error docs
- [ ] M7-6 Legacy count form · [ ] M7-7 Optional extras (completions, verbose) · [ ] M7-8 Library docs
- [ ] M7-X Exit: tag `v0.7.0`

## M8 — Differential + hardening → [details](docs/plan/milestones/M8-differential-hardening.md)

- [ ] M8-1 Differential CI · [ ] M8-2 Scenario generator · [ ] M8-3 Fuzzing · [ ] M8-4 Panic audit
- [ ] M8-5 Security review · [ ] M8-6 Benchmarks · [ ] M8-7 Coverage ≥ 85%
- [ ] M8-X Exit: tag `v0.8.0`

## M9 — Release → [details](docs/plan/milestones/M9-release.md)

- [ ] M9-1 README final · [ ] M9-2 Docs · [ ] M9-3 Release engineering · [ ] M9-4 Case study · [ ] M9-5 Clean-clone audit
- [ ] M9-6 Repo presentation (user actions) · [ ] M9-X Tag `v1.0.0`

## Python test files to port

Tick when every behavior in the file is ported or excluded. Mapping in `docs/plan/03-port-map.md`.

- [x] test_dir_selection · [x] test_dir_config · [x] test_dir_api · [ ] test_contracts (M1)
- [ ] test_safety · [ ] test_plan · [ ] test_execution · [ ] test_names · [ ] test_filesystem_ops · [ ] test_adapters · [ ] test_exceptions · [ ] test_progress (M2)
- [ ] test_ffmpeg_engine · [ ] test_conversion · [ ] test_export · [ ] test_media_conformance (M3, M4, M6 parts)
- [ ] test_audio_chain (M4) · [ ] test_naming_profiles · [ ] test_filename_parsing (M5) · [ ] test_album_packaging (M6) · [ ] test_cli (M1→M7)
- [ ] test_backward_compat — read first; port only what concerns the contract surface, exclude legacy `Dir` boolean wrappers
- [~] test_audio_fx, test_new_audio_ops — pydub effects, out of scope
- [~] test_distribution_integrity, test_installed_runner, test_import_observation — Python packaging

## Backlog / ideas (unscheduled)

- Pure-Rust read-only probe behind `MediaEngine` (post-1.0)
- Peak-normalize chain effect (post-1.0; Python lacks it outside pydub)
- Recursive scan (Python roadmap item)
