# Test Suite Organization

## Structure

```
tests/
├── unit/                    # Unit tests (fast, no I/O)
│   ├── config_test.rs      # Configuration management
│   ├── generator_test.rs   # Secret generator
│   ├── i18n_test.rs        # Internationalization
│   ├── special_chars_test.rs
│   ├── comment_handling_test.rs
│   ├── quoting_test.rs
│   └── json_extraction_test.rs
│
├── integration/             # Integration tests (moderate speed)
│   ├── crud_operations_test.rs
│   ├── edge_cases_test.rs
│   ├── agesmith_flow_test.rs
│   ├── integration_quoting_test.rs
│   └── sops_special_chars_test.rs
│
├── e2e/                     # End-to-end tests (slow, requires SOPS)
│   ├── sops_integration_test.rs
│   ├── read_test.rs
│   └── storage_test.rs
│
├── helpers/                 # Shared test utilities
│   └── mod.rs
│
└── diagnose_sops.sh        # Manual diagnostic script

```

## Running Tests

### All tests
```bash
cargo test
```

### By category
```bash
# Unit tests only (fast)
cargo test --test '*' --lib

# Integration tests
cargo test --test 'integration/*'

# E2E tests (requires SOPS setup)
cargo test --test 'e2e/*'
```

### Specific test file
```bash
cargo test --test config_test
cargo test --test crud_operations_test
```

### With output
```bash
cargo test -- --nocapture
cargo test --test agesmith_flow_test -- --nocapture
```

### Ignored tests (require SOPS configuration)
```bash
cargo test -- --ignored
```

## Test Coverage

### Unit Tests (8 files, ~40 tests)
- ✅ Character handling (special chars, comments, quoting)
- ✅ JSON extraction methods
- ✅ Configuration management
- ✅ Secret generator
- ✅ Internationalization

### Integration Tests (5 files, ~30 tests)
- ✅ CRUD operations (create, read, update, delete)
- ✅ Search and filtering
- ✅ Bulk operations
- ✅ Edge cases (unicode, long values, malformed data)
- ✅ Complete flow simulation

### E2E Tests (3 files, ~10 tests)
- ✅ SOPS integration
- ✅ File reading/writing
- ✅ Storage operations

## Test Helpers

Located in `tests/helpers/mod.rs`:

- `create_test_secrets()` - Generate test data
- `needs_quoting()` - Check if value needs quotes
- `quote_value()` - Quote and escape values
- `unquote_value()` - Unquote and unescape values
- `verify_roundtrip()` - Test value preservation

## Coverage Goals

- [x] Character handling: 100%
- [x] CRUD operations: 100%
- [x] Edge cases: 80%
- [ ] UI components: 0% (TUI testing is complex)
- [ ] Event handling: 0% (requires mock terminal)
- [x] Configuration: 90%
- [x] I18n: 80%
- [x] Generator: 90%

## Adding New Tests

### 1. Unit Test
```rust
// tests/unit/my_feature_test.rs
#[test]
fn test_my_feature() {
    // Arrange
    let input = "test";
    
    // Act
    let result = my_function(input);
    
    // Assert
    assert_eq!(result, expected);
}
```

### 2. Integration Test
```rust
// tests/integration/my_flow_test.rs
#[test]
fn test_complete_flow() {
    // Setup
    let mut state = setup_test_state();
    
    // Execute flow
    state.step1();
    state.step2();
    
    // Verify
    assert!(state.is_valid());
}
```

### 3. Using Helpers
```rust
use crate::helpers::*;

#[test]
fn test_with_helpers() {
    let secrets = create_test_secrets();
    assert!(verify_roundtrip("pass#word"));
}
```

## CI/CD Integration

```yaml
# .github/workflows/test.yml
- name: Run tests
  run: |
    cargo test --all
    cargo test -- --ignored  # If SOPS is configured
```

## Performance

- Unit tests: ~0.1s total
- Integration tests: ~0.5s total
- E2E tests: ~2s total (depends on SOPS)
- **Total: ~3s for full suite**

## Notes

- E2E tests may fail if SOPS is not configured
- Use `#[ignore]` for tests requiring external setup
- Keep unit tests fast (no I/O, no network)
- Use helpers to reduce code duplication
