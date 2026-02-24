use std::fs;
use std::path::PathBuf;

#[test]
fn test_env_special_characters() {
    // Casos de prueba con caracteres especiales comunes en contraseñas
    let test_cases = vec![
        ("SIMPLE", "password123"),
        ("WITH_EQUALS", "pass=word=123"),
        ("WITH_HASH", "pass#word#123"),
        ("WITH_QUOTES", r#"pass"word"123"#),
        ("WITH_SINGLE_QUOTES", "pass'word'123"),
        ("WITH_SPACES", "pass word 123"),
        ("WITH_SPECIAL", "p@ss!w0rd$%^&*()"),
        ("WITH_BACKSLASH", r"pass\word\123"),
        ("WITH_NEWLINE_ESCAPED", r"pass\nword"),
        ("WITH_TAB_ESCAPED", r"pass\tword"),
        ("WITH_SEMICOLON", "pass;word;123"),
        ("WITH_PIPE", "pass|word|123"),
        ("WITH_AMPERSAND", "pass&word&123"),
        ("WITH_BRACKETS", "pass[word]123"),
        ("WITH_BRACES", "pass{word}123"),
        ("WITH_ANGLE", "pass<word>123"),
        ("WITH_TILDE", "pass~word~123"),
        ("WITH_BACKTICK", "pass`word`123"),
        ("COMPLEX_PASSWORD", r#"P@$$w0rd!#%&*()=+[]{}|;:'",.<>?/\~`"#),
        ("URL_WITH_PARAMS", "https://user:pass@host.com/path?key=value&other=123"),
        ("BASE64_LIKE", "YWJjZGVmZ2hpamtsbW5vcHFyc3R1dnd4eXo="),
        ("JSON_ESCAPED", r#"{"key":"value","nested":{"data":"test"}}"#),
    ];

    println!("\n=== Testing ENV format parsing ===\n");
    
    for (key, value) in &test_cases {
        // Crear contenido .env
        let env_content = format!("{}={}", key, value);
        
        // Simular el parsing que hace el código
        if let Some((parsed_key, parsed_value)) = env_content.split_once('=') {
            let parsed_key = parsed_key.trim();
            let parsed_value = parsed_value.trim();
            
            if parsed_value != *value {
                println!("❌ FAIL: {}", key);
                println!("   Original: {:?}", value);
                println!("   Parsed:   {:?}", parsed_value);
                println!("   Content:  {:?}", env_content);
            } else {
                println!("✅ PASS: {}", key);
            }
        }
    }
}

#[test]
fn test_ini_special_characters() {
    let test_cases = vec![
        ("simple", "password123"),
        ("with_equals", "pass=word=123"),
        ("with_hash", "pass#word#123"),
        ("with_semicolon", "pass;word;123"),
        ("with_quotes", r#"pass"word"123"#),
        ("with_spaces", "pass word 123"),
        ("complex", r#"P@$$w0rd!#%&*()=+[]{}|;:'",.<>?/\~`"#),
    ];

    println!("\n=== Testing INI format parsing ===\n");
    
    for (key, value) in &test_cases {
        let ini_content = format!("[DEFAULT]\n{}={}", key, value);
        
        // Simular el parsing
        for line in ini_content.lines() {
            if line.starts_with('[') || line.trim().is_empty() {
                continue;
            }
            
            if let Some((parsed_key, parsed_value)) = line.split_once('=') {
                let parsed_key = parsed_key.trim();
                let parsed_value = parsed_value.trim();
                
                if parsed_value != *value {
                    println!("❌ FAIL: {}", key);
                    println!("   Original: {:?}", value);
                    println!("   Parsed:   {:?}", parsed_value);
                } else {
                    println!("✅ PASS: {}", key);
                }
            }
        }
    }
}

#[test]
fn test_value_roundtrip() {
    // Test que el valor sobrevive el ciclo completo: parse -> store -> serialize
    let test_values = vec![
        "simple",
        "with=equals=multiple",
        "with spaces and = equals",
        r#"with"quotes"and=equals"#,
        "P@$$w0rd!#%&*()=+[]{}|;:'\",.<>?/\\~`",
    ];

    println!("\n=== Testing value roundtrip ===\n");
    
    for original in &test_values {
        // Simular guardado en ENV
        let env_line = format!("KEY={}", original);
        
        // Simular lectura
        if let Some((_, parsed)) = env_line.split_once('=') {
            // Simular guardado de vuelta
            let reconstructed = format!("KEY={}", parsed);
            
            if reconstructed != env_line {
                println!("❌ FAIL roundtrip");
                println!("   Original:      {:?}", env_line);
                println!("   Reconstructed: {:?}", reconstructed);
            } else {
                println!("✅ PASS roundtrip: {:?}", original);
            }
        }
    }
}

#[test]
fn test_json_escaping() {
    use serde_json::Value;
    
    let test_values = vec![
        r#"simple"#,
        r#"with"quotes"#,
        r#"with\backslash"#,
        r#"with\nnewline"#,
        r#"with=equals"#,
        r#"P@$$w0rd!#%&*()=+[]{}|;:'",.<>?/\~`"#,
    ];

    println!("\n=== Testing JSON escaping ===\n");
    
    for original in &test_values {
        // Crear JSON value
        let json_value = Value::String(original.to_string());
        
        // Serializar
        let serialized = serde_json::to_string(&json_value).unwrap();
        
        // Deserializar
        let deserialized: Value = serde_json::from_str(&serialized).unwrap();
        
        if let Some(result) = deserialized.as_str() {
            if result != *original {
                println!("❌ FAIL JSON roundtrip");
                println!("   Original:     {:?}", original);
                println!("   Serialized:   {:?}", serialized);
                println!("   Deserialized: {:?}", result);
            } else {
                println!("✅ PASS JSON roundtrip: {:?}", original);
            }
        }
    }
}
