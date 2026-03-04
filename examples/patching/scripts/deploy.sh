#!/bin/bash
set -euo pipefail

echo "[*] Deploying ${FORGE_PROJECT_NAME} v${FORGE_PROJECT_VERSION}"
echo "[*] Binary: ${FORGE_BIN_DIR}/${FORGE_PROJECT_NAME}"

# Example: copy the binary to a staging directory
# cp "${FORGE_BIN_DIR}/${FORGE_PROJECT_NAME}" /opt/staging/

echo "[+] Deploy complete"
