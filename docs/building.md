# Building & Cross-Compilation

This guide covers how Forge compiles C/C++ projects, including native and cross-compilation workflows.

## Native Builds

Build with the default compiler (gcc):

```sh
forge build
```

Build in release mode (applies `[build.flags.release]`):

```sh
forge build --release
```

Force a full rebuild, ignoring the incremental cache:

```sh
forge build --force
```

## Build Output

All build artifacts go to the `bin/` directory:

```
bin/
├── myproject           # native binary
├── myproject-aarch64   # target-specific binary
├── obj/
│   ├── native/         # object files for native build
│   └── aarch64/        # object files for target build
├── build.log           # timestamped build log
└── .forge-cache        # incremental build cache
```

## Incremental Builds

Forge uses SHA256 hashing to detect changes. For each source file, it computes a cache key from:

- File contents
- Compiler flags

If the cache key matches the previous build, the file is skipped. Use `--force` to bypass the cache.

The cache is stored in `bin/.forge-cache` and is automatically created and updated on each build.

## Cross-Compilation

### Defining Targets

Add targets in `forge.toml`:

```toml
[targets.aarch64-linux]
cc = "aarch64-linux-gnu-gcc"
sysroot = "/usr/aarch64-linux-gnu"
flags = ["-march=armv8-a"]

[targets.x86_64-musl]
cc = "x86_64-linux-musl-gcc"
flags = ["-static"]
link = ["c"]
```

### Building a Single Target

```sh
forge build --target aarch64-linux
```

The output binary will be named `<project>-<target>` (e.g., `bin/myproject-aarch64-linux`).

### Building All Targets

```sh
forge build --all-targets
```

This iterates over every target in `[targets]`, skipping any with `enabled = false`. Each target compiles independently with its own object directory.

### Disabling a Target

```toml
[targets.mips-linux]
cc = "mips-linux-gnu-gcc"
enabled = false
```

Disabled targets are skipped during `--all-targets` and cannot be built with `--target`.

### Target-Specific Sources and Exclusions

Targets can add or exclude source files:

```toml
[targets.aarch64-linux]
cc = "aarch64-linux-gnu-gcc"
sources = ["platform/linux_arm64/*.c"]
exclude = ["src/platform/windows_*.c"]
```

## Build Flags

Flags are applied in this order:

1. `[build.flags.common]` — always included
2. `[build.flags.debug]` or `[build.flags.release]` — based on build mode
3. `[targets.<name>.flags]` — target-specific additions

## Compiler Selection

The compiler is resolved in this order:

1. Target's `cc` field (if building for a target)
2. `[build].compiler` field
3. Default: `gcc`

Forge validates that the selected compiler exists in `PATH` before building.

## Static Linking and Stripping

```toml
[build]
static = true   # pass -static to the linker
strip = true     # run strip on the output binary
```

Stripping removes symbol tables, reducing binary size. This is applied as a post-link step.

## Build Logs

Every successful build appends a line to `bin/build.log`:

```
[1709571234] built myproject (5 compiled, 3 skipped)
```

## CLI Reference

```
forge build [OPTIONS]

Options:
    --target <NAME>    Build for a specific target
    --all-targets      Build all enabled targets
    --release          Build in release mode
    --force            Force rebuild, ignoring cache
-v, --verbose          Show detailed output
-q, --quiet            Suppress non-error output
```
