use serde_json::{json, Value};

/// Test de las funciones de entrecomillado
#[test]
fn test_quoting_functions() {
    println!("\n=== Testing quoting/unquoting functions ===\n");
    
    let test_cases = vec![
        ("simple", "simple", false),
        ("pass#word", r#""pass#word""#, true),
        ("pass;word", r#""pass;word""#, true),
        ("pass word", r#""pass word""#, true),
        (" leading", r#"" leading""#, true),
        ("trailing ", r#""trailing ""#, true),
        ("pass=word", "pass=word", false), // = no necesita comillas
        (r#"pass"word"#, r#""pass\"word""#, true),
        ("pass\nword", r#""pass\nword""#, true),
        ("P@$$w0rd!#%", r#""P@$$w0rd!#%""#, true),
    ];
    
    for (original, expected_quoted, should_quote) in test_cases {
        println!("Testing: {:?}", original);
        
        // Simular needs_quoting
        let needs_quote = original.contains('#')
            || original.contains(';')
            || original.contains('\n')
            || original.contains('\r')
            || original.starts_with(' ')
            || original.ends_with(' ')
            || (original.contains(' ') && !original.starts_with('"') && !original.ends_with('"'));
        
        if needs_quote != should_quote {
            println!("  ❌ needs_quoting mismatch: expected {}, got {}", should_quote, needs_quote);
        } else {
            println!("  ✅ needs_quoting: {}", needs_quote);
        }
        
        // Simular quote_env_value
        let quoted = if needs_quote {
            let escaped = original
                .replace('\\', r"\\")
                .replace('"', r#"\""#)
                .replace('\n', r"\n")
                .replace('\r', r"\r")
                .replace('\t', r"\t");
            format!(r#""{}"#, escaped)
        } else {
            original.to_string()
        };
        
        if quoted != expected_quoted {
            println!("  ❌ Quoted mismatch:");
            println!("     Expected: {:?}", expected_quoted);
            println!("     Got:      {:?}", quoted);
        } else {
            println!("  ✅ Quoted: {:?}", quoted);
        }
        
        // Simular unquote_env_value
        let unquoted = {
            let trimmed = quoted.trim();
            if (trimmed.starts_with('"') && trimmed.ends_with('"') && trimmed.len() >= 2)
                || (trimmed.starts_with('\'') && trimmed.ends_with('\'') && trimmed.len() >= 2)
            {
                let unquoted = &trimmed[1..trimmed.len() - 1];
                unquoted
                    .replace(r"\n", "\n")
                    .replace(r"\r", "\r")
                    .replace(r"\t", "\t")
                    .replace(r#"\""#, "\"")
                    .replace(r"\'", "'")
                    .replace(r"\\", "\\")
            } else {
                trimmed.to_string()
            }
        };
        
        if unquoted != original {
            println!("  ❌ Roundtrip failed:");
            println!("     Original:  {:?}", original);
            println!("     Quoted:    {:?}", quoted);
            println!("     Unquoted:  {:?}", unquoted);
        } else {
            println!("  ✅ Roundtrip successful");
        }
        
        println!();
    }
}

#[test]
fn test_json_with_quoted_values() {
    println!("\n=== Testing JSON with quoted values ===\n");
    
    let test_cases = vec![
        ("SIMPLE", "password123"),
        ("WITH_HASH", r#""pass#word#123""#),
        ("WITH_SEMICOLON", r#""pass;word;123""#),
        ("WITH_SPACE", r#""pass word 123""#),
        ("COMPLEX", r#""P@$$w0rd!#%&*()""#),
    ];
    
    // Simular lo que hace save_changes
    let mut json_obj = serde_json::Map::new();
    for (key, value) in &test_cases {
        json_obj.insert(key.to_string(), Value::String(value.to_string()));
    }
    
    let json_value = Value::Object(json_obj);
    let json_str = serde_json::to_string_pretty(&json_value).unwrap();
    
    println!("Generated JSON:");
    println!("{}", json_str);
    println!();
    
    // Parsear de vuelta
    let parsed: Value = serde_json::from_str(&json_str).unwrap();
    
    println!("Verifying values:");
    for (key, expected_value) in &test_cases {
        if let Some(actual_value) = parsed.get(*key).and_then(|v| v.as_str()) {
            if actual_value == *expected_value {
                println!("  ✅ {}: {:?}", key, actual_value);
            } else {
                println!("  ❌ {} mismatch:", key);
                println!("     Expected: {:?}", expected_value);
                println!("     Got:      {:?}", actual_value);
            }
        } else {
            println!("  ❌ {}: not found or not a string", key);
        }
    }
}

#[test]
fn test_problematic_passwords() {
    println!("\n=== Testing real-world problematic passwords ===\n");
    
    let passwords = vec![
        "MyP@ssw0rd#2024",           // Hash
        "Secret;Key;123",             // Semicolon
        "Pass Word With Spaces",     // Spaces
        "a=b=c",                      // Multiple equals
        r#"Has"Quotes"Inside"#,       // Quotes
        "Mix#All;Of=Them Together",  // Multiple special chars
        "https://user:pass@host.com?key=val&other=123", // URL
    ];
    
    for password in passwords {
        println!("Password: {:?}", password);
        
        // Verificar si necesita comillas
        let needs_quote = password.contains('#')
            || password.contains(';')
            || password.contains('\n')
            || password.starts_with(' ')
            || password.ends_with(' ')
            || (password.contains(' ') && !password.starts_with('"'));
        
        if needs_quote {
            println!("  ⚠️  Needs quoting");
            
            // Entrecomillar
            let escaped = password
                .replace('\\', r"\\")
                .replace('"', r#"\""#);
            let quoted = format!(r#""{}"#, escaped);
            println!("  → Quoted: {:?}", quoted);
            
            // Desescapar
            let unquoted = if quoted.starts_with('"') && quoted.ends_with('"') {
                let inner = &quoted[1..quoted.len() - 1];
                inner
                    .replace(r#"\""#, "\"")
                    .replace(r"\\", "\\")
            } else {
                quoted.clone()
            };
            
            if unquoted == password {
                println!("  ✅ Roundtrip successful");
            } else {
                println!("  ❌ Roundtrip failed: {:?}", unquoted);
            }
        } else {
            println!("  ✅ No quoting needed");
        }
        println!();
    }
}
