# Binary Patching Example

Demonstrates the full build-patch-validate-package workflow using sentinel markers.

## Project Structure

```
patching/
├── forge.toml
├── scripts/
│   └── deploy.sh
└── src/
    ├── config.h      # sentinel definitions
    └── main.c
```

## Workflow

### 1. Build a release binary

```sh
forge build --release
```

### 2. Patch with operator values

Replace sentinel markers with real deployment values:

```sh
forge patch bin/agent \
    --callback-ip 10.0.0.1 \
    --callback-port 443 \
    --sleep-seconds 30
```

### 3. Validate the binary

Run security checks defined in `[validate]`:

```sh
forge validate bin/agent
```

Expected output:

```
  Validating  bin/agent
      ✓ debug symbols
      ✓ plaintext strings
      ✓ compiler watermarks
      ✓ binary size (< 1MB)
      ✓ unpatched sentinels
      ✓ RPATH/RUNPATH
      ✓ build path leakage
```

### 4. Package for distribution

```sh
forge package --format raw
```

### 5. Run deployment script

```sh
forge scripts deploy
```

## How Patching Works

1. Sentinel strings (e.g. `AAAA_CALLBACK_IP_AAAA`) are compiled into the binary as constant data.
2. `forge patch` locates each sentinel's byte pattern and overwrites it with the provided value.
3. The binary size is verified unchanged after patching.
4. `forge validate` confirms no unpatched sentinels remain.

## What This Demonstrates

- `[patch.<field>]` configuration with `string`, `u16`, and `u32` types
- `[validate]` checks: debug symbols, plaintext strings, watermarks, size, RPATH, build paths
- `[scripts]` integration for deployment automation
- `[package]` for artifact packaging
- The end-to-end release workflow: build → patch → validate → package → deploy
