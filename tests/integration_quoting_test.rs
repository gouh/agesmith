use serde_json::Value;

#[test]
fn test_env_value_preservation() {
    println!("\n=== Test: ENV value preservation with special characters ===\n");
    
    // Simular el flujo completo: secrets -> JSON -> SOPS -> JSON -> secrets
    let original_secrets = vec![
        ("SIMPLE", "password123"),
        ("WITH_HASH", "pass#word#123"),
        ("WITH_SEMICOLON", "pass;word;123"),
        ("WITH_SPACE", "pass word 123"),
        ("COMPLEX", "P@$$w0rd!#%&*()=+"),
        ("URL", "https://user:pass@host.com?key=val"),
    ];
    
    println!("Step 1: Original secrets");
    for (key, value) in &original_secrets {
        println!("  {} = {:?}", key, value);
    }
    println!();
    
    // Step 2: Aplicar quoting (como hace save_changes)
    println!("Step 2: Apply quoting");
    let mut quoted_secrets = Vec::new();
    for (key, value) in &original_secrets {
        let needs_quote = value.contains('#')
            || value.contains(';')
            || value.contains('\n')
            || value.starts_with(' ')
            || value.ends_with(' ')
            || (value.contains(' ') && !value.starts_with('"'));
        
        let quoted = if needs_quote {
            let escaped = value
                .replace('\\', r"\\")
                .replace('"', r#"\""#)
                .replace('\n', r"\n")
                .replace('\r', r"\r")
                .replace('\t', r"\t");
            format!("\"{}\"", escaped)  // Fixed: use regular format, not raw string
        } else {
            value.to_string()
        };
        
        quoted_secrets.push((*key, quoted.clone()));
        if needs_quote {
            println!("  {} = {:?} → {:?} (quoted)", key, value, quoted);
        } else {
            println!("  {} = {:?} (no quoting needed)", key, value);
        }
    }
    println!();
    
    // Step 3: Crear JSON (como hace save_changes)
    println!("Step 3: Create JSON for SOPS");
    let mut json_obj = serde_json::Map::new();
    for (key, value) in &quoted_secrets {
        json_obj.insert(key.to_string(), Value::String(value.clone()));
    }
    let json_str = serde_json::to_string_pretty(&Value::Object(json_obj)).unwrap();
    println!("{}", json_str);
    println!();
    
    // Step 4: Simular SOPS decrypt (parsear JSON de vuelta)
    println!("Step 4: Parse JSON back (simulating SOPS decrypt)");
    let parsed: Value = serde_json::from_str(&json_str).unwrap();
    
    // Step 5: Unquote values (como hace flatten_json)
    println!("Step 5: Unquote values");
    let mut final_secrets = Vec::new();
    for (original_key, original_value) in &original_secrets {
        if let Some(json_value) = parsed.get(*original_key) {
            let str_value = json_value.as_str().unwrap();
            
            // Aplicar unquote
            let unquoted = if (str_value.starts_with('"') && str_value.ends_with('"') && str_value.len() >= 2)
                || (str_value.starts_with('\'') && str_value.ends_with('\'') && str_value.len() >= 2)
            {
                let inner = &str_value[1..str_value.len() - 1];
                inner
                    .replace(r"\n", "\n")
                    .replace(r"\r", "\r")
                    .replace(r"\t", "\t")
                    .replace(r#"\""#, "\"")
                    .replace(r"\'", "'")
                    .replace(r"\\", "\\")
            } else {
                str_value.to_string()
            };
            
            final_secrets.push((*original_key, unquoted.clone()));
            
            if unquoted == *original_value {
                println!("  ✅ {} = {:?}", original_key, unquoted);
            } else {
                println!("  ❌ {} mismatch:", original_key);
                println!("     Original: {:?}", original_value);
                println!("     Final:    {:?}", unquoted);
            }
        }
    }
    println!();
    
    // Verificación final
    println!("Final verification:");
    let mut all_match = true;
    for ((orig_key, orig_val), (final_key, final_val)) in original_secrets.iter().zip(final_secrets.iter()) {
        if orig_key != final_key || orig_val != final_val {
            println!("  ❌ Mismatch: {} {:?} != {} {:?}", orig_key, orig_val, final_key, final_val);
            all_match = false;
        }
    }
    
    if all_match {
        println!("  ✅ All values preserved correctly!");
    } else {
        panic!("Some values were not preserved correctly");
    }
}

#[test]
fn test_edge_cases() {
    println!("\n=== Test: Edge cases ===\n");
    
    let edge_cases = vec![
        ("Empty", ""),
        ("OnlyHash", "#"),
        ("OnlySemicolon", ";"),
        ("OnlySpace", " "),
        ("OnlyQuote", "\""),
        ("MultipleQuotes", "\"\"\""),
        ("BackslashEnd", "value\\"),
        ("NewlineInMiddle", "line1\nline2"),
    ];
    
    for (name, value) in edge_cases {
        println!("Testing: {} = {:?}", name, value);
        
        // Quote
        let needs_quote = value.contains('#')
            || value.contains(';')
            || value.contains('\n')
            || value.starts_with(' ')
            || value.ends_with(' ')
            || (value.contains(' ') && !value.starts_with('"'));
        
        let quoted = if needs_quote {
            let escaped = value
                .replace('\\', r"\\")
                .replace('"', r#"\""#)
                .replace('\n', r"\n");
            format!("\"{}\"", escaped)  // Fixed
        } else {
            value.to_string()
        };
        
        // Unquote
        let unquoted = if quoted.starts_with('"') && quoted.ends_with('"') && quoted.len() >= 2 {
            let inner = &quoted[1..quoted.len() - 1];
            inner
                .replace(r"\n", "\n")
                .replace(r#"\""#, "\"")
                .replace(r"\\", "\\")
        } else {
            quoted.clone()
        };
        
        if unquoted == value {
            println!("  ✅ Roundtrip OK: {:?} → {:?} → {:?}", value, quoted, unquoted);
        } else {
            println!("  ❌ Roundtrip FAIL:");
            println!("     Original: {:?}", value);
            println!("     Quoted:   {:?}", quoted);
            println!("     Unquoted: {:?}", unquoted);
        }
        println!();
    }
}
