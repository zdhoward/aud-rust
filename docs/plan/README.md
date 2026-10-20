# Implementation plan — index

| Doc | Purpose |
| --- | --- |
| [01-scope.md](01-scope.md) | What is ported, what is not, success criteria, decisions already made |
| [02-architecture.md](02-architecture.md) | Workspace layout, module design, key types, dependency list |
| [03-port-map.md](03-port-map.md) | Python module → Rust module, owning milestone, conformance coverage |
| [04-verification.md](04-verification.md) | Test layers, differential harness, CI jobs, performance claims |
| [05-porting-traps.md](05-porting-traps.md) | Python behaviors that silently differ in Rust. Read before porting a module |
| [06-spec-gaps.md](06-spec-gaps.md) | Spec ambiguities and contradictions found during the port (append-only) |
| [07-ai-workflow.md](07-ai-workflow.md) | How AI assistance is run, recorded and shown |
| [milestones/](milestones/) | M0–M9: scope, tasks, exit criteria per milestone |

Milestone sequence (each exits with a tagged `v0.N.0`; M9 tags `v1.0.0`):

| M | Name | Delivers | Tag |
| --- | --- | --- | --- |
| [M0](milestones/M0-bootstrap.md) | Bootstrap | toolchain, workspace, CI, repo hygiene | v0.0.1 |
| [M1](milestones/M1-tracer-bullet.md) | Tracer bullet | `aud list --json --json-version 1` end to end | v0.1.0 |
| [M2](milestones/M2-safety-core.md) | Safety core | plans, collision policy, staged writes, file operations, results | v0.2.0 |
| [M3](milestones/M3-media-engine.md) | Media engine | FFmpeg runner, probe, convert/export, `inspect` | v0.3.0 |
| [M4](milestones/M4-audio-chain.md) | Audio chain | metadata canonicalization, effects, `effects` | v0.4.0 |
| [M5](milestones/M5-naming-and-parsing.md) | Naming + parsing | profiles, tree validation, filename grammar, `rename` | v0.5.0 |
| [M6](milestones/M6-album-packaging.md) | Album packaging | ZIP packages, 8 delivery profiles, `album` | v0.6.0 |
| [M7](milestones/M7-cli-parity.md) | CLI parity | progress, config hardening, exit codes, completions | v0.7.0 |
| [M8](milestones/M8-differential-hardening.md) | Differential + hardening | Python-vs-Rust suite, fuzzing, benchmarks, security review | v0.8.0 |
| [M9](milestones/M9-release.md) | Release | docs, binaries, case study, `1.0.0` | v1.0.0 |

Order is a dependency chain: each milestone consumes the previous one's types. Within a milestone, work in vertical slices (one behavior through every layer), not layer by layer.
