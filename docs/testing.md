# Testing

Forge includes a built-in test runner that supports C unit tests and shell script validation.

## Running Tests

```sh
forge test
```

This runs all test stages in order:

1. **Shellcheck** — validates shell scripts defined in `[scripts]`
2. **C tests** — discovers, compiles, and runs C test files

## C Tests

### Test Discovery

Forge automatically discovers C test files in the `tests/` directory matching these patterns:

- `tests/test_*.c`
- `tests/*_test.c`

No configuration is needed — just place test files in the `tests/` directory.

### Compilation

Each test file is compiled independently with:

- All `.c` files from `src/` (except `main.c`) linked in
- The `src/` directory added as an include path (`-I src/`)
- The compiler specified in `[build].compiler` (default: `gcc`)

Test binaries are placed in `bin/tests/`.

### Writing Tests

A test is any C program with a `main()` function that returns `0` on success or non-zero on failure.

```c
// tests/test_comms.c
#include <stdio.h>
#include <assert.h>
#include "comms.h"

int main(void) {
    // Test initialization
    assert(comms_init() == 0);

    // Test send/receive
    char buf[64];
    assert(comms_encode("hello", buf, sizeof(buf)) > 0);

    printf("all comms tests passed\n");
    return 0;
}
```

### Test Output

Tests display their status as they run:

```
  Compiling  test 'test_comms'
    Running  test 'test_comms'
     Passed  test 'test_comms'
```

On failure, both stdout and stderr from the test binary are printed.

## Shell Script Validation

If your `forge.toml` has a `[scripts]` section, `forge test` runs [shellcheck](https://www.shellcheck.net/) on each referenced script.

```toml
[scripts]
deploy = "scripts/deploy.sh"
cleanup = "scripts/cleanup.sh"
```

Shellcheck catches common scripting issues:
- Unquoted variables
- Unused variables
- Incorrect use of `test`/`[` constructs
- POSIX compliance issues

If `shellcheck` is not installed, Forge prints a warning and continues.

## Exit Codes

| Code | Meaning                    |
|------|----------------------------|
| `0`  | All tests passed           |
| `1`  | One or more tests failed   |

## CLI Reference

```
forge test [OPTIONS]

Options:
-v, --verbose    Show detailed output including test stdout
-q, --quiet      Suppress non-error output
```
