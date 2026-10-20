# M0 — Bootstrap (v0.0.1)

**Goal:** a green, empty-but-real Cargo workspace with CI on three OSes, so every later PR lands on infrastructure that already works.

**Spec refs:** none. **Plan refs:** [02-architecture.md](../02-architecture.md), [04-verification.md](../04-verification.md) (CI jobs).

## Tasks

1. **Toolchain.** Install Rust via rustup (`winget install Rustlang.Rustup`; if the MSVC linker is missing, install "Visual Studio Build Tools" with the C++ workload). Verify `rustc --version`, `cargo --version`, `ffmpeg -version`, `ffprobe -version`. Record versions in PROJECT_LOG.
2. **Dev tools.** `rustup component add rustfmt clippy llvm-tools-preview`; `cargo install cargo-deny cargo-audit cargo-llvm-cov` (or use `cargo binstall`).
3. **Workspace.** Root `Cargo.toml` with `[workspace]`, `resolver = "3"`, `[workspace.package]` (edition 2024, `rust-version`, license MIT, repository, authors from `LICENSE`), `[workspace.lints]` per 02-architecture. Crates `crates/aud-core` (lib) and `crates/aud-cli` (bin `aud`; depends on core). `rust-toolchain.toml` pinned to a stable release. `rustfmt.toml` only if needed.
4. **Smoke code.** `aud --version` prints the version. One `aud-core` unit test and one `aud-cli` `assert_cmd` test. `#![forbid(unsafe_code)]` in both crates.
5. **CI** (`.github/workflows/ci.yml`): jobs `lint`, `test` (matrix ubuntu/macos/windows with FFmpeg installed per OS), `supply-chain`. Add `coverage` and `msrv` jobs. Pin actions by major version; `permissions: contents: read`; concurrency group cancels superseded runs. `deny.toml` allowing MIT/Apache-2.0/BSD/ISC/Unicode licenses.
6. **Repo hygiene.** `dependabot.yml` (cargo + github-actions, weekly). `SECURITY.md` (private vulnerability reporting). `CONTRIBUTING.md` (summarize AGENTS.md conventions for humans). Replace the copied Python issue/PR templates: bug report asks for `aud --version`, OS, FFmpeg version; PR template has an **AI assistance** section. Delete the Python-specific fields.
7. **GitHub repo.** Already created by the planning session; confirm remote `origin`. Set the repo description and topics (`rust`, `audio`, `ffmpeg`, `cli`, `batch-processing`, `port`). Enable branch protection on `main`: require `lint`, `test`, `supply-chain`; require PR; no force-push. Enable private vulnerability reporting.
8. **README.** Replace the planning README with one that has real CI badges and a "Status: M0 complete" line; keep the project story short (see M9 for the final form).
9. **Reference script.** Verify `scripts/fetch-reference.ps1` clones `zdhoward/aud` at the pinned SHA into `.reference/aud`.

## Exit criteria (all checkable)

- [ ] `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`, `cargo deny check`, `cargo audit` pass locally.
- [ ] CI green on `main` for all three OSes.
- [ ] Branch protection active (verify with `gh api repos/zdhoward/aud-rust/branches/main/protection`).
- [ ] CHANGELOG has `0.0.1`; PROJECT_LOG records tool versions and any deviation; tag `v0.0.1`.

## Notes

- Do the work on `chore/m0-bootstrap`, open a PR, let CI run, merge. Branch protection is enabled *after* the first green run, or the PR cannot merge.
- If a CI step needs a tool that differs per OS (FFmpeg install), put the logic in `scripts/ci-install-ffmpeg.sh` and call it from the workflow; keep the workflow readable.
