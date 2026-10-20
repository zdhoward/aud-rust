# ADR-0002: Workspace and API shape

Status: Accepted · 2026-10-20

## Context

Python exposes a fluent `Dir` facade with ~60 methods (many boolean convenience wrappers) plus plan types. Its roadmap lists "registry dispatch" and "collapse duplicated try/except wrappers" as wanted simplifications.

## Decision

- Cargo workspace with `aud-core` (library, no terminal I/O) and `aud-cli` (binary `aud`).
- Operations are a closed `enum Operation`. Dispatch is an exhaustive `match`.
- `Dir` stays, but thin: configuration (extensions, allow/deny, glob pattern, link policy, overwrite policy) plus `plan()`, `preview()`, `execute()`. No per-operation convenience methods; builders on `Plan` replace them.
- Library errors are typed (`thiserror`); the CLI uses `anyhow` at the edge only.

## Consequences

- Smaller public surface than Python, easier to keep stable.
- Adding an operation touches one enum and its `match` sites; the compiler lists them.
- Third-party operations are impossible until a deliberate extension design. That matches Python, where the plugin surface is unscheduled.

## Alternatives considered

- Trait objects (`Box<dyn Operation>`): closest to Python, but loses exhaustiveness and needs downcasting for adapter dispatch.
- Single crate: simpler, but a lib/bin split shows API discipline and lets tests treat the CLI as a black box.
