# Repository Guidelines

- Repo: https://github.com/jspwrd/forge
- File references must be repo-root relative (e.g. `src/build/compiler.rs:42`); never absolute paths.

## Project Overview

Forge is a build system for C/C++ cross-compilation, written in Rust. It simplifies compiling, patching, validating, and packaging C/C++ projects across multiple targets with a single `forge.toml` manifest. Key capabilities: cross-compilation with per-target toolchains, SHA256-based incremental builds, binary patching via sentinel markers, binary validation, automated test discovery, multi-format packaging (raw, DEB, DMG), and certificate management.

## Project Structure & Module Organization

- Entry point: `src/main.rs` (parses CLI, dispatches to `forge::cli::dispatch`).
- Library root: `src/lib.rs` (re-exports all public modules).
- Error types: `src/error.rs` (`ForgeError` enum with `thiserror`, exit code mapping).
- CLI definitions: `src/cli/mod.rs` (clap `Cli` struct, `Command` enum, `dispatch` function).
- CLI commands: `src/cli/*_cmd.rs` — one file per command (`build_cmd`, `patch_cmd`, `validate_cmd`, `clean_cmd`, `test_cmd`, `new_cmd`, `init_cmd`, `scripts_cmd`, `targets_cmd`, `config_cmd`, `cert_cmd`, `package_cmd`).
- Core modules:
  - `src/build/` — compiler abstraction, build context, incremental caching (SHA256), dependency resolution, toolchain detection.
  - `src/manifest/` — `ForgeManifest` deserialization from `forge.toml` (`types.rs`), validation (`validation.rs`).
  - `src/patch/` — binary sentinel scanning (`scanner.rs`), patch types (`types.rs`), writer (`writer.rs`).
  - `src/validate/` — binary checks (`checks.rs`: debug symbols, plaintext strings, compiler watermarks, RPATH, build paths), reporting (`report.rs`).
  - `src/package/` — artifact packaging: raw (`raw.rs`), DEB (`deb.rs`), DMG (`dmg.rs`).
  - `src/cert/` — certificate management via OpenSSL (`openssl.rs`).
  - `src/scaffold/` — project templates (`templates.rs`) for `new`/`init`.
  - `src/test_runner/` — C test discovery and compilation (`c_test.rs`), shellcheck integration (`shellcheck.rs`).
  - `src/scripts/` — custom script execution, environment setup (`env.rs`).
  - `src/clean/` — build artifact removal, secure wipe (`secure.rs`).
  - `src/util/` — shared helpers: filesystem (`fs.rs`), process execution (`process.rs`), colored output (`output.rs`).
- Docs: `docs/` (configuration.md, building.md, testing.md, patching-and-validation.md).
- CI: `.github/workflows/ci.yml` (check, fmt, clippy, test, multi-OS build), `.github/workflows/release.yml` (multi-platform release).
- Docker: `Dockerfile` (multi-stage: rust:1.85-bookworm builder, debian:bookworm-slim runtime).

## Build, Test, and Development Commands

- Language: Rust (edition 2024).
- Build: `cargo build`
- Build release: `cargo build --release`
- Run: `cargo run -- <forge-args>`
- Check (type/borrow check without codegen): `cargo check --all-features`
- Lint: `cargo clippy --all-features -- -D warnings`
- Format check: `cargo fmt --all -- --check`
- Format fix: `cargo fmt --all`
- Tests: `cargo test --all-features`
- Install from source: `cargo install --path .`
- Docker build: `docker build -t forge .`
- Docker run: `docker run --rm -v "$(pwd)":/project forge build`

## Coding Style & Naming Conventions

- Language: Rust. Follow standard Rust idioms and conventions.
- Error handling: use `ForgeError` (defined in `src/error.rs`) for domain errors with `thiserror`; use `anyhow::Result` for command-level orchestration.
- CLI: use `clap` derive macros. Each command gets its own `Args` struct and `execute` function in `src/cli/*_cmd.rs`.
- Output: use `src/util/output.rs` helpers (`OutputConfig` with verbose/quiet/no_color`). Do not use `println!` directly for user-facing output.
- No `unsafe` code unless absolutely necessary and well-justified.
- No `#[allow(...)]` suppressions; fix the root cause.
- Add brief code comments for tricky or non-obvious logic.
- Keep files concise; aim for under ~500 LOC. Extract sub-modules when files grow.
- Naming: use **Forge** for product/docs headings; `forge` for CLI command and paths.

## Testing Guidelines

- Integration tests use `assert_cmd` and `predicates` (dev-dependencies).
- Unit tests as `#[cfg(test)]` modules within source files.
- Test naming: descriptive snake_case.
- Run `cargo test --all-features` before pushing when you touch logic.
- CI enforces: `cargo check`, `cargo fmt --check`, `cargo clippy -D warnings`, `cargo test`.

## Project Configuration (forge.toml)

The manifest file `forge.toml` supports these sections:
- `[project]` — name, version, description.
- `[build]` — compiler, standard, static linking, strip, sources (glob patterns), includes, link libs, flags (common/release/debug).
- `[targets.<name>]` — per-target: cc, cxx, flags, sources, exclude, link, sysroot, enabled.
- `[dependencies.<name>]` — path, version, header_only.
- `[patch.<name>]` — sentinel, type, size (for binary patching).
- `[validate]` — no_debug_symbols, no_plaintext_strings, no_compiler_watermarks, max_binary_size, no_rpath, no_buildpaths.
- `[scripts.<name>]` — custom script commands.
- `[package]` — formats (raw, deb, dmg).

## Commit & Pull Request Guidelines

- Follow concise, action-oriented commit messages (e.g. `build: add SHA256 incremental caching`).
- Group related changes; avoid bundling unrelated refactors.
- CI must pass (check, fmt, clippy, test) before merging.

## Security & Configuration Tips

- Never commit real secrets, keys, or live configuration values.
- Use obviously fake placeholders in docs, tests, and examples.
- The `src/clean/secure.rs` module handles secure file wiping — use it for sensitive build artifacts.
- Certificate management (`src/cert/`) wraps OpenSSL — validate all paths and inputs.

## Agent-Specific Notes

- When answering questions, verify in code; do not guess.
- Do not modify `Cargo.lock` manually; let `cargo` manage it.
- When adding dependencies, justify the addition and prefer minimal, well-maintained crates.
- The project uses Rust edition 2024 — use current Rust idioms.
- The project is structured as both a binary (`src/main.rs`) and a library (`src/lib.rs`) — keep the library API clean.

### Multi-Agent Safety

- Do **not** create/apply/drop `git stash` entries unless explicitly requested.
- When told to "push", you may `git pull --rebase` (never discard others' work).
- When told to "commit", scope to your changes only.
- Do **not** switch branches unless explicitly requested.
- When you see unrecognized files, keep going; focus on your own changes.

### Lint/Format Churn

- If diffs are formatting-only, auto-resolve without asking.
- Only ask when changes are semantic (logic/data/behavior).

### Bug Investigations

- Read source code of relevant crates and all related local code before concluding.
- Aim for high-confidence root cause.
