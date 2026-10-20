# Port map

Python source is at `<aud>/aud/…` (`S:\ZX\Programming\aud` or `.reference/aud`). Line counts are as of `bb01db7`; they size the work, they do not predict the Rust size.

| Python module | LoC | Rust module | M | Verified by |
| --- | --- | --- | --- | --- |
| `core/models.py` | 35 | `model.rs` | 1 | unit |
| `core/selection/{policy,extensions,composite,listing,scanner}.py` | ~100 | `selection/*` | 1 | `tests/test_dir_selection.py`, `test_dir_config.py` ported |
| `contracts.py`, `schemas/v1.json` | 34 | `contracts.rs` | 1 | schema validation of every emitted doc |
| `cli.py` (list only) | 411 total | `aud-cli` | 1 → 7 | `tests/test_cli.py` ported incrementally |
| `core/safety.py` | 163 | `safety.rs` | 2 | `tests/test_safety.py`; conformance `collisions` |
| `core/results.py` | 33 | `results.rs` | 2 | conformance `results` |
| `core/plan.py`, `core/execution.py` | 281 | `plan.rs`, `execution.rs` | 2 | `test_plan.py`, `test_execution.py` |
| `core/operations/{names,files,archive,base}.py` | ~130 | `operations/{names,files,archive}.rs` | 2 | `test_names.py`, `test_filesystem_ops.py`; conformance `copy` |
| `core/adapters/{filesystem,batch}.py` | 275 | `adapters/filesystem.rs`, `execution.rs` | 2 | `test_adapters.py` |
| `core/ffmpeg.py` | 616 | `media/{ffmpeg,limits,engine}.rs` | 3 | `test_ffmpeg_engine.py`; conformance `pcm`, `lossy`, `recovery` |
| `core/operations/audio/convert.py`, `adapters/convert.py` | 121 | `operations/convert.rs`, `adapters/convert.rs` | 3 | `test_conversion.py`, `test_export.py` |
| `core/metadata.py` | 98 | `metadata.rs` | 4 | conformance `metadata`, `artwork` |
| `core/effect_chain.py`, `operations/audio/{chain,effects}.py` | ~230 | `effects.rs`, `operations/{effects,chain}.rs` | 4 | `test_audio_chain.py` |
| `core/naming.py`, `operations/naming.py` | 267 | `naming.rs`, `operations/naming.rs` | 5 | `test_naming_profiles.py`; conformance `naming` |
| `core/filename.py` | 105 | `filename.rs` | 5 | `test_filename_parsing.py`; `filename.json` |
| `core/delivery.py` | 120 | `delivery.rs` | 6 | conformance `delivery_profiles`, `duration_boundaries` |
| `core/packaging.py` | 679 | `packaging.rs` | 6 | `test_album_packaging.py`; conformance `album` |
| `core/progress.py` | 39 | `progress.rs` | 2 (types), 7 (CLI) | `test_progress.py`; conformance `progress` |
| `exceptions.py` | 28 | `error.rs` | 2 | `test_exceptions.py` |
| `aud.py` (`Dir` facade, 860 lines) | 860 | `lib.rs` `Dir` (thin) | 1 → 6 | `test_dir_api.py`; facade grows with each milestone |
| `core/adapters/audio.py`, `audio_io.py` (pydub) | 285 | **not ported** | – | out of scope |
| `main.py`, `__main__.py`, `tools/*` | – | **not ported** | – | packaging scripts for Python |

## Python tests as spec

`tests/test_*.py` are the second-best spec after the docs. For each module: read the Python test file, list its behaviors as a checklist in the PR description, and port each as a Rust test or document why not. Tests that assert pydub behavior (`test_audio_fx.py`, `test_new_audio_ops.py`) are skipped. Packaging/release tests (`test_distribution_integrity.py`, `test_installed_runner.py`, `test_import_observation.py`) are Python-specific and skipped.

Tracking: `TODO.md` carries one checkbox per Python test file; tick it when the file's behaviors are all ported or consciously excluded.

## Conformance corpus ownership

| Group | File | Milestone that turns it green |
| --- | --- | --- |
| `copy`, `results` | `cases.json` | 2 |
| `progress`, `error` (document fixtures) | `cases.json` | 1 (schema) / 7 (CLI emission) |
| `naming` | `cases.json` | 5 |
| `pcm`, `metadata`, `artwork`, `lossy`, `recovery`, `collisions` | `media.json` | 3 (collisions: 2) |
| `album`, `delivery_profiles`, `duration_boundaries` | `media.json` | 6 |
| `valid`, `invalid` | `filename.json` | 5 |

The corpus README (`conformance/v1/README.md`) defines how each group is executed. The runner must follow it exactly: replace `$ROOT` only in path fields, compare decoded JSON, never compare encoder-version-dependent bytes.
