# Installation Scripts

This directory contains installation scripts for AgeSmith.

## Quick Install (Recommended)

### Universal Script (Linux/macOS)

```bash
curl -sSL https://raw.githubusercontent.com/yourusername/agesmith/main/scripts/install.sh | bash
```

This script will:
1. Detect your OS and architecture
2. Download the appropriate binary
3. Verify the checksum
4. Install to `~/.local/bin/`

### Custom Install Directory

```bash
curl -sSL https://raw.githubusercontent.com/yourusername/agesmith/main/scripts/install.sh | INSTALL_DIR=/usr/local/bin bash
```

### Specific Version

```bash
curl -sSL https://raw.githubusercontent.com/yourusername/agesmith/main/scripts/install.sh | VERSION=v0.1.0 bash
```

## Manual Installation

### 1. Download Binary

Go to [Releases](https://github.com/yourusername/agesmith/releases) and download the binary for your platform:

- **Linux**: `agesmith-{version}-x86_64-unknown-linux-gnu`
- **macOS Intel**: `agesmith-{version}-x86_64-apple-darwin`
- **macOS Apple Silicon**: `agesmith-{version}-aarch64-apple-darwin`
- **Windows**: `agesmith-{version}-x86_64-pc-windows-gnu.exe`

### 2. Verify Checksum

Download `checksums.txt` from the same release page.

**Linux/macOS:**
```bash
shasum -a 256 agesmith-*
# Compare with checksums.txt
```

**Windows (PowerShell):**
```powershell
Get-FileHash agesmith-*.exe -Algorithm SHA256
# Compare with checksums.txt
```

### 3. Install

**Linux/macOS:**
```bash
chmod +x agesmith-*
sudo mv agesmith-* /usr/local/bin/agesmith
```

**Windows:**
Move the `.exe` file to a directory in your PATH.

## Verify Installation

After installation, verify it works:

```bash
agesmith --version
```

Verify the installed binary checksum:

```bash
# Linux/macOS
shasum -a 256 $(which agesmith)

# Windows (PowerShell)
Get-FileHash (Get-Command agesmith).Source -Algorithm SHA256
```

Compare the output with the checksum in `checksums.txt` for your platform.

## Troubleshooting

### Binary not found after installation

Make sure the installation directory is in your PATH:

```bash
# Add to ~/.bashrc or ~/.zshrc
export PATH="$PATH:$HOME/.local/bin"
```

### Permission denied

The binary needs execute permissions:

```bash
chmod +x /path/to/agesmith
```

### Checksum mismatch

If the checksum doesn't match:
1. Re-download the binary
2. Check you downloaded the correct version
3. Report the issue if it persists

## Uninstall

```bash
# Remove binary
rm $(which agesmith)

# Remove config (optional)
rm -rf ~/.config/agesmith
```
