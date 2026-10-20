# ADR-0005: Verification strategy

Status: Accepted · 2026-10-20

## Context

A rewrite is only credible if equivalence is demonstrated. The Python project ships a portable conformance corpus built for exactly this purpose, but it states plainly that the corpus is "representative", not a full port gate.

## Decision

Verify in layers (full table in [04-verification.md](../plan/04-verification.md)): unit and property tests, ported Python tests, the conformance corpus on three operating systems, JSON Schema validation of every emitted document, failure injection through a fake engine, and a differential suite that runs the pinned Python implementation against Rust on generated inputs. Differences must be recorded in `06-spec-gaps.md` to be allowed.

## Consequences

- The Python source at SHA `bb01db7` is a test oracle; the pin is recorded in `spec/README.md` and changes only by deliberate re-snapshot.
- The differential suite is the feature that makes the portfolio claim ("same behavior") falsifiable.
- CI cost grows (Python + FFmpeg on runners). The differential job is separate and may run nightly.

## Alternatives considered

- Conformance corpus only: too thin; its authors say so.
- Golden outputs recorded from Python and committed: drift-prone and encoder-version-dependent.
