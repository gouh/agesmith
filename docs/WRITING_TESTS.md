# Writing Tests for AgeSmith

## Quick Start

### 1. Choose Test Type

- **Unit Test**: Testing a single function or component
- **Integration Test**: Testing multiple components working together
- **E2E Test**: Testing with real SOPS encryption

### 2. Create Test File

```bash
# Unit test
touch tests/my_feature_test.rs

# Integration test
touch tests/my_integration_test.rs

# E2E test (requires SOPS)
touch tests/my_e2e_test.rs
```

### 3. Basic Test Template

```rust
#[test]
fn test_my_feature() {
    // Arrange - Set up test data
    let input = "test_value";
    
    // Act - Execute the function
    let result = my_function(input);
    
    // Assert - Verify the result
    assert_eq!(result, expected_value);
}
```

## Test Patterns

### Testing with Special Characters

```rust
#[test]
fn test_special_characters() {
    let test_cases = vec![
        ("simple", "password123"),
        ("with_hash", "pass#word"),
        ("with_semicolon", "pass;word"),
        ("complex", "P@$$w0rd!#%"),
    ];
    
    for (name, value) in test_cases {
        // Your test logic here
        assert!(!value.is_empty());
        println!("✅ {}: {}", name, value);
    }
}
```

### Testing JSON Operations

```rust
#[test]
fn test_json_roundtrip() {
    use serde_json::{json, Value};
    
    let original = "pass#word";
    let json_val = Value::String(original.to_string());
    
    // Serialize
    let json_str = serde_json::to_string(&json_val).unwrap();
    
    // Deserialize
    let parsed: Value = serde_json::from_str(&json_str).unwrap();
    let result = parsed.as_str().unwrap();
    
    assert_eq!(result, original);
}
```

### Testing with HashMap (Secrets)

```rust
#[test]
fn test_secret_operations() {
    use std::collections::HashMap;
    
    let mut secrets = HashMap::new();
    
    // Create
    secrets.insert("KEY".to_string(), "value".to_string());
    assert_eq!(secrets.len(), 1);
    
    // Read
    assert_eq!(secrets.get("KEY"), Some(&"value".to_string()));
    
    // Update
    secrets.insert("KEY".to_string(), "new_value".to_string());
    assert_eq!(secrets.get("KEY"), Some(&"new_value".to_string()));
    
    // Delete
    secrets.remove("KEY");
    assert_eq!(secrets.len(), 0);
}
```

### Testing Edge Cases

```rust
#[test]
fn test_edge_cases() {
    let edge_cases = vec![
        ("empty", ""),
        ("whitespace", "   "),
        ("unicode", "🔐密码"),
        ("very_long", &"a".repeat(10000)),
    ];
    
    for (name, value) in edge_cases {
        // Test that your code handles these cases
        println!("Testing: {}", name);
        // Your assertions here
    }
}
```

## Using Test Helpers

```rust
// Import helpers (if available)
use crate::helpers::*;

#[test]
fn test_with_helpers() {
    // Use pre-made test data
    let secrets = create_test_secrets();
    
    // Use helper functions
    assert!(needs_quoting("pass#word"));
    assert!(!needs_quoting("simple"));
    
    // Verify roundtrip
    assert!(verify_roundtrip("pass#word"));
}
```

## Best Practices

### 1. Descriptive Names

```rust
// ❌ Bad
#[test]
fn test1() { }

// ✅ Good
#[test]
fn test_password_with_hash_character() { }
```

### 2. Clear Output

```rust
#[test]
fn test_with_output() {
    println!("\n=== Testing my feature ===\n");
    
    let result = my_function();
    println!("Result: {:?}", result);
    
    assert!(result.is_ok());
    println!("✅ Test passed");
}
```

### 3. Multiple Assertions

```rust
#[test]
fn test_multiple_conditions() {
    let value = "test";
    
    assert!(!value.is_empty(), "Value should not be empty");
    assert_eq!(value.len(), 4, "Value should have 4 characters");
    assert!(value.is_ascii(), "Value should be ASCII");
}
```

### 4. Test Organization

```rust
#[test]
fn test_feature_a() {
    // Test one aspect
}

#[test]
fn test_feature_b() {
    // Test another aspect
}

#[test]
fn test_feature_integration() {
    // Test both together
}
```

## Running Tests

```bash
# Run all tests
cargo test

# Run specific test file
cargo test --test my_feature_test

# Run specific test function
cargo test test_my_feature

# Run with output
cargo test -- --nocapture

# Run with specific pattern
cargo test password
```

## Debugging Tests

### Print Debug Info

```rust
#[test]
fn test_with_debug() {
    let value = "test";
    println!("Debug: {:?}", value);
    dbg!(&value);  // Alternative
}
```

### Conditional Compilation

```rust
#[test]
#[ignore]  // Skip by default
fn test_slow_operation() {
    // This test is slow
}

// Run with: cargo test -- --ignored
```

### Test-only Code

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_internal() {
        // Test code here
    }
}
```

## Common Patterns

### Testing Errors

```rust
#[test]
fn test_error_handling() {
    let result = function_that_fails();
    assert!(result.is_err());
    
    if let Err(e) = result {
        assert!(e.to_string().contains("expected error"));
    }
}
```

### Testing with Regex

```rust
#[test]
fn test_with_regex() {
    use regex::Regex;
    
    let pattern = Regex::new(r"^[0-9a-f]+$").unwrap();
    let hex = "abc123";
    
    assert!(pattern.is_match(hex));
}
```

### Testing Collections

```rust
#[test]
fn test_collections() {
    let items = vec!["a", "b", "c"];
    
    assert_eq!(items.len(), 3);
    assert!(items.contains(&"b"));
    assert_eq!(items[0], "a");
}
```

## Tips

1. **Keep tests simple** - One concept per test
2. **Use descriptive names** - Test name should explain what it tests
3. **Test edge cases** - Empty, null, very large, special characters
4. **Use helpers** - Don't repeat code
5. **Add output** - Use `println!` for debugging
6. **Test failures** - Not just success cases
7. **Keep tests fast** - Unit tests should be < 1ms
8. **Document complex tests** - Add comments explaining why

## Example: Complete Test File

```rust
/// Tests for password validation
use std::collections::HashMap;

#[test]
fn test_password_validation() {
    println!("\n=== Testing password validation ===\n");
    
    let valid_passwords = vec![
        "simple123",
        "P@ssw0rd!",
        "very_long_password_123",
    ];
    
    for password in valid_passwords {
        assert!(is_valid_password(password));
        println!("✅ Valid: {}", password);
    }
}

#[test]
fn test_invalid_passwords() {
    let invalid = vec![
        "",           // Empty
        "123",        // Too short
        "   ",        // Only spaces
    ];
    
    for password in invalid {
        assert!(!is_valid_password(password));
        println!("✅ Rejected: {:?}", password);
    }
}

// Helper function for tests
fn is_valid_password(password: &str) -> bool {
    !password.trim().is_empty() && password.len() >= 4
}
```

## Resources

- [Rust Book - Testing](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Cargo Test Documentation](https://doc.rust-lang.org/cargo/commands/cargo-test.html)
- Project tests: `tests/` directory
- Test helpers: `tests/helpers/mod.rs`
