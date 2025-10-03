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
        print_error "Could not fetch latest release. Make sure you have published a release on GitHub."
        print_info "Visit https://github.com/$GITHUB_REPO/releases to create a release."
        exit 1
    fi

    print_info "Latest release is $tag_name"

    # Determine the archive extension based on target
    local archive_ext
    if [[ "$target" == *"-pc-windows-"* ]]; then
        archive_ext=".zip"
    else
        archive_ext=".tar.gz"
    fi
    
    local asset_name="${BINARY_NAME}-${target}${archive_ext}"
    local download_url="https://github.com/$GITHUB_REPO/releases/download/$tag_name/${asset_name}"

    print_info "Downloading $BINARY_NAME for $target..."
    curl -L -o "/tmp/$BINARY_NAME${archive_ext}" "$download_url"
    
    # Extract the binary
    print_info "Extracting $BINARY_NAME..."
    if [[ "$archive_ext" == ".tar.gz" ]]; then
        tar -xzf "/tmp/$BINARY_NAME${archive_ext}" -C /tmp
    else
        # For .zip files, need to install unzip if not available
        if ! command -v unzip &> /dev/null; then
            print_error "unzip is required but not installed"
            exit 1
        fi
        unzip -o "/tmp/$BINARY_NAME${archive_ext}" -d /tmp
    fi

    # The extracted binary might be in a subdirectory, so find it
    EXTRACTED_BINARY=$(find /tmp -name "$BINARY_NAME" -type f -executable | head -n 1)
    if [ -z "$EXTRACTED_BINARY" ]; then
        # If not found, try to look for any executable file in the temp directory
        print_error "Could not find extracted binary"
        ls -la /tmp/
        exit 1
    fi

    print_info "Making $BINARY_NAME executable..."
    chmod +x "$EXTRACTED_BINARY"

    if [ "$EUID" -eq 0 ]; then
        print_info "Installing $BINARY_NAME to $INSTALL_DIR..."
        install -m 755 "$EXTRACTED_BINARY" "$INSTALL_DIR/"
    else
        print_warn "Not running as root. Installing to ~/.local/bin instead."
        mkdir -p "$HOME/.local/bin"
        install -m 755 "$EXTRACTED_BINARY" "$HOME/.local/bin/"
        print_info "Please ensure ~/.local/bin is in your PATH"
    fi

    # Cleanup
    rm "/tmp/$BINARY_NAME${archive_ext}"  # Remove the archive file
    rm "$EXTRACTED_BINARY"  # Remove the extracted binary after installation

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