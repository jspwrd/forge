#!/usr/bin/env bash
set -euo pipefail

REPO="jspwrd/forge"
INSTALL_DIR="${FORGE_INSTALL_DIR:-$HOME/.forge/bin}"
BASE_URL="https://github.com/${REPO}/releases"

info() { printf "\033[1;32m%s\033[0m %s\n" "$1" "$2"; }
warn() { printf "\033[1;33mwarning:\033[0m %s\n" "$1"; }
err()  { printf "\033[1;31merror:\033[0m %s\n" "$1" >&2; exit 1; }

detect_os() {
    case "$(uname -s)" in
        Linux*)  echo "unknown-linux-gnu" ;;
        Darwin*) echo "apple-darwin" ;;
        *)       err "unsupported OS: $(uname -s)" ;;
    esac
}

detect_arch() {
    case "$(uname -m)" in
        x86_64|amd64)  echo "x86_64" ;;
        aarch64|arm64) echo "aarch64" ;;
        *)             err "unsupported architecture: $(uname -m)" ;;
    esac
}

get_latest_version() {
    local url="${BASE_URL}/latest"
    if command -v curl &>/dev/null; then
        curl -fsSL -o /dev/null -w "%{url_effective}" "$url" 2>/dev/null | rev | cut -d'/' -f1 | rev
    elif command -v wget &>/dev/null; then
        wget -qO /dev/null --max-redirect=0 "$url" 2>&1 | grep -i "Location" | cut -d'/' -f8 | tr -d '\r'
    else
        err "curl or wget is required"
    fi
}

download() {
    local url="$1" dest="$2"
    if command -v curl &>/dev/null; then
        curl -fsSL -o "$dest" "$url"
    elif command -v wget &>/dev/null; then
        wget -qO "$dest" "$url"
    fi
}

main() {
    local version="${1:-}"
    local os arch target

    os="$(detect_os)"
    arch="$(detect_arch)"
    target="${arch}-${os}"

    info "Detected" "platform ${target}"

    if [ -z "$version" ]; then
        info "Fetching" "latest release..."
        version="$(get_latest_version)"
        if [ -z "$version" ]; then
            err "could not determine latest version. Pass a version explicitly: install.sh v0.1.0"
        fi
    fi

    # Normalize version tag
    if [[ "$version" != v* ]]; then
        version="v${version}"
    fi

    local tarball="forge-${target}.tar.gz"
    local url="${BASE_URL}/download/${version}/${tarball}"

    info "Downloading" "forge ${version} (${target})"

    local tmpdir
    tmpdir="$(mktemp -d)"
    trap 'rm -rf "$tmpdir"' EXIT

    download "$url" "${tmpdir}/${tarball}" || err "download failed — does ${version} exist for ${target}?"

    info "Extracting" "${tarball}"
    tar xzf "${tmpdir}/${tarball}" -C "$tmpdir"

    mkdir -p "$INSTALL_DIR"
    mv "${tmpdir}/forge" "${INSTALL_DIR}/forge"
    chmod +x "${INSTALL_DIR}/forge"

    # Write version file for tracking
    echo "$version" > "${INSTALL_DIR}/../.forge-version"

    info "Installed" "forge ${version} to ${INSTALL_DIR}/forge"

    # Check PATH
    if ! echo "$PATH" | tr ':' '\n' | grep -qx "$INSTALL_DIR"; then
        warn "${INSTALL_DIR} is not in your PATH"
        echo ""
        echo "Add it by appending one of these to your shell profile:"
        echo ""
        echo "  # bash (~/.bashrc)"
        echo "  export PATH=\"${INSTALL_DIR}:\$PATH\""
        echo ""
        echo "  # zsh (~/.zshrc)"
        echo "  export PATH=\"${INSTALL_DIR}:\$PATH\""
        echo ""
        echo "  # fish (~/.config/fish/config.fish)"
        echo "  fish_add_path ${INSTALL_DIR}"
        echo ""
        echo "Then restart your shell or run: source ~/.bashrc"
    fi

    echo ""
    info "Done!" "Run 'forge --version' to verify."
}

main "$@"
