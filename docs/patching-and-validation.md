# Binary Patching & Validation

Forge supports post-compilation binary patching and validation — injecting operator-specific values into compiled binaries and verifying binaries meet security requirements.

## Binary Patching

### Overview

Patching replaces sentinel markers in a compiled binary with runtime values. This allows building a single binary and customizing it per deployment without recompilation.

### Defining Patch Fields

In `forge.toml`, define sentinel markers:

```toml
[patch.callback_ip]
sentinel = "AAAA_CALLBACK_IP_AAAA"
type = "string"
size = 64

[patch.callback_port]
sentinel = "BBBB_PORT_BBBB"
type = "u16"
size = 2

[patch.sleep_interval]
sentinel = "CCCC_SLEEP_CCCC"
type = "u32"
size = 4
```

### Using Sentinels in C Code

Place sentinel values in your source as static buffers:

```c
// config.h
#define CALLBACK_IP   "AAAA_CALLBACK_IP_AAAA"
#define CALLBACK_PORT 0x4242  // sentinel bytes
```

The sentinel string must appear exactly once in the compiled binary.

### Applying Patches

```sh
forge patch bin/myapp --callback-ip 10.0.0.1 --callback-port 8443
```

Field names are converted from `forge.toml` format (underscores) to CLI format (hyphens) automatically. Both `--field=value` and `--field value` formats are accepted.

### How It Works

1. Forge reads the binary into memory
2. For each field, it locates the sentinel bytes
3. The value is encoded according to the field's `type`
4. The encoded value replaces the sentinel at the found offset
5. The patched binary is written atomically (write to temp file, then rename)
6. Binary size is verified unchanged after patching

### Supported Types

| Type     | Description              |
|----------|--------------------------|
| `string` | UTF-8 string, null-padded to `size` bytes |
| `u16`    | 16-bit unsigned integer  |
| `u32`    | 32-bit unsigned integer  |

## Binary Validation

### Overview

Validation checks compiled binaries against a set of security and quality rules defined in `forge.toml`.

### Running Validation

```sh
forge validate bin/myapp
```

### Available Checks

#### Debug Symbols

```toml
[validate]
no_debug_symbols = true
```

Fails if the binary contains debug symbols (detected via the `file` command).

#### Plaintext Strings

```toml
[validate]
no_plaintext_strings = ["password", "secret_key", "/home/"]
```

Scans the binary for specified strings. Fails if any are found.

#### Compiler Watermarks

```toml
[validate]
no_compiler_watermarks = true
```

Checks for compiler-identifying strings (e.g., `GCC:`, `clang version`).

#### Binary Size

```toml
[validate]
max_binary_size = "2MB"
```

Fails if the binary exceeds the specified size. Supports `KB`, `MB` suffixes.

#### Unpatched Sentinels

Automatically checks all `[patch]` fields. Fails if any sentinel strings are still present in the binary, indicating the binary was not patched before validation.

#### RPATH/RUNPATH

```toml
[validate]
no_rpath = true
```

Fails if the binary has RPATH or RUNPATH set, which could leak filesystem paths.

#### Build Path Leakage

```toml
[validate]
no_buildpaths = true
```

Checks for build directory paths embedded in the binary.

### Validation Report

Forge outputs a pass/fail report for each check:

```
  Validating  bin/myapp
      ✓ debug symbols
      ✓ plaintext strings
      ✗ compiler watermarks — found "GCC: (Ubuntu 11.4.0)"
      ✓ binary size
      ✓ unpatched sentinels
      ✓ RPATH/RUNPATH
      ✓ build path leakage
```

### Typical Workflow

A typical release workflow combines build, patch, and validate:

```sh
# Build release binary
forge build --release

# Patch with operator values
forge patch bin/myapp --callback-ip 10.0.0.1 --callback-port 443

# Validate the patched binary
forge validate bin/myapp

# Package for distribution
forge package --format deb
```

## CLI Reference

```
forge patch <BINARY> [-- --field-name value ...]

Arguments:
    <BINARY>    Path to the binary to patch

Options:
-v, --verbose    Show detailed output
-q, --quiet      Suppress non-error output
```

```
forge validate <BINARY>

Arguments:
    <BINARY>    Path to the binary to validate

Options:
-v, --verbose    Show detailed output
-q, --quiet      Suppress non-error output
```
