/// Test para detectar el problema específico con # en valores
#[test]
fn test_hash_in_env_values() {
    println!("\n=== Testing # character handling ===\n");
    
    let test_cases = vec![
        "PASSWORD=simple",
        "PASSWORD=pass#word",
        "PASSWORD=pass#word#123",
        "PASSWORD=#startswithhash",
        "PASSWORD=endswith#",
        "PASSWORD=multiple#hash#symbols#here",
    ];
    
    for line in test_cases {
        println!("Testing line: {:?}", line);
        
        // Simular el código actual
        let trimmed = line.trim();
        
        // El código actual verifica: line.starts_with('#')
        // Esto solo ignora líneas que EMPIEZAN con #, no valores que CONTIENEN #
        if trimmed.starts_with('#') {
            println!("  ⚠️  Line ignored (starts with #)");
            continue;
        }
        
        if let Some((key, value)) = trimmed.split_once('=') {
            let key = key.trim();
            let value = value.trim();
            
            println!("  Key: {:?}", key);
            println!("  Value: {:?}", value);
            
            // Verificar si el valor contiene #
            if value.contains('#') {
                println!("  ⚠️  Value contains # - potential issue with .env parsers");
            }
        }
        println!();
    }
}

#[test]
fn test_env_comment_handling() {
    println!("\n=== Testing comment handling in ENV files ===\n");
    
    // Algunos parsers de .env tratan # como inicio de comentario
    // incluso dentro de valores no entrecomillados
    let test_cases = vec![
        ("PASSWORD=value", "value", "No comment"),
        ("PASSWORD=value # comment", "value # comment", "Hash in value"),
        ("PASSWORD=value#nocomment", "value#nocomment", "Hash without space"),
        ("# PASSWORD=value", "", "Commented line"),
        ("PASSWORD=\"value # with hash\"", "\"value # with hash\"", "Quoted value"),
    ];
    
    for (line, expected_value, description) in test_cases {
        println!("{}: {:?}", description, line);
        
        let trimmed = line.trim();
        
        // Código actual
        if trimmed.starts_with('#') {
            println!("  → Line ignored (comment)");
            continue;
        }
        
        if let Some((_, value)) = trimmed.split_once('=') {
            let value = value.trim();
            println!("  → Parsed value: {:?}", value);
            
            if value != expected_value {
                println!("  ❌ MISMATCH! Expected: {:?}", expected_value);
            } else {
                println!("  ✅ Correct");
            }
        }
        println!();
    }
}

#[test]
fn test_sops_dotenv_format() {
    println!("\n=== Understanding SOPS dotenv format ===\n");
    
    // SOPS puede manejar .env de forma especial
    // Necesitamos entender cómo SOPS interpreta los valores
    
    let test_content = r#"# Comment line
SIMPLE=value
WITH_HASH=pass#word
WITH_EQUALS=a=b=c
WITH_SPACE=value with spaces
WITH_QUOTE=value"with"quotes
EMPTY=
"#;
    
    println!("Test content:");
    println!("{}", test_content);
    println!("\nParsing:");
    
    for line in test_content.lines() {
        let line = line.trim();
        
        if line.is_empty() {
            println!("  (empty line)");
            continue;
        }
        
        if line.starts_with('#') {
            println!("  (comment): {}", line);
            continue;
        }
        
        if let Some((key, value)) = line.split_once('=') {
            println!("  {} = {:?}", key.trim(), value.trim());
        }
    }
}

#[test]
fn test_ini_comment_handling() {
    println!("\n=== Testing comment handling in INI files ===\n");
    
    // En INI, tanto # como ; pueden ser comentarios
    let test_content = r#"[DEFAULT]
simple=value
with_hash=pass#word
with_semicolon=pass;word
# commented_key=value
; also_commented=value
"#;
    
    println!("Test content:");
    println!("{}", test_content);
    println!("\nParsing:");
    
    for line in test_content.lines() {
        let line = line.trim();
        
        if line.is_empty() {
            continue;
        }
        
        if line.starts_with('[') {
            println!("  [Section]: {}", line);
            continue;
        }
        
        // Código actual solo verifica starts_with('#')
        if line.starts_with('#') {
            println!("  (# comment): {}", line);
            continue;
        }
        
        // ⚠️ El código NO verifica ';' para comentarios en INI
        if line.starts_with(';') {
            println!("  ⚠️  (; comment) - NOT HANDLED: {}", line);
            continue;
        }
        
        if let Some((key, value)) = line.split_once('=') {
            let value = value.trim();
            println!("  {} = {:?}", key.trim(), value);
            
            if value.contains(';') {
                println!("    ⚠️  Value contains ; - may be truncated by INI parsers");
            }
            if value.contains('#') {
                println!("    ⚠️  Value contains # - may be truncated by some parsers");
            }
        }
    }
}
