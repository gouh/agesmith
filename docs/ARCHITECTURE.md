# AgeSmith Architecture

## Overview

AgeSmith is built with a clean, modular architecture following Rust best practices. The application uses an event-driven model with centralized state management.

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────┐
│                        main.rs                          │
│                    (Entry Point)                        │
│                                                         │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐ │
│  │ Terminal     │  │ Event Loop   │  │ State Init   │ │
│  │ Setup        │  │ (Tokio)      │  │              │ │
│  └──────────────┘  └──────────────┘  └──────────────┘ │
└─────────────────────────────────────────────────────────┘
                           │
                           ▼
        ┌──────────────────────────────────────┐
        │          events.rs                   │
        │      (Event Handling)                │
        │                                      │
        │  ┌────────────┐  ┌────────────┐    │
        │  │ Keyboard   │  │ Mode       │    │
        │  │ Events     │  │ Routing    │    │
        │  └────────────┘  └────────────┘    │
        └──────────────────────────────────────┘
                           │
                           ▼
        ┌──────────────────────────────────────┐
        │          state.rs                    │
        │      (Business Logic)                │
        │                                      │
        │  ┌────────────┐  ┌────────────┐    │
        │  │ App State  │  │ Operations │    │
        │  │ Management │  │ (CRUD)     │    │
        │  └────────────┘  └────────────┘    │
        └──────────────────────────────────────┘
                           │
        ┌──────────────────┼──────────────────┐
        │                  │                  │
        ▼                  ▼                  ▼
┌──────────────┐  ┌──────────────┐  ┌──────────────┐
│   sops.rs    │  │   ui.rs      │  │  config.rs   │
│ (Encryption) │  │ (Rendering)  │  │ (Settings)   │
└──────────────┘  └──────────────┘  └──────────────┘
        │                  │                  │
        ▼                  ▼                  ▼
┌──────────────┐  ┌──────────────┐  ┌──────────────┐
│ generator.rs │  │   i18n.rs    │  │   help.rs    │
│  (Secrets)   │  │ (Languages)  │  │   (Help)     │
└──────────────┘  └──────────────┘  └──────────────┘
```

## Module Breakdown

### main.rs (94 lines)
**Purpose**: Application entry point and main event loop

**Responsibilities**:
- Terminal initialization and cleanup
- Event loop with Tokio async runtime
- Keyboard event capture
- UI rendering coordination

**Key Functions**:
- `main()` - Entry point, sets up terminal
- `run_app()` - Main event loop

**Dependencies**: `crossterm`, `ratatui`, `tokio`

### state.rs (1,268 lines)
**Purpose**: Application state and business logic

**Responsibilities**:
- Centralized state management
- Secret CRUD operations
- File operations (load, save, create)
- Search and filter logic
- Key management
- Favorites handling

**Key Structures**:
```rust
pub struct App {
    pub secrets: HashMap<String, String>,
    pub file_path: Option<PathBuf>,
    pub age_keys: Vec<AgeKey>,
    pub input_mode: InputMode,
    // ... 40+ fields
}

pub enum InputMode {
    Explorer,
    Secrets,
    Editing,
    Searching,
    // ... 20+ modes
}
```

**Key Functions**:
- `new()` - Initialize app state
- `load_file()` - Decrypt and load secrets
- `save_changes()` - Encrypt and save secrets
- `add_secret()` - Create new secret
- `delete_secret()` - Remove secret
- `search_secrets()` - Filter secrets

### ui.rs (1,357 lines)
**Purpose**: Visual rendering and UI components

**Responsibilities**:
- Render all UI components
- Layout management
- Theme application
- Modal rendering
- Table formatting

**Key Functions**:
- `ui()` - Main rendering function
- `render_file_explorer()` - File list panel
- `render_secrets_panel()` - Secrets table
- `render_key_selector_modal()` - Key selection
- `render_edit_modal()` - Edit dialog
- `render_help_modal()` - Help screen
- `render_zoom_modal()` - Value viewer

**UI Components**:
- File explorer (left panel)
- Secrets table (right panel)
- Modals (key selector, edit, help, zoom)
- Footer with commands
- Status messages

### events.rs (775 lines)
**Purpose**: Keyboard event handling and routing

**Responsibilities**:
- Capture keyboard input
- Route events based on input mode
- Handle text input
- Coordinate state updates

**Key Functions**:
- `handle_key_event()` - Main event router
- `handle_explorer_keys()` - File navigation
- `handle_secrets_keys()` - Secret operations
- `handle_editing_keys()` - Edit mode
- `handle_searching_keys()` - Search mode
- `handle_text_input()` - Text entry

**Event Flow**:
```
Keyboard Input
    ↓
handle_key_event()
    ↓
Mode-specific handler
    ↓
State update
    ↓
UI re-render
```

### sops.rs (250 lines)
**Purpose**: SOPS encryption/decryption operations

**Responsibilities**:
- Execute SOPS commands
- Parse decrypted JSON
- Flatten nested structures
- Handle age keys
- Detect encrypted values

**Key Functions**:
- `decrypt_file()` - Decrypt with SOPS
- `flatten_json()` - Convert nested JSON to flat map
- `get_encrypted_keys()` - Find encrypted values
- `read_age_keys()` - Parse age identities
- `quote_env_value()` - Handle special characters
- `unquote_env_value()` - Restore original values

**SOPS Integration**:
```rust
Command::new("sops")
    .arg("-d")
    .arg("--output-type").arg("json")
    .arg(file_path)
    .env("SOPS_AGE_KEY", key)
    .output()
```

### config.rs (90 lines)
**Purpose**: Configuration and favorites management

**Responsibilities**:
- Load/save configuration
- Manage favorites list
- Theme settings
- User preferences

**Key Structures**:
```rust
pub struct Config {
    pub theme: String,
    pub auto_lock_minutes: u32,
    pub clipboard_clear_seconds: u32,
    pub language: String,
}

pub struct Theme {
    pub primary: (u8, u8, u8),
    pub success: (u8, u8, u8),
    pub error: (u8, u8, u8),
    // ...
}
```

**Configuration Path**: `~/.config/agesmith/config.toml`

### generator.rs (57 lines)
**Purpose**: Secret generation utilities

**Responsibilities**:
- Generate passwords
- Generate hex tokens
- Generate base64 tokens
- Generate UUIDs

**Key Functions**:
- `generate_password()` - Random password
- `generate_hex()` - Hex string
- `generate_base64()` - Base64 string
- `generate_uuid()` - UUID v4

### i18n.rs (160 lines)
**Purpose**: Internationalization support

**Responsibilities**:
- Manage translations
- Language switching
- Text formatting

**Supported Languages**:
- English (en)
- Spanish (es)

**Key Structure**:
```rust
pub struct I18n {
    lang: Language,
    translations: HashMap<&'static str, (&'static str, &'static str)>,
}
```

### help.rs (33 lines)
**Purpose**: Help system

**Responsibilities**:
- Provide command help
- Context-sensitive help text

## Data Flow

### Loading a File

```
User selects file
    ↓
events.rs: handle_explorer_keys()
    ↓
state.rs: load_file()
    ↓
sops.rs: decrypt_file()
    ↓
sops.rs: flatten_json()
    ↓
state.rs: Update secrets HashMap
    ↓
ui.rs: render_secrets_panel()
```

### Editing a Secret

```
User presses 'e'
    ↓
events.rs: handle_secrets_keys()
    ↓
state.rs: Set InputMode::Editing
    ↓
ui.rs: render_edit_modal()
    ↓
User types new value
    ↓
events.rs: handle_editing_keys()
    ↓
state.rs: Update secret value
    ↓
User presses 's'
    ↓
state.rs: save_changes()
    ↓
sops.rs: Encrypt with SOPS
```

### Searching Secrets

```
User presses '/'
    ↓
events.rs: Set InputMode::SearchingSecrets
    ↓
ui.rs: Show search input
    ↓
User types query
    ↓
events.rs: handle_text_input()
    ↓
state.rs: Filter secrets
    ↓
ui.rs: Render filtered results
```

## State Management

### Centralized State

All application state is managed in the `App` struct:

```rust
pub struct App {
    // File state
    pub file_path: Option<PathBuf>,
    pub secrets: HashMap<String, String>,
    pub encrypted_keys: Vec<String>,
    
    // UI state
    pub input_mode: InputMode,
    pub table_state: TableState,
    pub explorer_state: ListState,
    
    // User input
    pub search_query: String,
    pub edit_buffer: String,
    
    // Configuration
    pub config: Config,
    pub theme: Theme,
    pub i18n: I18n,
    
    // ... more fields
}
```

### Immutable Updates

State updates follow functional patterns:

```rust
// Don't mutate directly
app.secrets.insert(key, value);

// Instead, use methods
app.add_secret(key, value)?;
```

## Error Handling

Uses `anyhow::Result` for error propagation:

```rust
pub fn load_file(&mut self, path: PathBuf) -> Result<()> {
    let decrypted = decrypt_file(&path, key)?;
    self.secrets = parse_secrets(decrypted)?;
    Ok(())
}
```

## Async Runtime

Uses Tokio for async operations:

```rust
#[tokio::main]
async fn main() -> Result<()> {
    // Event loop with async/await
}
```

## Testing Strategy

### Unit Tests
- Individual function testing
- Mock data
- Fast execution

### Integration Tests
- Multi-component workflows
- Realistic scenarios
- Medium speed

### E2E Tests
- Full application flow
- Real SOPS encryption
- Slower execution

See [TESTING.md](TESTING.md) for details.

## Performance Considerations

### Optimizations
- Lazy loading of files
- Efficient HashMap lookups
- Minimal re-renders
- Async I/O operations

### Bottlenecks
- Large files (1000+ secrets)
- SOPS encryption/decryption
- Terminal rendering

## Security

### Best Practices
- No secrets in memory longer than needed
- Clipboard auto-clear
- Encrypted file validation
- Key permission checks

### Threat Model
- Protects against: Unauthorized file access
- Does not protect against: Memory dumps, keyloggers

## Future Architecture

### Planned Improvements
- Plugin system
- Modular format handlers
- Async file operations
- Caching layer
- Event sourcing for undo/redo

## Dependencies

### Core
- `ratatui` - TUI framework
- `crossterm` - Terminal manipulation
- `tokio` - Async runtime
- `serde_json` - JSON parsing
- `anyhow` - Error handling

### Utilities
- `arboard` - Clipboard
- `rand` - Random generation
- `uuid` - UUID generation
- `regex` - Pattern matching
- `toml` - Configuration
- `base64` - Encoding
- `dirs` - Directory paths

## Code Metrics

| Module | Lines | Complexity | Test Coverage |
|--------|-------|------------|---------------|
| main.rs | 94 | Low | N/A |
| state.rs | 1,268 | High | 70% |
| ui.rs | 1,357 | Medium | 10% |
| events.rs | 775 | Medium | 10% |
| sops.rs | 250 | Medium | 95% |
| config.rs | 90 | Low | 90% |
| generator.rs | 57 | Low | 90% |
| i18n.rs | 160 | Low | 80% |
| help.rs | 33 | Low | 50% |

**Total**: ~4,300 lines of code

## Contributing

When contributing, maintain:
- Module separation
- Single responsibility
- Clear function names
- Comprehensive tests
- Documentation

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.
