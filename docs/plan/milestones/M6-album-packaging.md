# M6 — Album packaging (v0.6.0)

**Goal:** generic and service-specific local ZIP packages with ordered audio, standalone artwork, sidecars, manifest and checksums; eight versioned delivery profiles; CLI `album`.

**Spec refs:** `spec/ALBUM_PACKAGING.md` (entire), `spec/CLI.md` (`album.json`), `spec/NAMING_PROFILES.md` (package naming), `conformance/v1/media.json` (`album`, `delivery_profiles`, `duration_boundaries`).
**Python refs:** `core/packaging.py` (679 lines; read fully), `core/delivery.py`, `aud.py` (`plan_album`, `package_album`), `core/operations/archive.py`; tests `test_album_packaging.py`, album parts of `test_media_conformance.py`.

## Tasks

1. `delivery.rs`: static profile data copied verbatim from `delivery.py` (names, formats, rate/depth pairs, limits, source URLs, `revision`, `checked_on`). Lookup normalizes `casefold`, strips `-` and spaces. Table-driven test against the conformance `delivery_profiles` group.
2. Duration boundary logic (`track_limit_exclusive`, `min_average_seconds`, track and release caps) — `duration_boundaries` group.
3. Package input snapshot (`_input_stamp`: size and mtime fields), archive member name validation and portable collision checks (`_member`, `_check_members`).
4. Artwork inspection via the engine (`_artwork`): dimensions, format, standalone requirement; sidecar copies bounded and byte-exact; opaque cue contents never interpreted.
5. Settings resolution (`_settings`): sample-change consent (`allow_audio_changes`), dither, lossy consent, track caps. Derivatives for service profiles via M3 conversion; generic packages copy originals byte-for-byte.
6. Manifest JSON (`_manifest`) and checksums (SHA-256); staged ZIP via M2 machinery; `track_order`, `default_disc`, `naming` integration; submission state stays `manual`.
7. `AlbumPackage::preview()` and `execute()` with progress (`staged` events; final `success` event signals publication) and cancellation.
8. CLI `album` with `--service`, `--target`, `album.json` loader.
9. Conformance + differential: compare member lists, manifest JSON, checksums for original-payload members; derivatives by decoded PCM.

## Traps

T-3, T-9, T-22, T-23. Manifest content is part of the contract; ZIP bytes are not.

## Exit criteria

- [ ] Conformance `album`, `delivery_profiles`, `duration_boundaries` green on three OSes.
- [ ] Differential `album` scenarios for `generic` and all 8 services match.
- [ ] Failed or cancelled packages remove only owned staging files; existing target preserved (tests).
- [ ] CHANGELOG `0.6.0`; PROJECT_LOG; tag `v0.6.0`.
