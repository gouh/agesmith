# AgeSmith Project Goals & Roadmap

## Project Vision

**AgeSmith** is a powerful Terminal User Interface (TUI) for managing SOPS-encrypted secrets with age encryption, providing a secure, intuitive, and efficient way to handle sensitive data.

## ✅ Completed Features

### Core Functionality
- [x] Age identity management from `~/.config/sops/age/keys.txt`
- [x] Automatic key detection based on file recipients
- [x] SOPS file decryption using system binary
- [x] Table-based secret display with masked values
- [x] Selective masking (only encrypted values)
- [x] In-place editing (edit, add, delete secrets)
- [x] Smart re-encryption maintaining original recipients
- [x] Nested JSON structure flattening

### File Format Support
- [x] JSON format
- [x] YAML format
- [x] ENV (.env) format
- [x] INI (.ini) format
- [x] Smart value quoting for special characters
- [x] Format auto-detection

### User Interface
- [x] File explorer with directory navigation
- [x] Dual-panel layout (explorer + secrets)
- [x] Visual indicators (⭐ favorites, ✓ marked, 📁 directories)
- [x] Help panel with all commands (`?`)
- [x] Modern theme with vibrant colors
- [x] Zoom modal for full value viewing
- [x] Pretty-print JSON in zoom view

### Search & Filter
- [x] Secret search by key or value
- [x] Regex search mode
- [x] Key search in selector
- [x] Real-time filtering

### Security Features
- [x] Recipient validation
- [x] Key matching indicators
- [x] Manual key selection
- [x] Encrypted value detection
- [x] Secure clipboard operations

### Utilities
- [x] Secret generator (passwords, hex, base64, UUID)
- [x] Clipboard support (copy keys and values)
- [x] Favorites management
- [x] Batch file operations (mark/unmark)
- [x] File creation wizard

### Configuration
- [x] Persistent configuration (`~/.config/agesmith/config.toml`)
- [x] Theme customization
- [x] Auto-lock timeout
- [x] Clipboard clear timeout
- [x] Language selection (EN/ES)

### Internationalization
- [x] English language support
- [x] Spanish language support
- [x] Configurable language switching
- [x] Complete translation coverage

### Quality & Testing
- [x] Comprehensive test suite (80+ tests)
- [x] Unit tests for core functions
- [x] Integration tests for workflows
- [x] E2E tests with SOPS
- [x] Special character handling
- [x] Edge case coverage
- [x] Zero compilation warnings

### Documentation
- [x] README with features and usage
- [x] Test documentation
- [x] Test writing guide
- [x] Architecture overview
- [x] Configuration examples

## 🚧 In Progress

### Bug Fixes
- [ ] Improve error messages
- [ ] Handle network timeouts gracefully
- [ ] Better handling of corrupted files

### Performance
- [ ] Optimize large file loading (1000+ secrets)
- [ ] Lazy loading for file explorer
- [ ] Cache decrypted values

## 📋 Planned Features

### High Priority

#### Enhanced Security
- [ ] Password strength indicator
- [ ] Secret expiration warnings
- [ ] Audit log for changes
- [ ] Two-factor authentication support
- [ ] Secure memory wiping

#### File Operations
- [ ] File diff viewer (before/after save)
- [ ] Undo/redo functionality
- [ ] File history/versioning
- [ ] Batch edit multiple files
- [ ] Import/export between formats

#### Search & Navigation
- [ ] Fuzzy search
- [ ] Recent files list
- [ ] Bookmarks/quick access
- [ ] Global search across all files
- [ ] Search history

#### UI Improvements
- [ ] Customizable key bindings
- [ ] Multiple theme options
- [ ] Split view for comparing files
- [ ] Tabs for multiple files
- [ ] Status bar with more info

### Medium Priority

#### Integration
- [ ] Git integration (commit on save)
- [ ] Cloud sync support
- [ ] Webhook notifications
- [ ] External editor integration
- [ ] Shell completion scripts

#### Advanced Features
- [ ] Secret templates
- [ ] Variable substitution
- [ ] Secret sharing (encrypted)
- [ ] Team collaboration features
- [ ] Role-based access control

#### Developer Experience
- [ ] Plugin system
- [ ] API for automation
- [ ] CLI mode (non-interactive)
- [ ] Scripting support
- [ ] REST API server mode

### Low Priority

#### Nice to Have
- [ ] Mouse support
- [ ] Drag and drop
- [ ] Syntax highlighting for values
- [ ] Auto-complete for keys
- [ ] Secret suggestions
- [ ] Dark/light theme toggle hotkey
- [ ] Export to password managers
- [ ] QR code generation for secrets
- [ ] Biometric authentication

#### Platform Support
- [ ] Windows native support
- [ ] macOS keychain integration
- [ ] Linux keyring integration
- [ ] Mobile companion app
- [ ] Web interface

## 🎯 Milestones

### v0.1.0 (Current) ✅
- Core functionality
- Basic file format support
- Essential UI features
- Configuration system
- Internationalization

### v0.2.0 (Next)
- [ ] Enhanced security features
- [ ] File diff viewer
- [ ] Undo/redo
- [ ] Improved error handling
- [ ] Performance optimizations

### v0.3.0
- [ ] Git integration
- [ ] Secret templates
- [ ] Advanced search
- [ ] Plugin system
- [ ] CLI mode

### v1.0.0 (Stable)
- [ ] All high-priority features
- [ ] Comprehensive documentation
- [ ] Security audit
- [ ] Performance benchmarks
- [ ] Production-ready

## 🐛 Known Issues

### Critical
- None currently

### High
- None currently

### Medium
- [ ] Large files (10,000+ secrets) can be slow
- [ ] Some terminal emulators have rendering issues
- [ ] Clipboard may not work in all environments

### Low
- [ ] Help panel could be more detailed
- [ ] Some error messages are too technical
- [ ] Theme colors not optimized for all terminals

## 🔬 Technical Debt

- [ ] Refactor large UI functions (>200 lines)
- [ ] Add more inline documentation
- [ ] Improve error type hierarchy
- [ ] Reduce code duplication in event handlers
- [ ] Add performance benchmarks
- [ ] Implement proper logging system

## 📊 Metrics & Goals

### Code Quality
- [x] Zero compilation warnings
- [x] 70%+ test coverage
- [ ] 90%+ test coverage (goal)
- [ ] Code documentation >80%
- [ ] Clippy clean (all lints)

### Performance
- [x] Startup time <100ms
- [x] File load time <500ms (small files)
- [ ] File load time <2s (large files)
- [ ] Search response <100ms
- [ ] Memory usage <50MB

### User Experience
- [x] Intuitive key bindings
- [x] Clear visual feedback
- [ ] Comprehensive help system
- [ ] Onboarding tutorial
- [ ] Video demonstrations

## 🤝 Contributing

We welcome contributions! Priority areas:

1. **Testing**: Add more test coverage
2. **Documentation**: Improve guides and examples
3. **Features**: Implement planned features
4. **Bug Fixes**: Fix known issues
5. **Performance**: Optimize slow operations

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## 📅 Release Schedule

- **Patch releases** (v0.1.x): As needed for bug fixes
- **Minor releases** (v0.x.0): Every 2-3 months
- **Major releases** (vx.0.0): When significant features complete

## 🎓 Learning Resources

- [SOPS Documentation](https://github.com/mozilla/sops)
- [Age Encryption](https://age-encryption.org/)
- [Ratatui TUI Framework](https://ratatui.rs/)
- [Rust Book](https://doc.rust-lang.org/book/)

## 📝 Notes

- Features marked with ✅ are complete
- Features marked with 🚧 are in progress
- Features marked with [ ] are planned
- Priority levels are subject to change based on user feedback

## 🔄 Last Updated

**Date**: 2026-02-24  
**Version**: 0.1.0  
**Status**: Active Development
