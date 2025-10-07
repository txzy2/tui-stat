#!/bin/bash
set -e

# Download and install tui-stat
REPO="txzy2/tuitask"
BINARY="tuitask"

echo "Installing tui-stat..."

# Detect OS and architecture
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

case $OS in
linux*)
  OS="unknown-linux-gnu"
  ;;
darwin*)
  OS="apple-darwin"
  ;;
*)
  echo "Unsupported OS: $OS"
  exit 1
  ;;
esac

case $ARCH in
x86_64 | amd64)
  ARCH="x86_64"
  ;;
aarch64 | arm64)
  ARCH="aarch64"
  ;;
*)
  echo "Unsupported architecture: $ARCH"
  exit 1
  ;;
esac

TARGET="${ARCH}-${OS}"
echo "Detected target: $TARGET"

# Get the latest release
LATEST_TAG=$(curl -s "https://api.github.com/repos/$REPO/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')

if [ -z "$LATEST_TAG" ]; then
  echo "Could not fetch latest release. Make sure you're connected to the internet."
  exit 1
fi

echo "Latest release: $LATEST_TAG"

# Download the binary
DOWNLOAD_URL="https://github.com/$REPO/releases/download/$LATEST_TAG/${BINARY}-${ARCH}-${OS}"

echo "Downloading $DOWNLOAD_URL..."
curl -L -o "$BINARY" "$DOWNLOAD_URL"

# Make it executable
chmod +x "$BINARY"

# Install to /usr/local/bin or ~/.local/bin
if [ "$EUID" -eq 0 ]; then
  INSTALL_DIR="/usr/local/bin"
else
  INSTALL_DIR="$HOME/.local/bin"
  mkdir -p "$INSTALL_DIR"
fi

echo "Installing to $INSTALL_DIR..."
install -m 755 "$BINARY" "$INSTALL_DIR/"

# Cleanup
rm "$BINARY"

echo "Installation complete! Run '$BINARY --help' to get started."

