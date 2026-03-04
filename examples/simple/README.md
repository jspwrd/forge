# Simple Example

The smallest possible Forge project — a single C file with a minimal `forge.toml`.

## Project Structure

```
simple/
├── forge.toml
└── src/
    └── main.c
```

## Usage

```sh
# Debug build (default)
forge build

# Release build
forge build --release

# Run the binary
./bin/hello
```

## What This Demonstrates

- Minimal `forge.toml` with only `[project]`, `[build]`, and `[build.flags]`
- Glob-based source discovery (`src/**/*.c`)
- Debug vs. release flag profiles
