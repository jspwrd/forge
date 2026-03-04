# Forge

A build system for C/C++ cross-compilation, written in Rust.

Forge simplifies compiling, patching, validating, and packaging C/C++ projects across multiple targets with a single `forge.toml` manifest.

## Features

- **Cross-compilation** — define multiple targets with per-target compilers, flags, and sysroots
- **Incremental builds** — SHA256-based caching skips unchanged source files
- **Binary patching** — inject operator-specific values into compiled binaries via sentinel markers
- **Binary validation** — check for debug symbols, plaintext strings, compiler watermarks, RPATH leaks, and more
- **Test runner** — auto-discovers and compiles C test files; runs shellcheck on project scripts
- **Packaging** — produce raw, DEB, and DMG artifacts
- **Certificate management** — generate and manage certificates for your project
- **Project scaffolding** — `forge new` and `forge init` create ready-to-build project structures

## Installation

### One-line install (Linux / macOS)

```sh
curl -fsSL https://raw.githubusercontent.com/jspwrd/forge/main/install.sh | bash
```

Or install a specific version:

```sh
curl -fsSL https://raw.githubusercontent.com/jspwrd/forge/main/install.sh | bash -s -- v0.1.0
```

The installer places the binary in `~/.forge/bin` and prints instructions for adding it to your `PATH`.

### Install from source

```sh
cargo install --path .
```

### Updating

```sh
forge update
```

### Uninstalling

```sh
forge uninstall
```

## Quick Start

### Create a project

```sh
forge new myproject
cd myproject
```

### Build

```sh
forge build
```

### Run tests

```sh
forge test
```

## Commands

| Command    | Description                                  |
|------------|----------------------------------------------|
| `new`      | Create a new forge project                   |
| `init`     | Initialize forge in an existing directory    |
| `build`    | Build the project                            |
| `patch`    | Patch compiled binaries with operator values |
| `validate` | Validate compiled binaries                   |
| `clean`    | Clean build artifacts                        |
| `test`     | Run tests                                    |
| `scripts`  | Run project scripts                          |
| `targets`  | List available build targets                 |
| `config`   | Show or manage configuration                 |
| `cert`     | Manage certificates                          |
| `package`  | Package build artifacts                      |
| `update`   | Update forge to the latest version           |
| `uninstall`| Uninstall forge from your system             |

## Global Options

```
-v, --verbose    Enable verbose output
-q, --quiet      Suppress non-error output
    --no-color   Disable colored output
```

## Configuration

Projects are configured via `forge.toml` at the project root. See [docs/configuration.md](docs/configuration.md) for the full reference.

### Minimal example

```toml
[project]
name = "hello"
version = "0.1.0"

[build]
sources = ["src/*.c"]
```

### Cross-compilation example

```toml
[project]
name = "hello"
version = "0.1.0"

[build]
sources = ["src/*.c"]
includes = ["include"]
standard = "c11"

[build.flags]
common = ["-Wall", "-Wextra"]
release = ["-O2"]
debug = ["-g", "-DDEBUG"]

[targets.aarch64-linux]
cc = "aarch64-linux-gnu-gcc"
sysroot = "/usr/aarch64-linux-gnu"
flags = ["-march=armv8-a"]

[targets.x86_64-linux]
cc = "x86_64-linux-gnu-gcc"
```

## Docker

```sh
# Build the image
docker build -t forge .

# Use forge inside a container
docker run --rm -v "$(pwd)":/project forge build
```

## Documentation

See the [docs/](docs/) directory for detailed documentation:

- [Configuration Reference](docs/configuration.md)
- [Building & Cross-Compilation](docs/building.md)
- [Testing](docs/testing.md)
- [Binary Patching & Validation](docs/patching-and-validation.md)

## License

See [LICENSE](LICENSE) for details.
