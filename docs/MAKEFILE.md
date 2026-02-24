# Makefile Documentation

This document describes the available Makefile commands for building, testing, and managing AgeSmith.

## Prerequisites

- **Rust toolchain** (1.70+)
- **cargo-edit** (for version bumping): `cargo install cargo-edit`
- **cross** (optional, for cross-compilation): `cargo install cross`

## Available Commands

### Help

```bash
make help
```

Shows all available commands with descriptions.

### Building

#### Build for Current Platform

```bash
make build
```

Creates an optimized release binary for your current platform in `target/release/agesmith`.

#### Build for All Platforms

```bash
make build-all
```

Builds binaries for multiple platforms:
- Linux (x86_64)
- macOS Intel (x86_64)
- macOS Apple Silicon (aarch64)
- Windows (x86_64)

Binaries are created in the `dist/` directory with the format:
```
agesmith-{version}-{target}
```

**Note**: Requires `cross` for cross-compilation. Install with:
```bash
cargo install cross
```

### Testing

#### Run Tests

```bash
make test
```

Runs all library tests.

#### Run Tests with Output

```bash
make test-verbose
```

Runs tests with full output (useful for debugging).

#### Code Quality Checks

```bash
make check
```

Runs `cargo check` and `cargo clippy` to verify code quality.

#### Format Code

```bash
make fmt
```

Formats all Rust code using `rustfmt`.

### Version Management

AgeSmith follows [Semantic Versioning](https://semver.org/):
- **MAJOR**: Breaking changes
- **MINOR**: New features (backward compatible)
- **PATCH**: Bug fixes

#### Bump Patch Version

```bash
make bump-patch
```

Increments the patch version (e.g., 0.1.0 → 0.1.1).

#### Bump Minor Version

```bash
make bump-minor
```

Increments the minor version (e.g., 0.1.0 → 0.2.0).

#### Bump Major Version

```bash
make bump-major
```

Increments the major version (e.g., 0.1.0 → 1.0.0).

### Installation

#### Install to System

```bash
make install
```

Installs the binary to your system (typically `~/.cargo/bin/`).

### Development

#### Run in Development Mode

```bash
make dev
```

Runs AgeSmith in development mode (unoptimized, faster compilation).

### Cleanup

```bash
make clean
```

Removes all build artifacts and the `dist/` directory.

## Typical Workflows

### Development Workflow

```bash
# Make changes to code
make fmt              # Format code
make check            # Check for errors
make test             # Run tests
make dev              # Test locally
```

### Release Workflow

```bash
# Prepare release
make test             # Ensure tests pass
make bump-minor       # Bump version
make build-all        # Build for all platforms
git add .
git commit -m "Release v0.2.0"
git tag v0.2.0
git push --tags
```

### Quick Build and Install

```bash
make build && make install
```

## Platform-Specific Notes

### macOS

Both Intel and Apple Silicon binaries are built by default.

### Linux

Requires `cross` for cross-compilation from macOS/Windows.

### Windows

Windows binaries have `.exe` extension. Cross-compilation from Unix systems requires `cross`.

## Troubleshooting

### Cross-compilation fails

If `cross` is not installed or fails, the Makefile will fall back to `cargo build --target`, which may not work for all platforms.

**Solution**: Install cross:
```bash
cargo install cross
```

### Version bump fails

Requires `cargo-edit`.

**Solution**: Install cargo-edit:
```bash
cargo install cargo-edit
```

### Tests fail

Some tests require SOPS and age to be installed.

**Solution**: Install dependencies:
```bash
# macOS
brew install sops age

# Linux
sudo apt install age
# Install SOPS from: https://github.com/mozilla/sops/releases
```

## Examples

### Build and test before committing

```bash
make fmt && make check && make test && make build
```

### Create a patch release

```bash
make test && make bump-patch && make build-all
```

### Clean rebuild

```bash
make clean && make build
```
