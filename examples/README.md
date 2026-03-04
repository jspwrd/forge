# Examples

Runnable example projects demonstrating Forge features. Each directory is a self-contained project with its own `forge.toml`.

## Examples

| Example | Description |
|---------|-------------|
| [`simple/`](simple/) | Minimal single-file project — the smallest possible Forge setup |
| [`cross-compile/`](cross-compile/) | Multi-target builds with per-architecture toolchains and flags |
| [`patching/`](patching/) | Full build → patch → validate → package workflow with sentinel markers |

## Running an Example

```sh
cd examples/simple
forge build
./bin/hello
```

## What's Covered

### `simple/`

Start here. Shows the bare minimum `forge.toml` for a C project: source glob, include path, and debug/release flag profiles.

### `cross-compile/`

Builds the same codebase for x86_64, aarch64, and armhf from a single manifest. Demonstrates:

- Per-target `cc`, `sysroot`, and `flags`
- Target-specific `exclude` patterns
- Disabling targets with `enabled = false`
- Static linking and stripping

### `patching/`

The full release lifecycle. Demonstrates:

- Defining sentinel markers in `[patch]` and C headers
- Replacing sentinels with `forge patch`
- Validating binaries against security rules (`[validate]`)
- Deployment scripts via `[scripts]`
- Artifact packaging via `[package]`

## Prerequisites

- **simple**: Only `gcc` needed.
- **cross-compile**: Cross-compiler toolchains (`aarch64-linux-gnu-gcc`, `arm-linux-gnueabihf-gcc`). On Debian/Ubuntu: `sudo apt install gcc-aarch64-linux-gnu gcc-arm-linux-gnueabihf`.
- **patching**: Only `gcc` needed.

## See Also

- [Configuration Reference](../docs/configuration.md)
- [Building & Cross-Compilation](../docs/building.md)
- [Binary Patching & Validation](../docs/patching-and-validation.md)
- [Testing](../docs/testing.md)
