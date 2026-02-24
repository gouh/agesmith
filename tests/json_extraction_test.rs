use serde_json::{json, Value};

#[test]
fn test_json_value_extraction() {
    println!("\n=== Testing JSON value extraction methods ===\n");
    
    let test_cases = vec![
        ("simple", "password123"),
        ("with_quotes", r#""pass#word""#),
        ("with_escaped", r#"\"pass#word\""#),
    ];
    
    for (key, original) in test_cases {
        println!("Testing: {} = {:?}", key, original);
        
        // Crear JSON value
        let json_val = Value::String(original.to_string());
        
        // Método 1: to_string() - MALO, agrega comillas extras
        let method1 = json_val.to_string();
        println!("  to_string():           {:?}", method1);
        
        // Método 2: to_string().trim_matches('"') - MALO, puede fallar
        let method2 = json_val.to_string().trim_matches('"').to_string();
        println!("  trim_matches('\"'):    {:?}", method2);
        
        // Método 3: as_str() - BUENO, obtiene el valor directo
        let method3 = json_val.as_str().unwrap_or("");
        println!("  as_str():              {:?}", method3);
        
        if method3 == original {
            println!("  ✅ as_str() preserves original");
        } else {
            println!("  ❌ as_str() mismatch: {:?} != {:?}", method3, original);
        }
        println!();
    }
}

#[test]
fn test_complete_flow_with_special_chars() {
    println!("\n=== Testing complete flow with special characters ===\n");
    
    // Simular el flujo completo
    let original_password = "pass#word#123";
    println!("Original password: {:?}", original_password);
    
    // Step 1: Usuario guarda (se entrecomilla)
    let needs_quote = original_password.contains('#');
    let quoted = if needs_quote {
        let escaped = original_password.replace('"', r#"\""#);
        format!("\"{}\"", escaped)
    } else {
        original_password.to_string()
    };
    println!("Step 1 - Quoted for SOPS: {:?}", quoted);
    
    // Step 2: Se guarda en JSON
    let json_obj = json!({
        "PASSWORD": quoted
    });
    let json_str = serde_json::to_string(&json_obj).unwrap();
    println!("Step 2 - JSON: {}", json_str);
    
    // Step 3: SOPS desencripta (parsear JSON)
    let parsed: Value = serde_json::from_str(&json_str).unwrap();
    let json_value = parsed.get("PASSWORD").unwrap();
    
    // Step 4: Extraer valor (CORRECTO: usar as_str())
    let extracted = json_value.as_str().unwrap();
    println!("Step 3 - Extracted with as_str(): {:?}", extracted);
    
    // Step 5: Desescapar (remover comillas)
    let final_value = if extracted.starts_with('"') && extracted.ends_with('"') && extracted.len() >= 2 {
        let inner = &extracted[1..extracted.len() - 1];
        inner.replace(r#"\""#, "\"")
    } else {
        extracted.to_string()
    };
    println!("Step 4 - Unquoted: {:?}", final_value);
    
    if final_value == original_password {
        println!("\n✅ SUCCESS: Password preserved correctly!");
    } else {
        println!("\n❌ FAIL: Password corrupted!");
        println!("   Expected: {:?}", original_password);
        println!("   Got:      {:?}", final_value);
        panic!("Password not preserved");
    }
}

#[test]
fn test_various_special_chars() {
    println!("\n=== Testing various special characters ===\n");
    
    let passwords = vec![
        "simple",
        "pass#word",
        "pass;word",
        "pass word",
        r#"pass"word"#,
        "P@$$w0rd!#%",
    ];
    
    for original in passwords {
        println!("Testing: {:?}", original);
        
        // Entrecomillar si es necesario
        let needs_quote = original.contains('#') 
            || original.contains(';')
            || original.contains(' ')
            || original.starts_with(' ')
            || original.ends_with(' ');
        
        let quoted = if needs_quote {
            let escaped = original.replace('"', r#"\""#);
            format!("\"{}\"", escaped)
        } else {
            original.to_string()
        };
        
        // Guardar en JSON
        let json_val = Value::String(quoted.clone());
        
        // Extraer con as_str()
        let extracted = json_val.as_str().unwrap();
        
        // Desescapar
        let final_val = if extracted.starts_with('"') && extracted.ends_with('"') && extracted.len() >= 2 {
            let inner = &extracted[1..extracted.len() - 1];
            inner.replace(r#"\""#, "\"")
        } else {
            extracted.to_string()
        };
        
        if final_val == original {
            println!("  ✅ Preserved: {:?} → {:?} → {:?}", original, quoted, final_val);
        } else {
            println!("  ❌ Corrupted: {:?} → {:?} → {:?}", original, quoted, final_val);
        }
    }
}
