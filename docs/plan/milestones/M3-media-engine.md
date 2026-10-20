# M3 — Media engine (v0.3.0)

**Goal:** bounded, cancellable FFmpeg execution; probing; verified conversion with profiles, 24-bit PCM, tags and artwork; CLI `inspect`, `convert`, `export`.

**Spec refs:** `spec/FFMPEG_CONVERSION.md` (entire), `spec/SAFE_PROCESSING.md` (audio exports), `spec/SECURITY_REVIEW.md` (process and path threats), `spec/CLI.md` (inspect/convert/export), `conformance/v1/media.json` (`pcm`, `metadata`*, `artwork`, `lossy`, `recovery`, `collisions` incl. cross-format).
**Python refs:** `core/ffmpeg.py` (read all 616 lines), `core/operations/audio/convert.py`, `core/adapters/convert.py`, `core/adapters/audio_io.py` (only for tool-presence checks), `aud.py` (`convert_*`, `export_for`, `_EXPORT_PRESETS`); tests `test_ffmpeg_engine.py`, `test_conversion.py`, `test_export.py`, `test_media_conformance.py`.

\* `metadata` conformance asserts tag round-trips; canonical `AudioMetadata` arrives in M4, so run only the tag-retention assertions here.

## Tasks

1. **`MediaLimits`** with validation (finite positive timeout, positive byte limits). Defaults: 300 s, 4 MiB stdout, 128 KiB stderr, 8 GiB output.
2. **`ProcessRunner`** (T-13): no shell, `stdin` null, allowlisted env, process group, bounded capture, output-size polling at 50 ms, timeout, cancellation, kill and reap in all paths. Tests with a tiny helper binary or `ffmpeg` flags that block/emit a lot (see `test_ffmpeg_engine.py` for how Python provokes these).
3. **Tool discovery.** Locate `ffmpeg`/`ffprobe`; actionable error when missing. Hardening flags Python passes (local protocols and demuxers only, single codec thread; read `ffmpeg.py` for exact argv).
4. **Probe.** ffprobe JSON → `AudioInfo` (codec, rate, channels, bit depth, duration, bit rate, tags, artwork stream indices); container tags beat stream tags; `LOSSLESS_CODECS`/`LOSSY_CODECS` sets verbatim.
5. **`ConvertFormat`** + `validate_conversion` (every rule in `ffmpeg.py` lines ~179–260), profiles `flac-preserve|mp3-v0|mp3-v2|mp3-320`, lossless-input rule with `allow_lossy`, mono/stereo variants, `drop_artwork`, explicit tags, collision policy for cross-format outputs.
6. **Encode + verify.** Stage → encode → probe → full decode → check rate/channels/depth (and duration within tolerance for lossless→WAV/FLAC) → publish exclusively (M2 machinery). `MediaEngine` trait + `FfmpegEngine` + `FakeEngine`.
7. **Presets.** `export_for` presets `cd`, `amuse`, `mp3`, `wav` (see `_EXPORT_PRESETS`); export = converted copies to a different directory, selection untouched.
8. **CLI.** `inspect`, `convert`, `export` with flags in `spec/CLI.md`; `--timeout`, `--output-bytes`; dry-run semantics.
9. **Conformance.** `pcm` (exact sample equality through FLAC), `lossy` (effective profile/encoder/bitrate), `artwork`, `recovery` (fake-engine error injection and cancellation: statuses, `a.flac` preserved, no leftovers), cross-format `collisions`.

## Traps

T-5, T-13, T-20, T-21, T-22. Also: FFmpeg flag spelling differs by version; assert behavior, not argv text, in unit tests, but keep one test that pins the argv the Rust code emits.

## Exit criteria

- [ ] Conformance `pcm`, `lossy`, `artwork`, `recovery`, `collisions` green on three OSes.
- [ ] Differential scenarios `convert`/`export`/`inspect` match on decoded PCM and tags; lossy bytes not compared.
- [ ] Timeout, output-limit, and cancellation tests kill and reap the child (no orphan processes: assert via process listing in the test).
- [ ] CI installs FFmpeg with `libmp3lame` and `libvorbis`; tests skip with a clear message only when an encoder is genuinely absent.
- [ ] CHANGELOG `0.3.0`; PROJECT_LOG; tag `v0.3.0`.
