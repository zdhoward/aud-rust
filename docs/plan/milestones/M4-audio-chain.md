# M4 — Audio chain and canonical metadata (v0.4.0)

**Goal:** ordered native effects compiled to one FFmpeg filter graph with a single final encode; canonical metadata views over raw tags; CLI `effects`.

**Spec refs:** `spec/AUDIO_CHAINS.md` (entire), `spec/FFMPEG_CONVERSION.md` (metadata sections), `spec/CLI.md` (`effects.json`, output flags), `conformance/v1/media.json` (`metadata`, remaining tag cases).
**Python refs:** `core/effect_chain.py`, `core/operations/audio/{chain,effects}.py` (only the six supported effects), `core/metadata.py`, `core/adapters/convert.py` (chain routing); tests `test_audio_chain.py`, parts of `test_media_conformance.py`.

## Tasks

1. `AudioMetadata::from_tags` — port line by line, including alias conflict handling, `N/M` positions, year/date parsing (`YYYY[-MM[-DD]]` validated as a real calendar date — Rust needs its own check; `time`/`chrono` optional, a 30-line validator is enough), issues list. Property tests on tag permutations.
2. Effect types and `validate_effects` (1–64 effects in CLI config; finite numbers; non-negative except gain; cutoff > 0).
3. `compile_effects(effects, rate, channels, duration)` → filter strings + final duration. Mind T-10 (rounding) and T-11 (float text). Unit tests assert exact filter strings against Python-produced expectations (generate them once with a script from the pinned Python and store them in `tests/fixtures/filters.json`, with the generator script committed).
4. `AudioChain` operation: one decode, one final encode, optional `ConvertFormat` output; without output, publishes in place with source format and properties. Fade-out requires known duration ≥ fade length; Nyquist check; left/right phase inversion requires stereo.
5. Chain result details: canonical source/output metadata with parsing issues.
6. CLI `effects` with `effects.json` parsing (bounded JSON loader from T-14 now becomes shared infrastructure in `aud-cli::config`).
7. Decoded-sample tests: gain of −6.0206 dB halves 16-bit sample values within rounding; pad adds exactly `round_ties_even(s·rate)` frames; fade ramps; phase inversion negates; low/high-pass attenuate a tone above/below cutoff by a stated margin.

## Traps

T-10, T-11, T-12, T-14, T-15. Native effects "are not promised to match pydub": do not compare with pydub, only with the pinned Python's FFmpeg chain.

## Exit criteria

- [ ] Filter strings equal Python's for the fixture matrix (or divergence explained in SG).
- [ ] Differential `effects` scenarios match on decoded PCM exactly (same FFmpeg, same graph ⇒ same samples; if not, investigate rather than loosen).
- [ ] Single-encode property proven: test counts encoder invocations via the fake engine.
- [ ] CHANGELOG `0.4.0`; PROJECT_LOG; tag `v0.4.0`.
