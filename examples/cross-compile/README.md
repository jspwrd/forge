# Cross-Compilation Example

Demonstrates building a single project for multiple architectures using per-target toolchain configuration.

## Project Structure

```
cross-compile/
├── forge.toml
└── src/
    ├── main.c
    ├── platform.h
    └── platform/
        └── linux.c
```

## Targets

| Target           | Compiler                     | Notes                   |
|------------------|------------------------------|-------------------------|
| `linux-x86_64`   | `gcc`                        | Native build            |
| `linux-aarch64`  | `aarch64-linux-gnu-gcc`      | ARM 64-bit              |
| `linux-armhf`    | `arm-linux-gnueabihf-gcc`    | ARM 32-bit hard-float   |
| `linux-mips`     | `mips-linux-gnu-gcc`         | Disabled by default     |

## Usage

```sh
# Build for a single target
forge build --target linux-x86_64

# Build all enabled targets
forge build --all-targets

# Build a specific target in release mode
forge build --target linux-aarch64 --release

# List available targets
forge targets
```

## What This Demonstrates

- Multiple `[targets.<name>]` sections with per-target compilers and flags
- `sysroot` configuration for cross-compilers
- `exclude` patterns to skip platform-specific files
- `enabled = false` to disable a target without removing it
- Static linking and stripping applied globally via `[build]`
- Platform detection in C using preprocessor macros
