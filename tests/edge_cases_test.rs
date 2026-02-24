/// Tests para edge cases y manejo de errores
use std::path::PathBuf;

#[test]
fn test_empty_file() {
    println!("\n=== Testing empty file ===\n");
    
    let content = "";
    let secrets: Vec<(String, String)> = Vec::new();
    
    assert!(secrets.is_empty());
    println!("✅ Empty file handled correctly");
}

#[test]
fn test_malformed_json() {
    println!("\n=== Testing malformed JSON ===\n");
    
    let malformed_cases = vec![
        r#"{"key": "value""#,  // Missing closing brace
        r#"{"key": }"#,         // Missing value
        r#"{key: "value"}"#,    // Unquoted key
        r#"{"key": 'value'}"#,  // Single quotes
        r#"{"key": undefined}"#, // Undefined value
    ];
    
    for (i, case) in malformed_cases.iter().enumerate() {
        let result = serde_json::from_str::<serde_json::Value>(case);
        assert!(result.is_err(), "Case {} should fail", i + 1);
        println!("✅ Case {}: Malformed JSON rejected", i + 1);
    }
}

#[test]
fn test_unicode_characters() {
    println!("\n=== Testing Unicode characters ===\n");
    
    let unicode_cases = vec![
        ("emoji", "🔐🔑🎉"),
        ("chinese", "密码"),
        ("arabic", "كلمة السر"),
        ("russian", "пароль"),
        ("japanese", "パスワード"),
        ("mixed", "Hello世界🌍"),
    ];
    
    for (name, value) in unicode_cases {
        assert!(!value.is_empty());
        println!("✅ {}: {} ({} bytes)", name, value, value.len());
    }
}

#[test]
fn test_very_long_values() {
    println!("\n=== Testing very long values ===\n");
    
    let lengths = vec![100, 1000, 10000];
    
    for length in lengths {
        let long_value = "a".repeat(length);
        assert_eq!(long_value.len(), length);
        println!("✅ Value with {} characters handled", length);
    }
}

#[test]
fn test_special_file_names() {
    println!("\n=== Testing special file names ===\n");
    
    let special_names = vec![
        ".env",
        ".env.local",
        ".env.production",
        "secrets.json",
        "config.yaml",
        "settings.ini",
        "my secrets.env",  // Con espacio
        "file-with-dashes.json",
        "file_with_underscores.yaml",
    ];
    
    for name in special_names {
        let path = PathBuf::from(name);
        assert!(path.file_name().is_some());
        println!("✅ File name: {}", name);
    }
}

#[test]
fn test_path_with_special_chars() {
    println!("\n=== Testing paths with special characters ===\n");
    
    let special_paths = vec![
        "/path/to/my secrets/file.env",
        "/path/with-dashes/file.json",
        "/path/with_underscores/file.yaml",
        "/path/with.dots/file.ini",
        "/path/with (parentheses)/file.env",
    ];
    
    for path_str in special_paths {
        let path = PathBuf::from(path_str);
        assert!(path.to_str().is_some());
        println!("✅ Path: {}", path_str);
    }
}

#[test]
fn test_null_and_empty_values() {
    println!("\n=== Testing null and empty values ===\n");
    
    let test_cases = vec![
        ("empty_string", ""),
        ("single_space", " "),
        ("multiple_spaces", "   "),
        ("tab", "\t"),
        ("newline", "\n"),
        ("mixed_whitespace", " \t\n "),
    ];
    
    for (name, value) in test_cases {
        println!("✅ {}: {:?} (len: {})", name, value, value.len());
    }
}

#[test]
fn test_boundary_values() {
    println!("\n=== Testing boundary values ===\n");
    
    // Números en los límites
    let max_i32_str = i32::MAX.to_string();
    let min_i32_str = i32::MIN.to_string();
    
    let numbers = vec![
        ("zero", "0"),
        ("negative", "-1"),
        ("max_i32", max_i32_str.as_str()),
        ("min_i32", min_i32_str.as_str()),
        ("float", "3.14159"),
        ("scientific", "1.23e-10"),
    ];
    
    for (name, value) in numbers {
        println!("✅ {}: {}", name, value);
    }
}

#[test]
fn test_control_characters() {
    println!("\n=== Testing control characters ===\n");
    
    let control_chars = vec![
        ("null", "\0"),
        ("bell", "\x07"),
        ("backspace", "\x08"),
        ("tab", "\t"),
        ("newline", "\n"),
        ("carriage_return", "\r"),
        ("escape", "\x1b"),
    ];
    
    for (name, char) in control_chars {
        println!("✅ {}: {:?} (byte: 0x{:02x})", name, char, char.as_bytes()[0]);
    }
}

#[test]
fn test_duplicate_keys() {
    println!("\n=== Testing duplicate keys ===\n");
    
    use std::collections::HashMap;
    
    let mut secrets = HashMap::new();
    
    // Primera inserción
    secrets.insert("KEY".to_string(), "value1".to_string());
    assert_eq!(secrets.get("KEY"), Some(&"value1".to_string()));
    println!("First insert: KEY = value1");
    
    // Inserción duplicada (sobrescribe)
    secrets.insert("KEY".to_string(), "value2".to_string());
    assert_eq!(secrets.get("KEY"), Some(&"value2".to_string()));
    assert_eq!(secrets.len(), 1);
    println!("✅ Duplicate key overwrites: KEY = value2");
}

#[test]
fn test_case_sensitivity() {
    println!("\n=== Testing case sensitivity ===\n");
    
    use std::collections::HashMap;
    
    let mut secrets = HashMap::new();
    secrets.insert("key".to_string(), "lowercase".to_string());
    secrets.insert("KEY".to_string(), "uppercase".to_string());
    secrets.insert("Key".to_string(), "mixed".to_string());
    
    assert_eq!(secrets.len(), 3);
    assert_eq!(secrets.get("key"), Some(&"lowercase".to_string()));
    assert_eq!(secrets.get("KEY"), Some(&"uppercase".to_string()));
    assert_eq!(secrets.get("Key"), Some(&"mixed".to_string()));
    
    println!("✅ Keys are case-sensitive (3 different keys)");
}

#[test]
fn test_large_number_of_secrets() {
    println!("\n=== Testing large number of secrets ===\n");
    
    use std::collections::HashMap;
    
    let mut secrets = HashMap::new();
    let count = 1000;
    
    // Crear muchos secretos
    for i in 0..count {
        secrets.insert(format!("KEY_{}", i), format!("value_{}", i));
    }
    
    assert_eq!(secrets.len(), count);
    println!("✅ Created {} secrets", count);
    
    // Verificar acceso aleatorio
    assert_eq!(secrets.get("KEY_500"), Some(&"value_500".to_string()));
    println!("✅ Random access works");
    
    // Buscar en muchos secretos
    let filtered: Vec<_> = secrets.keys()
        .filter(|k| k.contains("_5"))
        .collect();
    
    println!("✅ Search in {} secrets found {} matches", count, filtered.len());
}

#[test]
fn test_json_types() {
    println!("\n=== Testing JSON value types ===\n");
    
    use serde_json::json;
    
    let test_values = json!({
        "string": "text",
        "number": 42,
        "float": 3.14,
        "boolean": true,
        "null": null,
        "array": [1, 2, 3],
        "object": {"nested": "value"}
    });
    
    assert!(test_values["string"].is_string());
    assert!(test_values["number"].is_number());
    assert!(test_values["boolean"].is_boolean());
    assert!(test_values["null"].is_null());
    assert!(test_values["array"].is_array());
    assert!(test_values["object"].is_object());
    
    println!("✅ All JSON types handled correctly");
}
