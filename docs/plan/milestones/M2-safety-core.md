# M2 — Safety core (v0.2.0)

**Goal:** plans, previews, collision resolution, staged and exclusive writes, file operations, per-file results, cancellation. No audio yet. This is the heart of the contract and the best showcase for Rust's strengths (ownership, RAII cleanup, enums).

**Spec refs:** `spec/SAFE_PROCESSING.md` (entire), `spec/ARCHITECTURE.md`, `spec/API.md` (operations, results, errors), `conformance/v1/cases.json` (`copy`, `results`), `conformance/v1/media.json` (`collisions`).
**Python refs:** `core/safety.py`, `core/results.py`, `core/plan.py`, `core/execution.py`, `core/operations/{base,names,files,archive}.py`, `core/adapters/{filesystem,batch}.py`, `exceptions.py`, `core/progress.py` (types only); tests `test_safety.py`, `test_plan.py`, `test_execution.py`, `test_names.py`, `test_filesystem_ops.py`, `test_adapters.py`, `test_exceptions.py`, `test_progress.py`.

## Tasks (vertical slices; one PR each)

1. **Results + errors.** `Status` (`success`, `skipped`, `failed`, `not_run`, `cancelled`; confirm full list in schema), `FileResult`, `BatchResult::succeeded`, `AudError`. Conformance `results`.
2. **Destination resolution.** `path_key` (T-3, T-4), `is_link` (T-6), `check_path`, `Destination`, `destinations()` with policies `error|skip|overwrite|auto-rename`, protected paths, same-file alias detection (T-19), case/Unicode collision rejection, in-place rules. Property tests: resolution is deterministic, never yields two equal keys, `auto-rename` terminates and never collides.
3. **Staged write + exclusive publish.** `StagedFile` RAII guard (T-20), `hard_link` publish (T-5, ADR-0003), explicit replace via `rename`. Test: destination appears between preflight and publish ⇒ survives, error reported, staging removed.
4. **Operations + plan.** `Operation` enum (names, copy/move/backup, zip). `Iterate` counter clones on resolve (T-24). `Plan::new(files).add(op)`. `resolve()` per `execution.py` including grouped zip (duplicate member names rejected). `PlanPreview`/`PreviewStep` serialization matches schema `preview`.
5. **Filesystem adapter + executor.** Preflight all steps, record per-file results, only successes change the selection, `continue_on_error`, failed/cancelled dependencies do not reach later steps (`not_run`), cancellation between files via `CancelToken`, move deletes the source only after publish.
6. **`Dir` facade.** `plan()`, `preview()`, `execute()`, `last_result` equivalent. Selection snapshot semantics.
7. **Conformance runner.** `copy` group end to end (write input, preview, execute, compare v1 docs and byte equality), `collisions` group via the fake engine seam where conversion is needed (leave marked `pending-M3` otherwise).
8. **Progress types.** `ProgressEvent`, `ProgressReporter` with failing-callback disablement (`catch_unwind`). Emission from the executor; CLI display comes in M7.

## Traps

T-2, T-3, T-4, T-5, T-6, T-19, T-20, T-21, T-24, T-25.

## Exit criteria

- [ ] Conformance `copy`, `results` green; `collisions` green for the filesystem-only cases.
- [ ] All Python tests listed above ported or consciously excluded (noted in TODO.md).
- [ ] Failure-injection tests prove: no leftover staging files after error, cancel, or panic-unwind; original destination bytes preserved.
- [ ] Preview equals execution targets for every operation (property test).
- [ ] Differential scenarios for copy/move/backup/zip/name operations via a temporary dev-only harness subcommand or library driver (the CLI exposes these only through `rename`/`album`; use `Dir`-level driver in the harness where Python has none).
- [ ] CI green; CHANGELOG `0.2.0`; PROJECT_LOG entry; tag `v0.2.0`.
