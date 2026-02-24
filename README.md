# AgeSmith

**Forging secure secrets with age**

A powerful TUI (Terminal User Interface) for managing SOPS-encrypted secrets with age encryption.

## Features

- Automatically reads age identities from `~/.config/sops/age/keys.txt`
- **Auto-key detection**: Automatically selects the correct key based on file recipients
- Decrypts SOPS files using the system binary
- Displays secrets in table format with masked values by default
- **Only masks encrypted values**: Unencrypted values are always shown
- **Clipboard support**: Copy decrypted values or keys with `c` and `C`
- **In-place editing**: Edit, add, and delete secrets directly in the TUI
- **Smart saving**: Re-encrypts with SOPS maintaining original recipients
- **Secret generator**: Generate secure passwords, hex/base64 tokens, and UUIDs
- **Advanced search**: Normal or regex search to filter secrets
- **Zoom modal**: View full values with scroll and pretty print JSON
- **Persistent configuration**: Customize timeouts and preferences in `~/.config/agesmith/config.toml`
- **Internationalization**: Support for Spanish and English (configurable)
- Allows manual selection of a specific age key by injecting `SOPS_AGE_KEY`
- Flattens nested JSON structures for simple visualization
- **Key search**: Filter keys by name or public key
- **Secret search**: Filter secrets by key or value
- **Recipient validation**: Indicates which keys match the encrypted file
- **Detailed information**: Shows public keys and file recipients
- **Help panel**: Press `?` to see all available commands
- **Modern theme**: Vibrant colors and intuitive UI

## Requirements

- Rust (1.70+)
- `sops` installed on the system
- Age keys in `~/.config/sops/age/keys.txt`

## Installation

```bash
cargo build --release
```

## Usage

```bash
# Start in current directory
cargo run

# Start in a specific directory
cargo run -- /path/to/secrets

# With compiled binary
./target/release/agesmith
./target/release/agesmith /path/to/secrets
```

## Controls

### In Explorer:

- **↑/↓**: Navigate files and directories
- **Enter**: Open directory or load encrypted file
- **m**: Mark/unmark file for batch operations
- **Tab**: Switch to secrets panel
- **k**: Open age key selector
- **q**: Quit

**Visual indicators**:
- ⭐ File in favorites
- ✓ Marked file
- 📁 Directory
- 📄 File

### In Secrets Panel:

- **↑/↓**: Navigate secrets
- **v**: Toggle show/hide values (encrypted values only)
- **z**: Open zoom modal to view full value
- **c**: Copy selected secret value to clipboard
- **C**: Copy selected secret key to clipboard
- **f**: Add/remove current file from favorites
- **/**: Activate secret search
- **e**: Edit selected secret value
- **n**: Add new secret (key + value)
- **d**: Delete selected secret (with confirmation)
- **s**: Save changes to file (re-encrypts with SOPS)
- **g**: Open secret generator (passwords, tokens, UUIDs)
- **k**: Open age key selector
- **?**: Show help panel with all commands
- **Tab**: Return to explorer
- **q**: Quit

### In Secret Search Mode:

- **Type**: Filter secrets by key or value
- **r**: Toggle regex mode (🔍→🔎)
- **Enter**: Apply filter
- **Esc**: Cancel search

### In Zoom Modal:

- **↑/↓**: Scroll through content
- **j**: Toggle pretty print JSON
- **Esc/z**: Close modal

### In Key Selector:

- **↑/↓**: Navigate available keys
- **/**: Activate key search
- **Enter**: Apply selected key and retry decryption
- **Esc**: Cancel

### In Search Mode:

- **Type**: Filter keys by name or public key
- **Enter**: Apply filter and return to selector
- **Esc**: Cancel search

## Configuration

Create the file `~/.config/agesmith/config.toml`:

```toml
theme = "dark"
auto_lock_minutes = 15
clipboard_clear_seconds = 3
language = "en"  # "en" for English, "es" for Spanish
```

## Code Structure

- **App State**: Manages secrets, table state, age keys, and input mode
- **SOPS Backend**: Uses `std::process::Command` to invoke `sops -d`
- **UI**: Main table with command footer and key selection modal
- **Identities**: Parser for `keys.txt` that extracts keys and comments

## Architecture

The project is organized into 8 specialized modules:

- **main.rs** (94 lines) - Entry point and event loop
- **config.rs** (90 lines) - Configuration and favorites management
- **sops.rs** (175 lines) - SOPS/age encryption operations
- **state.rs** (564 lines) - Application state and business logic
- **ui.rs** (571 lines) - Visual component rendering
- **events.rs** (429 lines) - Event handling
- **generator.rs** (57 lines) - Secret generation
- **help.rs** (33 lines) - Help panel
- **i18n.rs** (160 lines) - Internationalization

## License

MIT

## Author

Built with ❤️ using Rust, Ratatui, and age encryption
