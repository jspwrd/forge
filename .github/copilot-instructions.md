# Forge Codebase Patterns

**Always reuse existing code - no redundancy!**

## Tech Stack

- **Language**: Rust (edition 2024)
- **CLI Framework**: clap (derive macros)
- **Config Parsing**: serde + toml (`forge.toml` manifest)
- **Error Handling**: thiserror (`ForgeError`) + anyhow
- **Lint/Format**: clippy, rustfmt (`cargo fmt`, `cargo clippy`)
- **Tests**: cargo test (assert_cmd + predicates for integration)
- **Hashing**: sha2 (SHA256 for incremental builds)

## Anti-Redundancy Rules

- If a function already exists, use it — do NOT create a duplicate.
- Before creating any utility or helper, search for existing implementations first.
- Reuse `src/util/` for shared helpers (filesystem, process, output).

## Source of Truth Locations

### Configuration (`src/manifest/`)

- **ForgeManifest**: all manifest types in `types.rs`, validation in `validation.rs`.
- **NEVER** duplicate manifest parsing or validation logic elsewhere.

### Error Handling (`src/error.rs`)

- **ForgeError**: the single error enum for domain errors.
- Each variant maps to an exit code via `exit_code()`.
- Add new variants here; do not create parallel error types.

### Output (`src/util/output.rs`)

- **OutputConfig**: verbose/quiet/no_color settings.
- All user-facing output goes through these helpers.
- **NEVER** use raw `println!` for user-facing messages.

### CLI (`src/cli/`)

- `mod.rs` — `Cli` struct, `Command` enum, `dispatch` function.
- One `*_cmd.rs` file per command, each with `Args` struct + `execute` function.

### Build System (`src/build/`)

- `compiler.rs` — compiler invocation and flag management.
- `incremental.rs` — SHA256-based caching.
- `context.rs` — build context and state.
- `toolchain.rs` — toolchain detection and selection.

### Binary Operations

- Patching: `src/patch/` (sentinel scanning + writing).
- Validation: `src/validate/` (binary checks + reporting).

## Code Quality

- Rust, strict typing, no `unsafe` without justification
- Keep files under ~500 LOC — extract sub-modules when larger
- Unit tests: `#[cfg(test)]` blocks within source files
- Run `cargo clippy --all-features -- -D warnings` before commits
- Run `cargo fmt --all -- --check` before commits
- Run `cargo test --all-features` before pushing
