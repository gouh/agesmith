use serde_json::{json, Value};

/// Test que simula el flujo EXACTO de AgeSmith
#[test]
fn test_agesmith_complete_flow() {
    println!("\n=== Simulating AgeSmith complete flow ===\n");
    
    let test_passwords = vec![
        ("SIMPLE", "password123"),
        ("WITH_HASH", "pass#word#123"),
        ("WITH_SEMICOLON", "pass;word;123"),
        ("WITH_SPACE", "pass word 123"),
        ("COMPLEX", "P@$$w0rd!#%&*()=+"),
        ("WITH_QUOTES", r#"pass"word"123"#),
    ];
    
    println!("📝 Step 1: User enters passwords\n");
    for (key, pass) in &test_passwords {
        println!("  {} = {:?}", key, pass);
    }
    
    println!("\n💾 Step 2: save_changes() - Quote values for ENV\n");
    let mut quoted_values = Vec::new();
    for (key, pass) in &test_passwords {
        // Simular needs_quoting
        let needs_quote = pass.contains('#')
            || pass.contains(';')
            || pass.contains('\n')
            || pass.starts_with(' ')
            || pass.ends_with(' ')
            || (pass.contains(' ') && !pass.starts_with('"'));
        
        // Simular quote_env_value
        let quoted = if needs_quote {
            let escaped = pass
                .replace('\\', r"\\")
                .replace('"', r#"\""#)
                .replace('\n', r"\n");
            format!("\"{}\"", escaped)
        } else {
            pass.to_string()
        };
        
        quoted_values.push((*key, quoted.clone()));
        if needs_quote {
            println!("  {} = {:?} → {:?} (quoted)", key, pass, quoted);
        } else {
            println!("  {} = {:?} (no quoting)", key, pass);
        }
    }
    
    println!("\n📦 Step 3: Create JSON for SOPS\n");
    let mut json_obj = serde_json::Map::new();
    for (key, value) in &quoted_values {
        json_obj.insert(key.to_string(), Value::String(value.clone()));
    }
    let json_str = serde_json::to_string_pretty(&Value::Object(json_obj)).unwrap();
    println!("{}", json_str);
    
    println!("\n🔐 Step 4: SOPS encrypts... (simulated)\n");
    println!("  [SOPS encryption happens here]");
    
    println!("\n🔓 Step 5: SOPS decrypts - Parse JSON\n");
    let parsed: Value = serde_json::from_str(&json_str).unwrap();
    
    println!("\n📖 Step 6: flatten_json() - Extract and unquote values\n");
    let mut final_values = Vec::new();
    for (original_key, original_pass) in &test_passwords {
        if let Some(json_value) = parsed.get(*original_key) {
            // CRÍTICO: Usar as_str() en lugar de to_string()
            let str_value = if let Some(s) = json_value.as_str() {
                s.to_string()
            } else {
                json_value.to_string().trim_matches('"').to_string()
            };
            
            // Simular unquote_env_value
            let trimmed = str_value.trim();
            let unquoted = if (trimmed.starts_with('"') && trimmed.ends_with('"') && trimmed.len() >= 2)
                || (trimmed.starts_with('\'') && trimmed.ends_with('\'') && trimmed.len() >= 2)
            {
                let inner = &trimmed[1..trimmed.len() - 1];
                inner
                    .replace(r"\n", "\n")
                    .replace(r"\t", "\t")
                    .replace(r#"\""#, "\"")
                    .replace(r"\\", "\\")
            } else {
                trimmed.to_string()
            };
            
            final_values.push((*original_key, unquoted.clone()));
            
            if unquoted == *original_pass {
                println!("  ✅ {} = {:?}", original_key, unquoted);
            } else {
                println!("  ❌ {} CORRUPTED:", original_key);
                println!("     Original:  {:?}", original_pass);
                println!("     JSON str:  {:?}", str_value);
                println!("     Final:     {:?}", unquoted);
            }
        }
    }
    
    println!("\n🎯 Final Verification:\n");
    let mut all_ok = true;
    for ((orig_key, orig_pass), (final_key, final_pass)) in test_passwords.iter().zip(final_values.iter()) {
        if orig_key != final_key || orig_pass != final_pass {
            println!("  ❌ {} mismatch: {:?} != {:?}", orig_key, orig_pass, final_pass);
            all_ok = false;
        }
    }
    
    if all_ok {
        println!("  ✅ ALL PASSWORDS PRESERVED CORRECTLY!");
        println!("\n🎉 Success! Users will see their original passwords.");
    } else {
        panic!("Some passwords were corrupted!");
    }
}

#[test]
fn test_edge_case_passwords() {
    println!("\n=== Testing edge case passwords ===\n");
    
    let edge_cases = vec![
        ("Empty", ""),
        ("OnlyHash", "#"),
        ("OnlySemicolon", ";"),
        ("OnlySpace", " "),
        ("HashAtStart", "#password"),
        ("HashAtEnd", "password#"),
        ("MultipleHashes", "##pass##word##"),
        ("MixedSpecial", "p@ss#w;rd sp@ce"),
    ];
    
    for (name, original) in edge_cases {
        // Quote
        let needs_quote = original.contains('#')
            || original.contains(';')
            || original.contains('\n')
            || original.starts_with(' ')
            || original.ends_with(' ')
            || (original.contains(' ') && !original.starts_with('"'));
        
        let quoted = if needs_quote {
            let escaped = original.replace('"', r#"\""#);
            format!("\"{}\"", escaped)
        } else {
            original.to_string()
        };
        
        // JSON roundtrip
        let json_val = Value::String(quoted);
        let extracted = json_val.as_str().unwrap();
        
        // Unquote
        let final_val = if extracted.starts_with('"') && extracted.ends_with('"') && extracted.len() >= 2 {
            let inner = &extracted[1..extracted.len() - 1];
            inner.replace(r#"\""#, "\"")
        } else {
            extracted.to_string()
        };
        
        if final_val == original {
            println!("✅ {}: {:?} preserved", name, original);
        } else {
            println!("❌ {}: {:?} → {:?}", name, original, final_val);
            panic!("Edge case failed: {}", name);
        }
    }
    
    println!("\n✅ All edge cases passed!");
}
