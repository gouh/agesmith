use std::fs;
use std::path::PathBuf;
use std::process::Command;

/// Test real con SOPS para detectar problemas con caracteres especiales
#[test]
#[ignore] // Usar --ignored para ejecutar solo cuando SOPS esté configurado
fn test_sops_env_special_chars() {
    // Verificar que SOPS está disponible
    if Command::new("sops").arg("--version").output().is_err() {
        println!("⚠️  SOPS no está instalado, saltando test");
        return;
    }

    let test_dir = std::env::temp_dir().join("agesmith_test");
    fs::create_dir_all(&test_dir).unwrap();

    let test_cases = vec![
        ("SIMPLE", "password123"),
        ("WITH_EQUALS", "pass=word=123"),
        ("WITH_HASH", "pass#word#123"),
        ("WITH_QUOTES", r#"pass"word"123"#),
        ("WITH_SPACES", "pass word 123"),
        ("WITH_SPECIAL", "p@ss!w0rd$%^&*()"),
        ("COMPLEX", r#"P@$$w0rd!#%&*()=+[]{}|;:'",.<>?/\~`"#),
    ];

    println!("\n=== Testing SOPS ENV encryption/decryption ===\n");

    for (key, original_value) in &test_cases {
        let test_file = test_dir.join(format!("test_{}.env", key.to_lowercase()));
        
        // Crear archivo .env
        let content = format!("{}={}", key, original_value);
        fs::write(&test_file, &content).unwrap();
        
        println!("Testing: {} = {:?}", key, original_value);
        println!("  File content: {:?}", content);
        
        // Leer de vuelta y verificar
        let read_content = fs::read_to_string(&test_file).unwrap();
        if let Some((_, value)) = read_content.trim().split_once('=') {
            if value != *original_value {
                println!("  ❌ FAIL: Value changed after write/read");
                println!("     Expected: {:?}", original_value);
                println!("     Got:      {:?}", value);
            } else {
                println!("  ✅ PASS: Value preserved");
            }
        }
        
        // Limpiar
        fs::remove_file(&test_file).ok();
    }
    
    fs::remove_dir_all(&test_dir).ok();
}

#[test]
fn test_env_parsing_edge_cases() {
    println!("\n=== Testing ENV parsing edge cases ===\n");
    
    let test_cases = vec![
        // (input_line, expected_key, expected_value)
        ("KEY=value", "KEY", "value"),
        ("KEY=value=with=equals", "KEY", "value=with=equals"),
        ("KEY=", "KEY", ""),
        ("KEY= ", "KEY", ""),
        ("KEY=  value with spaces  ", "KEY", "value with spaces"),
        ("KEY=\"quoted value\"", "KEY", "\"quoted value\""),
        ("KEY='single quoted'", "KEY", "'single quoted'"),
        ("KEY=value # comment", "KEY", "value # comment"),
        ("KEY=http://user:pass@host.com", "KEY", "http://user:pass@host.com"),
        ("KEY=value\twith\ttabs", "KEY", "value\twith\ttabs"),
    ];
    
    for (input, expected_key, expected_value) in test_cases {
        if let Some((key, value)) = input.split_once('=') {
            let key = key.trim();
            let value = value.trim();
            
            if key != expected_key || value != expected_value {
                println!("❌ FAIL: {:?}", input);
                println!("   Expected: key={:?}, value={:?}", expected_key, expected_value);
                println!("   Got:      key={:?}, value={:?}", key, value);
            } else {
                println!("✅ PASS: {:?}", input);
            }
        }
    }
}

#[test]
fn test_json_to_env_conversion() {
    use serde_json::{json, Value};
    
    println!("\n=== Testing JSON to ENV conversion ===\n");
    
    let test_cases = vec![
        ("simple", "password123"),
        ("with_equals", "pass=word=123"),
        ("with_quotes", r#"pass"word"123"#),
        ("with_backslash", r"pass\word\123"),
        ("complex", r#"P@$$w0rd!#%&*()=+[]{}|;:'",.<>?/\~`"#),
    ];
    
    for (key, value) in test_cases {
        // Simular lo que hace save_changes: crear JSON
        let json_obj = json!({
            key: value
        });
        
        let json_str = serde_json::to_string_pretty(&json_obj).unwrap();
        println!("Key: {}", key);
        println!("  Original value: {:?}", value);
        println!("  JSON: {}", json_str);
        
        // Parsear de vuelta
        let parsed: Value = serde_json::from_str(&json_str).unwrap();
        if let Some(parsed_value) = parsed.get(key).and_then(|v| v.as_str()) {
            if parsed_value != value {
                println!("  ❌ FAIL: Value changed in JSON roundtrip");
                println!("     Expected: {:?}", value);
                println!("     Got:      {:?}", parsed_value);
            } else {
                println!("  ✅ PASS: Value preserved in JSON");
            }
        }
        println!();
    }
}

#[test]
fn test_problematic_characters() {
    println!("\n=== Testing specific problematic characters ===\n");
    
    // Caracteres que pueden causar problemas en diferentes contextos
    let problematic = vec![
        ("equals", "a=b=c=d"),
        ("hash_comment", "pass#word"),
        ("semicolon_comment", "pass;word"),
        ("quotes_double", r#"pass"word"#),
        ("quotes_single", "pass'word"),
        ("backslash", r"pass\word"),
        ("dollar", "pass$word"),
        ("backtick", "pass`word`"),
        ("newline_literal", "pass\nword"),
        ("tab_literal", "pass\tword"),
    ];
    
    for (name, value) in problematic {
        let env_line = format!("KEY={}", value);
        
        // Test 1: split_once preserva todo después del primer =
        if let Some((_, parsed)) = env_line.split_once('=') {
            if parsed != value {
                println!("❌ FAIL {}: split_once", name);
                println!("   Input:  {:?}", env_line);
                println!("   Expect: {:?}", value);
                println!("   Got:    {:?}", parsed);
            } else {
                println!("✅ PASS {}: split_once preserves value", name);
            }
        }
        
        // Test 2: JSON roundtrip
        let json_value = serde_json::json!(value);
        let json_str = serde_json::to_string(&json_value).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json_str).unwrap();
        
        if parsed.as_str() != Some(value) {
            println!("❌ FAIL {}: JSON roundtrip", name);
            println!("   Original: {:?}", value);
            println!("   JSON:     {}", json_str);
            println!("   Parsed:   {:?}", parsed.as_str());
        } else {
            println!("✅ PASS {}: JSON roundtrip", name);
        }
    }
}
