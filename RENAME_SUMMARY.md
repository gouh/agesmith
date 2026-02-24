# 🎉 Project Renamed to AgeSmith

## ✅ Changes Applied

### 1. Package Name
**Cargo.toml**:
```toml
name = "agesmith"
description = "Forging secure secrets with age"
```

### 2. Binary Name
- Old: `tui-sops`
- New: `agesmith`
- Location: `target/release/agesmith`

### 3. Configuration Paths
- Old: `~/.config/tui-sops/`
- New: `~/.config/agesmith/`

Files affected:
- `~/.config/agesmith/config.toml`
- `~/.config/agesmith/favorites.json`

### 4. Documentation
- ✅ README.md - Updated with new name and tagline
- ✅ config.toml.example - Updated paths
- ✅ All references changed

### 5. Default Language
- Changed from Spanish to English
- Config default: `language = "en"`

## 🚀 Usage

### Build
```bash
cargo build --release
```

### Run
```bash
# From source
cargo run

# Binary
./target/release/agesmith

# With path
./target/release/agesmith /path/to/secrets
```

### Install (optional)
```bash
cargo install --path .
# Then use: agesmith
```

## 📝 Configuration

Create `~/.config/agesmith/config.toml`:
```toml
theme = "dark"
auto_lock_minutes = 15
clipboard_clear_seconds = 3
language = "en"  # or "es"
```

## 🎨 Branding

**Name**: AgeSmith
**Tagline**: "Forging secure secrets with age"
**Binary**: `agesmith`
**Config**: `~/.config/agesmith/`

## 📦 Next Steps

1. Update git remote if needed
2. Rename directory: `mv tui-sops agesmith`
3. Update any scripts/aliases
4. Consider publishing to crates.io

## ✨ Compilation

```
✅ Compiles successfully
✅ Binary: 3.5M
✅ No errors
⚠️  1 warning (unused code - normal)
```

---

**Date**: 2026-02-23
**Version**: 0.1.0
**Status**: ✅ Ready to use
