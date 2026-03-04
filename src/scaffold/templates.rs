pub fn forge_toml(name: &str) -> String {
    format!(
        r#"[project]
name = "{name}"
version = "0.1.0"
description = "A forge project"

[build]
compiler = "gcc"
standard = "c11"
static = false
strip = true
sources = ["src/**/*.c"]
includes = ["src"]
link = []

[build.flags]
common = ["-Wall", "-Wextra", "-Werror"]
release = ["-O2", "-DNDEBUG"]
debug = ["-g", "-O0", "-DDEBUG"]

[targets.linux-x86_64]
cc = "gcc"
flags = []
enabled = true

[targets.linux-aarch64]
cc = "aarch64-linux-gnu-gcc"
flags = []
enabled = false

[patch.callback_ip]
sentinel = "AAAA_CALLBACK_IP_AAAA"
type = "ipv4"
size = 20

[patch.callback_port]
sentinel = "BBBB_CALLBACK_PORT_BBBB"
type = "u16"
size = 22

[validate]
no_debug_symbols = true
no_plaintext_strings = ["password", "secret"]
no_compiler_watermarks = true
max_binary_size = "1MB"
no_rpath = true
no_buildpaths = true

[scripts]
deploy = "scripts/deploy.sh"
cleanup = "scripts/cleanup.sh"
listener = "scripts/listener.sh"

[package]
formats = ["raw"]
"#
    )
}

pub fn main_c(name: &str) -> String {
    format!(
        r#"#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

#include "config.h"
#include "comms.h"

int main(void) {{
    const char *ip = CALLBACK_IP;
    const char *port = CALLBACK_PORT;

    printf("[*] {name} starting\\n");
    printf("[*] callback target: %s:%s\\n", ip, port);

    /* Beacon loop */
    while (1) {{
        int result = beacon(ip);
        if (result < 0) {{
            fprintf(stderr, "[-] beacon failed\\n");
        }}
        sleep(30);
    }}

    return 0;
}}
"#
    )
}

pub fn config_h() -> &'static str {
    r#"#ifndef CONFIG_H
#define CONFIG_H

/* Binary patching sentinels — do not modify these values directly.
   Use `forge patch` to replace them with operator values. */
#define CALLBACK_IP   "AAAA_CALLBACK_IP_AAAA"
#define CALLBACK_PORT "BBBB_CALLBACK_PORT_BBBB"

#endif /* CONFIG_H */
"#
}

pub fn comms_h() -> &'static str {
    r#"#ifndef COMMS_H
#define COMMS_H

/* Initialize communications subsystem */
int comms_init(void);

/* Send a beacon to the given IP address */
int beacon(const char *ip);

/* Clean up communications resources */
void comms_shutdown(void);

#endif /* COMMS_H */
"#
}

pub fn comms_c() -> &'static str {
    r#"#include "comms.h"
#include <stdio.h>
#include <string.h>

int comms_init(void) {
    /* TODO: Initialize network resources */
    return 0;
}

int beacon(const char *ip) {
    if (ip == NULL || strlen(ip) == 0) {
        return -1;
    }
    /* TODO: Implement beacon logic */
    printf("[*] beacon -> %s\n", ip);
    return 0;
}

void comms_shutdown(void) {
    /* TODO: Clean up network resources */
}
"#
}

pub fn platform_h() -> &'static str {
    r#"#ifndef PLATFORM_H
#define PLATFORM_H

#if defined(__linux__)
    #define PLATFORM_LINUX 1
#elif defined(__APPLE__)
    #define PLATFORM_MACOS 1
#elif defined(_WIN32)
    #define PLATFORM_WINDOWS 1
#else
    #error "Unsupported platform"
#endif

#endif /* PLATFORM_H */
"#
}

pub fn crypto_h() -> &'static str {
    r#"#ifndef CRYPTO_H
#define CRYPTO_H

#include <stddef.h>

/* XOR encode/decode a buffer in place */
void xor_encode(unsigned char *buf, size_t len, unsigned char key);

#endif /* CRYPTO_H */
"#
}

pub fn deploy_sh() -> &'static str {
    r#"#!/bin/bash
set -euo pipefail

echo "[*] Deploying ${FORGE_PROJECT_NAME} v${FORGE_PROJECT_VERSION}"
echo "[*] Binary: ${FORGE_BIN_DIR}/${FORGE_PROJECT_NAME}"

# TODO: Add deployment logic
echo "[+] Deploy complete"
"#
}

pub fn cleanup_sh() -> &'static str {
    r#"#!/bin/bash
set -euo pipefail

echo "[*] Cleaning up ${FORGE_PROJECT_NAME}"

# TODO: Add cleanup logic
echo "[+] Cleanup complete"
"#
}

pub fn listener_sh() -> &'static str {
    r#"#!/bin/bash
set -euo pipefail

echo "[*] Starting listener for ${FORGE_PROJECT_NAME}"

# TODO: Add listener logic
echo "[+] Listener started"
"#
}

pub fn test_comms_c() -> &'static str {
    r#"#include <stdio.h>
#include <assert.h>
#include "../src/comms.h"

int main(void) {
    printf("[*] Running comms tests...\n");

    /* Test init */
    assert(comms_init() == 0);

    /* Test beacon with valid IP */
    assert(beacon("127.0.0.1") == 0);

    /* Test beacon with NULL */
    assert(beacon(NULL) == -1);

    /* Test beacon with empty string */
    assert(beacon("") == -1);

    comms_shutdown();

    printf("[+] All comms tests passed\n");
    return 0;
}
"#
}

pub fn test_scripts_sh() -> &'static str {
    r#"#!/bin/bash
set -euo pipefail

echo "[*] Running script tests..."

# Test that required env vars are set
if [ -z "${FORGE_PROJECT_NAME:-}" ]; then
    echo "[-] FORGE_PROJECT_NAME not set"
    exit 1
fi

echo "[+] All script tests passed"
"#
}

pub fn gitignore() -> &'static str {
    r#"# Build artifacts
bin/

# Editor files
*.swp
*.swo
*~
.vscode/
.idea/

# OS files
.DS_Store
Thumbs.db
"#
}

pub fn readme(name: &str) -> String {
    format!(
        r#"# {name}

A forge project.

## Building

```bash
forge build
forge build --target linux-x86_64
forge build --all-targets
```

## Patching

```bash
forge patch bin/{name} --callback-ip 10.0.0.1 --callback-port 443
```

## Validating

```bash
forge validate bin/{name}
```

## Testing

```bash
forge test
```
"#
    )
}
