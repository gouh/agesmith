#!/bin/bash
# AgeSmith Installation Script
# Detects OS/architecture and installs the appropriate binary

set -e

REPO="gouh/agesmith"
INSTALL_DIR="${INSTALL_DIR:-$HOME/.local/bin}"
BINARY_NAME="agesmith"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}  AgeSmith Installer${NC}"
echo -e "${BLUE}========================================${NC}"
echo ""

# Detect OS and architecture
detect_platform() {
    local os=$(uname -s | tr '[:upper:]' '[:lower:]')
    local arch=$(uname -m)
    
    case "$os" in
        linux*)
            OS="linux"
            ;;
        darwin*)
            OS="darwin"
            ;;
        msys*|mingw*|cygwin*)
            OS="windows"
            ;;
        *)
            echo -e "${RED}Error: Unsupported OS: $os${NC}"
            exit 1
            ;;
    esac
    
    case "$arch" in
        x86_64|amd64)
            ARCH="x86_64"
            ;;
        aarch64|arm64)
            ARCH="aarch64"
            ;;
        *)
            echo -e "${RED}Error: Unsupported architecture: $arch${NC}"
            exit 1
            ;;
    esac
    
    # Determine target triple
    if [ "$OS" = "linux" ]; then
        TARGET="x86_64-unknown-linux-gnu"
    elif [ "$OS" = "darwin" ]; then
        if [ "$ARCH" = "aarch64" ]; then
            TARGET="aarch64-apple-darwin"
        else
            TARGET="x86_64-apple-darwin"
        fi
    elif [ "$OS" = "windows" ]; then
        TARGET="x86_64-pc-windows-gnu"
        BINARY_NAME="agesmith.exe"
    fi
    
    echo -e "${GREEN}✓${NC} Detected platform: ${YELLOW}$OS ($ARCH)${NC}"
    echo -e "${GREEN}✓${NC} Target: ${YELLOW}$TARGET${NC}"
}

# Get latest version from GitHub releases
get_latest_version() {
    echo ""
    echo "Fetching latest version..."
    
    # Try to get from GitHub API
    VERSION=$(curl -s "https://api.github.com/repos/$REPO/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')
    
    if [ -z "$VERSION" ]; then
        echo -e "${RED}Error: Could not fetch latest version${NC}"
        echo "Please specify version manually: curl -sSL https://raw.githubusercontent.com/$REPO/main/scripts/install.sh | VERSION=v0.1.0 bash"
        exit 1
    fi
    
    echo -e "${GREEN}✓${NC} Latest version: ${YELLOW}$VERSION${NC}"
}

# Download binary
download_binary() {
    echo ""
    echo "Downloading binary..."
    
    local download_url="https://github.com/$REPO/releases/download/$VERSION/${BINARY_NAME}-${VERSION#v}-$TARGET"
    local checksum_url="https://github.com/$REPO/releases/download/$VERSION/checksums.txt"
    
    local tmp_dir=$(mktemp -d)
    local binary_path="$tmp_dir/$BINARY_NAME"
    local checksum_path="$tmp_dir/checksums.txt"
    
    # Download binary
    if ! curl -sSL "$download_url" -o "$binary_path"; then
        echo -e "${RED}Error: Failed to download binary${NC}"
        echo "URL: $download_url"
        rm -rf "$tmp_dir"
        exit 1
    fi
    
    echo -e "${GREEN}✓${NC} Binary downloaded"
    
    # Download checksums
    if curl -sSL "$checksum_url" -o "$checksum_path" 2>/dev/null; then
        echo -e "${GREEN}✓${NC} Checksums downloaded"
        
        # Verify checksum
        echo ""
        echo "Verifying checksum..."
        
        local expected_checksum=$(grep "${BINARY_NAME}-${VERSION#v}-$TARGET" "$checksum_path" | awk '{print $1}')
        local actual_checksum=$(shasum -a 256 "$binary_path" | awk '{print $1}')
        
        if [ "$expected_checksum" = "$actual_checksum" ]; then
            echo -e "${GREEN}✓${NC} Checksum verified"
        else
            echo -e "${RED}✗${NC} Checksum mismatch!"
            echo "Expected: $expected_checksum"
            echo "Got:      $actual_checksum"
            rm -rf "$tmp_dir"
            exit 1
        fi
    else
        echo -e "${YELLOW}⚠${NC}  Could not download checksums (skipping verification)"
    fi
    
    # Install binary
    echo ""
    echo "Installing to $INSTALL_DIR..."
    
    mkdir -p "$INSTALL_DIR"
    chmod +x "$binary_path"
    mv "$binary_path" "$INSTALL_DIR/$BINARY_NAME"
    
    rm -rf "$tmp_dir"
    
    echo -e "${GREEN}✓${NC} Installed successfully"
}

# Check if install dir is in PATH
check_path() {
    echo ""
    if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
        echo -e "${YELLOW}⚠${NC}  Warning: $INSTALL_DIR is not in your PATH"
        echo ""
        echo "Add it to your PATH by adding this to your shell config:"
        echo ""
        echo -e "  ${BLUE}export PATH=\"\$PATH:$INSTALL_DIR\"${NC}"
        echo ""
    fi
}

# Show verification instructions
show_verification() {
    echo ""
    echo -e "${BLUE}========================================${NC}"
    echo -e "${GREEN}Installation complete!${NC}"
    echo -e "${BLUE}========================================${NC}"
    echo ""
    echo "Run: ${YELLOW}$BINARY_NAME --version${NC}"
    echo ""
    echo "To verify the installation checksum:"
    echo -e "  ${BLUE}shasum -a 256 $INSTALL_DIR/$BINARY_NAME${NC}"
    echo ""
    echo "Compare with checksums at:"
    echo -e "  ${BLUE}https://github.com/$REPO/releases/download/$VERSION/checksums.txt${NC}"
    echo ""
}

# Main installation flow
main() {
    detect_platform
    get_latest_version
    download_binary
    check_path
    show_verification
}

main
