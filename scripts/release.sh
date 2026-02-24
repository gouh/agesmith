#!/bin/bash
# Complete release script: bump version, build, checksum, and tag

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# Check if cargo-edit is installed
if ! command -v cargo-set-version &> /dev/null; then
    echo -e "${YELLOW}Installing cargo-edit...${NC}"
    cargo install cargo-edit
fi

# Get bump type (patch, minor, major)
BUMP_TYPE=${1:-patch}

if [[ ! "$BUMP_TYPE" =~ ^(patch|minor|major)$ ]]; then
    echo -e "${RED}Error: Invalid bump type. Use: patch, minor, or major${NC}"
    exit 1
fi

echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}  AgeSmith Release Process${NC}"
echo -e "${BLUE}========================================${NC}"
echo ""

# Get current version
CURRENT_VERSION=$(grep '^version' Cargo.toml | head -1 | cut -d'"' -f2)
echo -e "Current version: ${YELLOW}$CURRENT_VERSION${NC}"

# Bump version
echo ""
echo -e "${BLUE}Step 1: Bumping version ($BUMP_TYPE)...${NC}"
cargo set-version --bump $BUMP_TYPE

NEW_VERSION=$(grep '^version' Cargo.toml | head -1 | cut -d'"' -f2)
echo -e "${GREEN}✓${NC} New version: ${YELLOW}$NEW_VERSION${NC}"

# Build binaries
echo ""
echo -e "${BLUE}Step 2: Building binaries for all platforms...${NC}"
make clean
./scripts/build-release.sh

# Commit version bump
echo ""
echo -e "${BLUE}Step 3: Committing version bump...${NC}"
git add Cargo.toml
if [ -f Cargo.lock ]; then
    git add -f Cargo.lock 2>/dev/null || true
fi
git commit -m "chore: Bump version to $NEW_VERSION"
echo -e "${GREEN}✓${NC} Version bump committed"

# Create and push tag
echo ""
echo -e "${BLUE}Step 4: Creating and pushing tag...${NC}"
git tag "v$NEW_VERSION"
echo -e "${GREEN}✓${NC} Tag v$NEW_VERSION created"

# Push changes
echo ""
echo -e "${BLUE}Step 5: Pushing to remote...${NC}"
git push
git push origin "v$NEW_VERSION"
echo -e "${GREEN}✓${NC} Changes and tag pushed"

# Summary
echo ""
echo -e "${BLUE}========================================${NC}"
echo -e "${GREEN}Release v$NEW_VERSION completed!${NC}"
echo -e "${BLUE}========================================${NC}"
echo ""
echo "Next steps:"
echo "1. Go to: https://github.com/gouh/agesmith/releases/new?tag=v$NEW_VERSION"
echo "2. Upload binaries from dist/:"
echo "   - agesmith-$NEW_VERSION-aarch64-apple-darwin"
echo "   - agesmith-$NEW_VERSION-x86_64-apple-darwin"
echo "   - agesmith-$NEW_VERSION-x86_64-unknown-linux-gnu"
echo "   - checksums.txt"
echo "3. Publish the release"
echo ""
