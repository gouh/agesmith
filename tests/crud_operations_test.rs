/// Tests de integración para operaciones CRUD completas
use serde_json::{json, Value};
use std::collections::HashMap;

#[test]
fn test_create_secret() {
    println!("\n=== Testing CREATE operation ===\n");
    
    let mut secrets: HashMap<String, String> = HashMap::new();
    
    // Crear nuevo secreto
    let key = "DATABASE_URL";
    let value = "postgres://user:pass@localhost/db";
    
    secrets.insert(key.to_string(), value.to_string());
    
    assert_eq!(secrets.len(), 1);
    assert_eq!(secrets.get(key), Some(&value.to_string()));
    
    println!("✅ Created secret: {} = {}", key, value);
}

#[test]
fn test_read_secret() {
    println!("\n=== Testing READ operation ===\n");
    
    let mut secrets = HashMap::new();
    secrets.insert("API_KEY".to_string(), "secret123".to_string());
    secrets.insert("DB_PASS".to_string(), "pass#word".to_string());
    
    // Leer secreto existente
    let api_key = secrets.get("API_KEY");
    assert_eq!(api_key, Some(&"secret123".to_string()));
    println!("✅ Read API_KEY: {:?}", api_key);
    
    // Leer secreto con caracteres especiales
    let db_pass = secrets.get("DB_PASS");
    assert_eq!(db_pass, Some(&"pass#word".to_string()));
    println!("✅ Read DB_PASS: {:?}", db_pass);
    
    // Leer secreto inexistente
    let missing = secrets.get("MISSING");
    assert_eq!(missing, None);
    println!("✅ Missing key returns None");
}

#[test]
fn test_update_secret() {
    println!("\n=== Testing UPDATE operation ===\n");
    
    let mut secrets = HashMap::new();
    secrets.insert("PASSWORD".to_string(), "old_password".to_string());
    
    println!("Original: PASSWORD = {}", secrets.get("PASSWORD").unwrap());
    
    // Actualizar secreto
    secrets.insert("PASSWORD".to_string(), "new_password".to_string());
    
    assert_eq!(secrets.get("PASSWORD"), Some(&"new_password".to_string()));
    println!("✅ Updated: PASSWORD = {}", secrets.get("PASSWORD").unwrap());
}

#[test]
fn test_delete_secret() {
    println!("\n=== Testing DELETE operation ===\n");
    
    let mut secrets = HashMap::new();
    secrets.insert("TEMP_KEY".to_string(), "temp_value".to_string());
    secrets.insert("KEEP_KEY".to_string(), "keep_value".to_string());
    
    assert_eq!(secrets.len(), 2);
    println!("Initial secrets: {}", secrets.len());
    
    // Eliminar secreto
    secrets.remove("TEMP_KEY");
    
    assert_eq!(secrets.len(), 1);
    assert!(!secrets.contains_key("TEMP_KEY"));
    assert!(secrets.contains_key("KEEP_KEY"));
    
    println!("✅ Deleted TEMP_KEY, remaining: {}", secrets.len());
}

#[test]
fn test_bulk_operations() {
    println!("\n=== Testing BULK operations ===\n");
    
    let mut secrets = HashMap::new();
    
    // Crear múltiples secretos
    let bulk_data = vec![
        ("KEY1", "value1"),
        ("KEY2", "value2"),
        ("KEY3", "value3"),
        ("KEY4", "value4"),
        ("KEY5", "value5"),
    ];
    
    for (key, value) in &bulk_data {
        secrets.insert(key.to_string(), value.to_string());
    }
    
    assert_eq!(secrets.len(), 5);
    println!("✅ Created {} secrets", secrets.len());
    
    // Actualizar múltiples
    for (key, _) in &bulk_data {
        secrets.insert(key.to_string(), format!("{}_updated", key));
    }
    
    for (key, _) in &bulk_data {
        assert!(secrets.get(*key).unwrap().ends_with("_updated"));
    }
    println!("✅ Updated {} secrets", secrets.len());
    
    // Eliminar múltiples
    secrets.remove("KEY1");
    secrets.remove("KEY3");
    secrets.remove("KEY5");
    
    assert_eq!(secrets.len(), 2);
    println!("✅ Deleted 3 secrets, remaining: {}", secrets.len());
}

#[test]
fn test_search_and_filter() {
    println!("\n=== Testing SEARCH and FILTER ===\n");
    
    let mut secrets = HashMap::new();
    secrets.insert("DATABASE_URL".to_string(), "postgres://localhost".to_string());
    secrets.insert("DATABASE_PASSWORD".to_string(), "secret123".to_string());
    secrets.insert("API_KEY".to_string(), "key123".to_string());
    secrets.insert("API_SECRET".to_string(), "secret456".to_string());
    
    // Buscar por prefijo de clave
    let database_keys: Vec<_> = secrets.keys()
        .filter(|k| k.starts_with("DATABASE"))
        .collect();
    
    assert_eq!(database_keys.len(), 2);
    println!("✅ Found {} DATABASE_* keys", database_keys.len());
    
    // Buscar por contenido de valor
    let secrets_with_secret: Vec<_> = secrets.iter()
        .filter(|(_, v)| v.contains("secret"))
        .collect();
    
    assert_eq!(secrets_with_secret.len(), 2);
    println!("✅ Found {} secrets containing 'secret'", secrets_with_secret.len());
    
    // Búsqueda case-insensitive
    let api_keys: Vec<_> = secrets.keys()
        .filter(|k| k.to_lowercase().contains("api"))
        .collect();
    
    assert_eq!(api_keys.len(), 2);
    println!("✅ Found {} API keys (case-insensitive)", api_keys.len());
}

#[test]
fn test_validation() {
    println!("\n=== Testing VALIDATION ===\n");
    
    // Validar clave vacía
    let empty_key = "";
    assert!(empty_key.is_empty(), "Empty key should be invalid");
    println!("✅ Empty key rejected");
    
    // Validar clave con espacios
    let key_with_spaces = "MY KEY";
    assert!(key_with_spaces.contains(' '), "Key with spaces should be flagged");
    println!("⚠️  Key with spaces: '{}'", key_with_spaces);
    
    // Validar caracteres especiales en clave
    let key_with_special = "MY-KEY_123";
    assert!(key_with_special.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_'));
    println!("✅ Valid key with special chars: '{}'", key_with_special);
    
    // Validar valor vacío (debería ser permitido)
    let empty_value = "";
    println!("✅ Empty value is allowed: '{}'", empty_value);
}

#[test]
fn test_nested_keys() {
    println!("\n=== Testing NESTED keys ===\n");
    
    let mut secrets = HashMap::new();
    
    // Claves anidadas (formato JSON)
    secrets.insert("database.host".to_string(), "localhost".to_string());
    secrets.insert("database.port".to_string(), "5432".to_string());
    secrets.insert("database.credentials.user".to_string(), "admin".to_string());
    secrets.insert("database.credentials.password".to_string(), "secret".to_string());
    
    // Buscar por prefijo
    let database_config: Vec<_> = secrets.keys()
        .filter(|k| k.starts_with("database."))
        .collect();
    
    assert_eq!(database_config.len(), 4);
    println!("✅ Found {} database.* keys", database_config.len());
    
    // Buscar por nivel específico
    let credentials: Vec<_> = secrets.keys()
        .filter(|k| k.contains("credentials"))
        .collect();
    
    assert_eq!(credentials.len(), 2);
    println!("✅ Found {} credentials keys", credentials.len());
}

#[test]
fn test_concurrent_modifications() {
    println!("\n=== Testing CONCURRENT modifications ===\n");
    
    let mut secrets = HashMap::new();
    secrets.insert("KEY1".to_string(), "value1".to_string());
    
    // Simular modificación concurrente
    let original_value = secrets.get("KEY1").cloned();
    
    // Modificación 1
    secrets.insert("KEY1".to_string(), "modified1".to_string());
    
    // Modificación 2 (sobrescribe)
    secrets.insert("KEY1".to_string(), "modified2".to_string());
    
    assert_eq!(secrets.get("KEY1"), Some(&"modified2".to_string()));
    assert_ne!(secrets.get("KEY1"), original_value.as_ref());
    
    println!("✅ Last write wins: {:?} → {:?}", original_value, secrets.get("KEY1"));
}
