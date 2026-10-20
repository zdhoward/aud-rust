# Roadmap

Detailed milestone plans: [docs/plan/](docs/plan/README.md). Task-level status: [TODO.md](TODO.md). Versions change only when a milestone exits; tags are annotated and agree with CHANGELOG and PROJECT_LOG.

| Version | Milestone | Status |
| --- | --- | --- |
| 0.0.1 | M0 Bootstrap | done (v0.0.1) |
| 0.1.0 | M1 Tracer bullet (`list`) | done (v0.1.0) |
| 0.2.0 | M2 Safety core | not started |
| 0.3.0 | M3 Media engine (`inspect`, `convert`, `export`) | not started |
| 0.4.0 | M4 Audio chain (`effects`) | not started |
| 0.5.0 | M5 Naming + parsing (`rename`) | not started |
| 0.6.0 | M6 Album packaging (`album`) | not started |
| 0.7.0 | M7 CLI parity | not started |
| 0.8.0 | M8 Differential + hardening | not started |
| 1.0.0 | M9 Release + case study | not started |

## After 1.0 (ideas, not commitments)

- Pure-Rust read-only probe (`symphonia`/`lofty`) behind `MediaEngine`
- Peak-normalization effect with explicit headroom
- Recursive scanning with containment rules
- Parallel batch execution with bounded workers (Python roadmap #45), after serial failure and fidelity tests exist
- Track upstream Python contract changes: re-snapshot `spec/` and `conformance/` deliberately, bump the pin, re-run the differential suite
