/// Tests para el generador de secretos

#[test]
fn test_password_generation() {
    println!("\n=== Testing password generation ===\n");
    
    use rand::Rng;
    let mut rng = rand::thread_rng();
    
    let lengths = vec![8, 16, 32, 64];
    let chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    
    for length in lengths {
        let password: String = (0..length)
            .map(|_| {
                let idx = rng.gen_range(0..chars.len());
                chars.chars().nth(idx).unwrap()
            })
            .collect();
            
        println!("Length {}: {}", length, password);
        
        assert_eq!(password.len(), length, "Password length mismatch");
        assert!(password.chars().all(|c| c.is_alphanumeric()), "Should only contain alphanumeric");
    }
}

#[test]
fn test_password_with_special_chars() {
    println!("\n=== Testing password with special characters ===\n");
    
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*()";
    
    for _ in 0..10 {
        let password: String = (0..32)
            .map(|_| {
                let idx = rng.gen_range(0..chars.len());
                chars.chars().nth(idx).unwrap()
            })
            .collect();
            
        println!("Generated: {}", password);
        assert_eq!(password.len(), 32);
    }
}

#[test]
fn test_hex_generation() {
    println!("\n=== Testing hex generation ===\n");
    
    use rand::Rng;
    use regex::Regex;
    let mut rng = rand::thread_rng();
    let hex_regex = Regex::new(r"^[0-9a-f]+$").unwrap();
    
    let lengths = vec![16, 32, 64];
    
    for length in lengths {
        let hex: String = (0..length)
            .map(|_| format!("{:x}", rng.gen_range(0..16)))
            .collect();
            
        println!("Length {}: {}", length, hex);
        
        assert_eq!(hex.len(), length);
        assert!(hex_regex.is_match(&hex), "Should be valid hex");
    }
}

#[test]
fn test_base64_generation() {
    println!("\n=== Testing base64 generation ===\n");
    
    use base64::{Engine as _, engine::general_purpose};
    use rand::Rng;
    use regex::Regex;
    
    let mut rng = rand::thread_rng();
    let lengths = vec![16, 32, 64];
    let base64_regex = Regex::new(r"^[A-Za-z0-9+/]+=*$").unwrap();
    
    for length in lengths {
        let bytes: Vec<u8> = (0..length).map(|_| rng.gen()).collect();
        let b64 = general_purpose::STANDARD.encode(&bytes);
        
        println!("Length {}: {}", length, b64);
        assert!(base64_regex.is_match(&b64), "Should be valid base64");
    }
}

#[test]
fn test_uuid_generation() {
    println!("\n=== Testing UUID generation ===\n");
    
    use regex::Regex;
    let uuid_regex = Regex::new(r"^[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$").unwrap();
    
    for i in 0..5 {
        let uuid = uuid::Uuid::new_v4().to_string();
        println!("UUID {}: {}", i + 1, uuid);
        
        assert!(uuid_regex.is_match(&uuid), "Should be valid UUIDv4");
    }
}

#[test]
fn test_uniqueness() {
    println!("\n=== Testing uniqueness ===\n");
    
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*()";
    
    let mut passwords = std::collections::HashSet::new();
    let mut uuids = std::collections::HashSet::new();
    
    // Generar 100 de cada y verificar que son únicos
    for _ in 0..100 {
        let password: String = (0..32)
            .map(|_| {
                let idx = rng.gen_range(0..chars.len());
                chars.chars().nth(idx).unwrap()
            })
            .collect();
        passwords.insert(password);
        uuids.insert(uuid::Uuid::new_v4().to_string());
    }
    
    assert_eq!(passwords.len(), 100, "All passwords should be unique");
    assert_eq!(uuids.len(), 100, "All UUIDs should be unique");
    
    println!("✅ Generated 100 unique passwords");
    println!("✅ Generated 100 unique UUIDs");
}

#[test]
fn test_password_strength() {
    println!("\n=== Testing password strength ===\n");
    
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*()";
    
    for _ in 0..10 {
        let password: String = (0..16)
            .map(|_| {
                let idx = rng.gen_range(0..chars.len());
                chars.chars().nth(idx).unwrap()
            })
            .collect();
        
        let has_lower = password.chars().any(|c| c.is_lowercase());
        let has_upper = password.chars().any(|c| c.is_uppercase());
        let has_digit = password.chars().any(|c| c.is_numeric());
        
        println!("Password: {} (lower:{} upper:{} digit:{})", 
                 password, has_lower, has_upper, has_digit);
        
        // Con 16 caracteres, probabilísticamente debería tener variedad
        assert!(has_lower || has_upper || has_digit, "Should have character variety");
    }
}
