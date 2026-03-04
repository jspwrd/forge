# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.1] - 2026-03-03

### Fixed
- Resolve `tmpdir` unbound variable error in install script

## [1.0.0] - 2026-03-04

### Added
- Cross-compilation with per-target toolchains via `forge.toml` manifest
- SHA256-based incremental builds to skip unchanged source files
- Binary patching via sentinel markers for operator-specific values
- Binary validation checks: debug symbols, plaintext strings, compiler watermarks, RPATH, build paths
- Automated C test discovery and compilation
- Shellcheck integration for project scripts
- Multi-format packaging: raw, DEB, DMG
- Certificate management via OpenSSL
- Project scaffolding with `forge new` and `forge init`
- Self-update (`forge update`) and self-uninstall (`forge uninstall`) commands
- One-line installer script for Linux and macOS
- Docker support with multi-stage build
- Per-target compiler, flags, sysroot, and source configuration
- Secure build artifact wiping
- Verbose, quiet, and no-color output modes

[1.0.1]: https://github.com/jspwrd/forge/compare/v1.0.0...v1.0.1
[1.0.0]: https://github.com/jspwrd/forge/releases/tag/v1.0.0
