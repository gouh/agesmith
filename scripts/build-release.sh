#!/bin/bash
# Build script for multi-platform binaries using cross (Docker)

set -e

BINARY_NAME="agesmith"
VERSION=$(grep '^version' Cargo.toml | head -1 | cut -d'"' -f2)
BUILD_DIR="dist"
TARGETS="x86_64-unknown-linux-gnu x86_64-apple-darwin aarch64-apple-darwin"

echo "Building $BINARY_NAME v$VERSION for all platforms..."
mkdir -p "$BUILD_DIR"

for target in $TARGETS; do
    echo "Building for $target..."
    cross build --release --target "$target"
    
    if [ $? -eq 0 ]; then
        cp "target/$target/release/$BINARY_NAME" "$BUILD_DIR/${BINARY_NAME}-${VERSION}-${target}" 2>/dev/null || true
    fi
done

echo ""
echo "Binaries created in $BUILD_DIR/"
ls -lh "$BUILD_DIR/"

echo ""
echo "Generating checksums..."
cd "$BUILD_DIR" && shasum -a 256 agesmith-* > checksums.txt
cd ..

echo ""
echo "=========================================="
echo "Release build completed!"
echo "Version: $VERSION"
echo "=========================================="
echo ""
echo "Binaries available in $BUILD_DIR/:"
ls -lh "$BUILD_DIR/"
echo ""
echo "Checksums saved to $BUILD_DIR/checksums.txt"
cat "$BUILD_DIR/checksums.txt"
