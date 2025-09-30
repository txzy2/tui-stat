#!/bin/bash
#
# Installation script for tui-stat
#

set -e

# Default installation directory
INSTALL_DIR="/usr/local/bin"
BINARY_NAME="tui_stat"
GITHUB_REPO="txzy2/tui-stat"  # Update with your actual GitHub username/repo

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

print_info() {
    echo -e "${GREEN}INFO:${NC} $1"
}

print_warn() {
    echo -e "${YELLOW}WARN:${NC} $1"
}

print_error() {
    echo -e "${RED}ERROR:${NC} $1"
}

# Detect OS and architecture
detect_target() {
    local os=$(uname -s | tr '[:upper:]' '[:lower:]')
    local arch=$(uname -m)

    case $os in
        linux*)
            os="unknown-linux-gnu"
            ;;
        darwin*)
            os="apple-darwin"
            ;;
        *)
            print_error "Unsupported OS: $os"
            exit 1
            ;;
    esac

    case $arch in
        x86_64|amd64)
            arch="x86_64"
            ;;
        aarch64|arm64)
            arch="aarch64"
            ;;
        *)
            print_error "Unsupported architecture: $arch"
            exit 1
            ;;
    esac

    echo "${arch}-${os}"
}

# Download and install the binary
install_binary() {
    local target=$1
    local version=${2:-"latest"}

    print_info "Detecting latest release..."
    local release_url="https://api.github.com/repos/$GITHUB_REPO/releases"
    if [ "$version" = "latest" ]; then
        release_url="${release_url}/latest"
    fi

    local tag_name=$(curl -s "$release_url" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/' | head -n 1)
    if [ -z "$tag_name" ]; then
        print_error "Could not fetch latest release"
        exit 1
    fi

    print_info "Latest release is $tag_name"

    local asset_name="${BINARY_NAME}-${target}"
    local download_url="https://github.com/$GITHUB_REPO/releases/download/$tag_name/${asset_name}"

    print_info "Downloading $BINARY_NAME for $target..."
    curl -L -o "/tmp/$BINARY_NAME" "$download_url"

    print_info "Making $BINARY_NAME executable..."
    chmod +x "/tmp/$BINARY_NAME"

    if [ "$EUID" -eq 0 ]; then
        print_info "Installing $BINARY_NAME to $INSTALL_DIR..."
        install -m 755 "/tmp/$BINARY_NAME" "$INSTALL_DIR/"
    else
        print_warn "Not running as root. Installing to ~/.local/bin instead."
        mkdir -p "$HOME/.local/bin"
        install -m 755 "/tmp/$BINARY_NAME" "$HOME/.local/bin/"
        print_info "Please ensure ~/.local/bin is in your PATH"
    fi

    # Cleanup
    rm "/tmp/$BINARY_NAME"

    print_info "$BINARY_NAME installed successfully!"
    print_info "Run '$BINARY_NAME --help' to get started"
}

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        -h|--help)
            echo "Usage: $0 [OPTIONS]"
            echo "Install tui-stat binary"
            echo ""
            echo "Options:"
            echo "  -h, --help     Show this help message"
            echo "  -v, --version  Specify version to install (default: latest)"
            echo "  -t, --target   Specify target (e.g., x86_64-unknown-linux-gnu)"
            echo "  -d, --dir      Specify installation directory (default: /usr/local/bin)"
            exit 0
            ;;
        -v|--version)
            VERSION="$2"
            shift 2
            ;;
        -t|--target)
            TARGET="$2"
            shift 2
            ;;
        -d|--dir)
            INSTALL_DIR="$2"
            shift 2
            ;;
        *)
            print_error "Unknown option: $1"
            exit 1
            ;;
    esac
done

# Auto-detect target if not specified
if [ -z "$TARGET" ]; then
    TARGET=$(detect_target)
fi

print_info "Installing for target: $TARGET"
if [ -n "$VERSION" ]; then
    print_info "Installing version: $VERSION"
else
    VERSION="latest"
fi

# Check if curl is available
if ! command -v curl &> /dev/null; then
    print_error "curl is required but not installed"
    exit 1
fi

# Install the binary
install_binary "$TARGET" "$VERSION"