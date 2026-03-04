# Configuration Reference

Forge projects are configured through a `forge.toml` file at the project root. This document covers every section and field.

## `[project]`

Required. Defines project metadata.

| Field         | Type   | Required | Description                  |
|---------------|--------|----------|------------------------------|
| `name`        | string | yes      | Project name                 |
| `version`     | string | yes      | Semantic version             |
| `description` | string | no       | Short project description    |

```toml
[project]
name = "myapp"
version = "1.0.0"
description = "A network utility"
```

## `[build]`

Optional. Controls the compilation pipeline.

| Field      | Type     | Default | Description                        |
|------------|----------|---------|------------------------------------|
| `compiler` | string   | `gcc`   | Compiler executable                |
| `standard` | string   | —       | C standard (`c99`, `c11`, `c17`)   |
| `static`   | bool     | `false` | Enable static linking              |
| `strip`    | bool     | `false` | Strip symbols from output binary   |
| `sources`  | string[] | `[]`    | Glob patterns for source files     |
| `includes` | string[] | `[]`    | Include directories                |
| `link`     | string[] | `[]`    | Libraries to link against          |

```toml
[build]
compiler = "gcc"
standard = "c11"
static = true
strip = true
sources = ["src/**/*.c"]
includes = ["include", "vendor/openssl/include"]
link = ["pthread", "ssl", "crypto"]
```

### `[build.flags]`

Compiler flags, split by build profile.

| Field     | Type     | Description                              |
|-----------|----------|------------------------------------------|
| `common`  | string[] | Always applied                           |
| `release` | string[] | Applied in release mode (`--release`)    |
| `debug`   | string[] | Applied in debug mode (default)          |

```toml
[build.flags]
common = ["-Wall", "-Wextra", "-Werror"]
release = ["-O2", "-DNDEBUG"]
debug = ["-g", "-O0", "-DDEBUG"]
```

## `[targets.<name>]`

Optional. Define cross-compilation targets. Each target inherits the base `[build]` config and can override specific fields.

| Field     | Type     | Default | Description                           |
|-----------|----------|---------|---------------------------------------|
| `cc`      | string   | —       | C compiler override                   |
| `cxx`     | string   | —       | C++ compiler override                 |
| `flags`   | string[] | `[]`    | Additional compiler flags             |
| `sources` | string[] | `[]`    | Additional source files               |
| `exclude` | string[] | `[]`    | Glob patterns to exclude              |
| `link`    | string[] | `[]`    | Additional libraries                  |
| `sysroot` | string   | —       | Sysroot for cross-compilation         |
| `enabled` | bool     | `true`  | Whether this target is active         |

```toml
[targets.aarch64-linux]
cc = "aarch64-linux-gnu-gcc"
sysroot = "/usr/aarch64-linux-gnu"
flags = ["-march=armv8-a"]

[targets.x86_64-linux]
cc = "x86_64-linux-gnu-gcc"
enabled = true

[targets.mips-linux]
cc = "mips-linux-gnu-gcc"
enabled = false  # skip this target during --all-targets
```

## `[dependencies.<name>]`

Optional. Declare external dependencies.

| Field         | Type   | Default | Description                    |
|---------------|--------|---------|--------------------------------|
| `path`        | string | —       | Path to dependency source      |
| `version`     | string | —       | Version constraint             |
| `header_only` | bool   | `false` | Header-only library flag       |

```toml
[dependencies.mbedtls]
path = "vendor/mbedtls"
version = "3.0"
header_only = false
```

## `[patch.<field_name>]`

Optional. Define binary patching sentinels. Each field defines a placeholder in the compiled binary that can be replaced with operator-specific values at deploy time.

| Field      | Type   | Description                                   |
|------------|--------|-----------------------------------------------|
| `sentinel` | string | Byte pattern to locate in the binary          |
| `type`     | string | Value type (e.g., `string`, `u16`, `u32`)     |
| `size`     | int    | Size in bytes of the field                     |

```toml
[patch.callback_ip]
sentinel = "AAAA_CALLBACK_IP_AAAA"
type = "string"
size = 64

[patch.callback_port]
sentinel = "BBBB_PORT_BBBB"
type = "u16"
size = 2
```

Usage:

```sh
forge patch bin/myapp --callback-ip 10.0.0.1 --callback-port 8443
```

## `[validate]`

Optional. Define validation checks to run against compiled binaries.

| Field                   | Type     | Default | Description                              |
|-------------------------|----------|---------|------------------------------------------|
| `no_debug_symbols`      | bool     | `false` | Fail if debug symbols present            |
| `no_plaintext_strings`  | string[] | `[]`    | Fail if these strings appear in binary   |
| `no_compiler_watermarks`| bool     | `false` | Fail if compiler watermarks detected     |
| `max_binary_size`       | string   | —       | Max size (e.g., `"1MB"`, `"500KB"`)      |
| `no_rpath`              | bool     | `false` | Fail if RPATH/RUNPATH is set             |
| `no_buildpaths`         | bool     | `false` | Fail if build paths leak into binary     |

```toml
[validate]
no_debug_symbols = true
no_plaintext_strings = ["password", "secret_key"]
no_compiler_watermarks = true
max_binary_size = "2MB"
no_rpath = true
no_buildpaths = true
```

## `[scripts.<name>]`

Optional. Named shell scripts that can be run via `forge scripts`.

```toml
[scripts]
deploy = "scripts/deploy.sh"
cleanup = "scripts/cleanup.sh"
listener = "scripts/listener.sh"
```

Usage:

```sh
forge scripts deploy
forge scripts --list
```

## `[package]`

Optional. Packaging configuration.

| Field     | Type     | Default | Description                           |
|-----------|----------|---------|---------------------------------------|
| `formats` | string[] | `[]`    | Supported formats: `raw`, `deb`, `dmg`|

```toml
[package]
formats = ["raw", "deb"]
```

## Full Example

```toml
[project]
name = "beacon"
version = "2.1.0"
description = "Network beacon agent"

[build]
compiler = "gcc"
standard = "c11"
static = true
strip = true
sources = ["src/**/*.c"]
includes = ["include", "vendor/mbedtls/include"]
link = ["pthread", "mbedtls", "mbedcrypto"]

[build.flags]
common = ["-Wall", "-Wextra", "-Werror", "-fPIC"]
release = ["-O2", "-DNDEBUG", "-fstack-protector-strong"]
debug = ["-g", "-O0", "-DDEBUG"]

[targets.aarch64-linux]
cc = "aarch64-linux-gnu-gcc"
sysroot = "/usr/aarch64-linux-gnu"
flags = ["-march=armv8-a"]

[targets.x86_64-linux]
cc = "gcc"

[dependencies.mbedtls]
path = "vendor/mbedtls"
version = "3.0"

[patch.callback_ip]
sentinel = "AAAA_CALLBACK_IP_AAAA"
type = "string"
size = 64

[patch.callback_port]
sentinel = "BBBB_PORT_BBBB"
type = "u16"
size = 2

[validate]
no_debug_symbols = true
no_compiler_watermarks = true
no_rpath = true
no_buildpaths = true
max_binary_size = "2MB"

[scripts]
deploy = "scripts/deploy.sh"
cleanup = "scripts/cleanup.sh"

[package]
formats = ["raw", "deb"]
```
