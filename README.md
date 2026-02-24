# AgeSmith

<div align="center">

**🔐 Forging Secure Secrets with Age Encryption**

A powerful Terminal User Interface (TUI) for managing SOPS-encrypted secrets

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![Tests](https://img.shields.io/badge/tests-80%2B%20passing-brightgreen.svg)](./docs/TESTING.md)

[Features](#-features) •
[Installation](#-installation) •
[Usage](#-usage) •
[Documentation](#-documentation) •
[Contributing](#-contributing)

</div>

---

## 🎯 What is AgeSmith?

AgeSmith is a modern TUI application that makes managing encrypted secrets simple and secure. It provides an intuitive interface for working with SOPS-encrypted files using age encryption, supporting multiple file formats (JSON, YAML, ENV, INI) with advanced features like search, filtering, and in-place editing.

### Why AgeSmith?

- **🔒 Secure**: Built on SOPS and age encryption
- **⚡ Fast**: Instant file loading and searching
- **🎨 Beautiful**: Modern UI with intuitive controls
- **🌍 Universal**: Supports JSON, YAML, ENV, and INI formats
- **🔍 Smart**: Auto-detects keys and handles special characters
- **🌐 International**: English and Spanish support

## ✨ Features

### Core Capabilities

- **🔑 Automatic Key Management**
  - Reads age identities from `~/.config/sops/age/keys.txt`
  - Auto-detects correct key based on file recipients
  - Manual key selection when needed
  - Key validation and recipient matching

- **📁 Multi-Format Support**
  - JSON with nested structures
  - YAML configurations
  - ENV files (.env)
  - INI configuration files
  - Smart format detection
  - Automatic value quoting for special characters

- **✏️ In-Place Editing**
  - Edit secret values directly
  - Add new secrets with wizard
  - Delete with confirmation
  - Bulk operations on marked files
  - Undo-safe with automatic backups

- **🔍 Advanced Search**
  - Search by key or value
  - Regex pattern matching
  - Real-time filtering
  - Case-insensitive search
  - Search across nested structures

- **🎨 Modern Interface**
  - Dual-panel layout (explorer + secrets)
  - Visual indicators (⭐ favorites, ✓ marked, 📁 directories)
  - Masked values by default (toggle with `v`)
  - Zoom modal for full value viewing
  - Pretty-print JSON
  - Contextual help panel (`?`)

- **🛠️ Utilities**
  - Secret generator (passwords, hex, base64, UUID)
  - Clipboard support (copy keys/values)
  - Favorites management
  - File creation wizard
  - Batch operations

- **⚙️ Configuration**
  - Persistent settings in `~/.config/agesmith/config.toml`
  - Theme customization
  - Auto-lock timeout
  - Clipboard clear timeout
  - Language selection (EN/ES)

## 📋 Requirements

- **Rust** 1.70 or higher
- **SOPS** installed and in PATH
- **Age keys** in `~/.config/sops/age/keys.txt`

### Installing Dependencies

```bash
# macOS
brew install sops age

# Linux (Debian/Ubuntu)
sudo apt install age
# Install SOPS from: https://github.com/mozilla/sops/releases

# Generate age key
age-keygen -o ~/.config/sops/age/keys.txt
```

## 🚀 Installation

### Quick Install (Recommended)

**Linux/macOS:**
```bash
curl -sSL https://raw.githubusercontent.com/gouh/agesmith/main/scripts/install.sh | bash
```

This will automatically:
- Detect your OS and architecture
- Download the correct binary
- Verify the checksum
- Install to `~/.local/bin/`

### Using Makefile (For Developers)

```bash
# Clone the repository
git clone https://github.com/gouh/agesmith.git
cd agesmith

# Build and install
make build
make install
```

See [Makefile Documentation](docs/MAKEFILE.md) for all available commands.

### From Source

```bash
# Clone the repository
git clone https://github.com/gouh/agesmith.git
cd agesmith

# Build release version
cargo build --release

# Binary will be at: ./target/release/agesmith
```

### Manual Installation

1. Download the binary for your platform from [Releases](https://github.com/gouh/agesmith/releases)
2. Verify the checksum (see [Installation Guide](scripts/README.md))
3. Make it executable: `chmod +x agesmith-*`
4. Move to PATH: `sudo mv agesmith-* /usr/local/bin/agesmith`

### Building for Multiple Platforms

```bash
# Requires cross: cargo install cross
make build-all

# Binaries will be in dist/ directory
```

### Verify Installation

```bash
# Check version
agesmith --version

# Verify checksum
shasum -a 256 $(which agesmith)
# Compare with checksums.txt from the release
```

## 💻 Usage

### Quick Start

```bash
# Start in current directory
agesmith

# Start in specific directory
agesmith /path/to/secrets

# With compiled binary
./target/release/agesmith
./target/release/agesmith /path/to/secrets
```

### Basic Workflow

1. **Navigate** - Use `↑/↓` to browse files
2. **Open** - Press `Enter` to decrypt and view secrets
3. **Edit** - Press `e` to modify a value
4. **Save** - Press `s` to encrypt and save changes
5. **Search** - Press `/` to filter secrets

## ⌨️ Keyboard Shortcuts

### File Explorer

| Key | Action |
|-----|--------|
| `↑/↓` | Navigate files and directories |
| `Enter` | Open directory or decrypt file |
| `m` | Mark/unmark file for batch operations |
| `Tab` | Switch to secrets panel |
| `k` | Open age key selector |
| `i` | Initialize SOPS in directory |
| `+` | Create new secret file |
| `r` | Rename file |
| `D` | Delete file |
| `q` | Quit application |

### Secrets Panel

| Key | Action |
|-----|--------|
| `↑/↓` | Navigate secrets |
| `v` | Toggle show/hide values |
| `z` | Open zoom modal (full value view) |
| `c` | Copy secret value to clipboard |
| `C` | Copy secret key to clipboard |
| `f` | Add/remove file from favorites |
| `/` | Search secrets |
| `e` | Edit selected secret |
| `n` | Add new secret |
| `d` | Delete secret (with confirmation) |
| `s` | Save changes (re-encrypt) |
| `g` | Open secret generator |
| `k` | Open age key selector |
| `?` | Show help panel |
| `Tab` | Return to explorer |
| `q` | Quit application |

### Search Mode

| Key | Action |
|-----|--------|
| `Type` | Filter secrets by key or value |
| `r` | Toggle regex mode (🔍→🔎) |
| `Enter` | Apply filter |
| `Esc` | Cancel search |

### Zoom Modal

| Key | Action |
|-----|--------|
| `↑/↓` | Scroll through content |
| `j` | Toggle pretty-print JSON |
| `Esc/z` | Close modal |

## ⚙️ Configuration

Create `~/.config/agesmith/config.toml`:

```toml
# Theme: "dark" or "light"
theme = "dark"

# Auto-lock after N minutes of inactivity
auto_lock_minutes = 15

# Clear clipboard after N seconds
clipboard_clear_seconds = 30

# Language: "en" (English) or "es" (Spanish)
language = "en"
```

### Theme Colors

Customize colors in your config:

```toml
[theme]
primary = [100, 200, 255]
success = [100, 255, 100]
error = [255, 100, 100]
warning = [255, 200, 100]
bg = [0, 0, 0]
fg = [255, 255, 255]
```

## 📚 Documentation

- **[Makefile Guide](docs/MAKEFILE.md)** - Build, test, and release commands
- **[Testing Guide](docs/TESTING.md)** - Test suite documentation
- **[Writing Tests](docs/WRITING_TESTS.md)** - How to write new tests
- **[Roadmap](docs/ROADMAP.md)** - Project goals and planned features
- **[Architecture](docs/ARCHITECTURE.md)** - Code structure and design

## 🏗️ Architecture

```
src/
├── main.rs          # Entry point and event loop
├── state.rs         # Application state and business logic
├── ui.rs            # Visual components and rendering
├── events.rs        # Keyboard event handling
├── sops.rs          # SOPS/age encryption operations
├── config.rs        # Configuration management
├── generator.rs     # Secret generation utilities
├── i18n.rs          # Internationalization
└── help.rs          # Help system
```

### Key Components

- **State Management**: Centralized app state with immutable updates
- **Event Loop**: Async event handling with Tokio
- **UI Rendering**: Ratatui-based terminal interface
- **SOPS Integration**: Command-line interface to SOPS binary
- **Format Support**: Pluggable format handlers (JSON, YAML, ENV, INI)

## 🧪 Testing

AgeSmith has a comprehensive test suite with **80+ tests**:

```bash
# Run all tests
make test

# Run with output
make test-verbose

# Check code quality
make check

# Format code
make fmt
```

See [Testing Documentation](docs/TESTING.md) for details.

## 🤝 Contributing

Contributions are welcome! Here's how you can help:

1. **Report Bugs** - Open an issue with details
2. **Suggest Features** - Share your ideas
3. **Submit PRs** - Fix bugs or add features
4. **Improve Docs** - Help make documentation better
5. **Write Tests** - Increase test coverage

### Development Setup

```bash
# Clone and build
git clone https://github.com/gouh/agesmith.git
cd agesmith
make build

# Run tests
make test

# Check code quality
make check
make fmt
```

### Release Process

```bash
# Bump version (patch/minor/major)
make bump-patch

# Build for all platforms
make build-all

# Binaries will be in dist/
```

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- **[SOPS](https://github.com/mozilla/sops)** - Secrets OPerationS
- **[Age](https://age-encryption.org/)** - Simple, modern encryption
- **[Ratatui](https://ratatui.rs/)** - Terminal UI framework
- **[Crossterm](https://github.com/crossterm-rs/crossterm)** - Terminal manipulation

## 📧 Contact

- **Issues**: [GitHub Issues](https://github.com/gouh/agesmith/issues)
- **Discussions**: [GitHub Discussions](https://github.com/gouh/agesmith/discussions)

---

<div align="center">

**Built with ❤️ using Rust, Ratatui, and Age Encryption**

[⬆ Back to Top](#agesmith)

</div>
