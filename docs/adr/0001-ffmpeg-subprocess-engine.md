# ADR-0001: FFmpeg subprocess engine behind a trait

Status: Accepted · 2026-10-20

## Context

Python `aud` v2 already moved conversions to direct FFmpeg/ffprobe calls with explicit argument arrays, staged output, probe-and-decode verification, process limits and cancellation. The contract (24-bit PCM, tags, artwork, MP3/FLAC/Ogg profiles, effect filter chains) is defined in those terms, and the conformance corpus expects FFmpeg-grade behavior (lossy profile settings, artwork retention across formats).

## Decision

Implement `FfmpegEngine` (subprocess, no shell, allowlisted environment, limits, cancellation) behind `trait MediaEngine`. A fake engine serves failure-injection tests.

## Consequences

- Parity is achievable and checkable: same tool, same filters, comparable decoded PCM.
- No audio-codec work in Rust; the portfolio value is in the safety, contract, and verification engineering around it.
- Runtime dependency on external binaries, as in Python.
- The trait leaves room for a pure-Rust engine later without touching orchestration.

## Alternatives considered

- **Pure Rust (`symphonia`, `hound`, `claxon`, `lofty`)**: no MP3/Ogg *encoding* of equal quality, tag/artwork parity unproven, effect filters would be reimplemented and could not match FFmpeg's. Rejected for 1.0; a read-only probe or WAV path may come as a post-1.0 stretch behind the trait.
- **`ffmpeg-next` / libav bindings**: requires linking native libraries and `unsafe` FFI. Rejected (ADR-0003).
