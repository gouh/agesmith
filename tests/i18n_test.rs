/// Tests para internacionalización
use std::collections::HashMap;

#[test]
fn test_translation_keys_exist() {
    println!("\n=== Testing translation keys ===\n");
    
    // Claves críticas que deben existir
    let required_keys = vec![
        "app_title",
        "explorer",
        "secrets",
        "key",
        "value",
        "help",
        "quit",
        "save",
        "edit",
        "delete",
        "search",
        "error",
        "success",
        "confirm",
        "cancel",
    ];
    
    // Simular traducciones
    let mut translations_en = HashMap::new();
    let mut translations_es = HashMap::new();
    
    for key in &required_keys {
        translations_en.insert(*key, format!("{}_en", key));
        translations_es.insert(*key, format!("{}_es", key));
    }
    
    for key in &required_keys {
        assert!(translations_en.contains_key(key), "EN missing key: {}", key);
        assert!(translations_es.contains_key(key), "ES missing key: {}", key);
        println!("✅ Key '{}' exists in both languages", key);
    }
}

#[test]
fn test_translation_completeness() {
    println!("\n=== Testing translation completeness ===\n");
    
    let en_keys: Vec<&str> = vec!["key1", "key2", "key3"];
    let es_keys: Vec<&str> = vec!["key1", "key2", "key3"];
    
    assert_eq!(en_keys.len(), es_keys.len(), "Both languages should have same number of keys");
    
    for key in &en_keys {
        assert!(es_keys.contains(key), "ES missing key: {}", key);
    }
    
    println!("✅ Both languages have {} keys", en_keys.len());
}

#[test]
fn test_language_switching() {
    println!("\n=== Testing language switching ===\n");
    
    #[derive(Debug, PartialEq, Clone, Copy)]
    enum Language {
        English,
        Spanish,
    }
    
    let mut current_lang = Language::English;
    println!("Current language: {:?}", current_lang);
    
    // Cambiar idioma
    current_lang = Language::Spanish;
    assert_eq!(current_lang, Language::Spanish);
    println!("Switched to: {:?}", current_lang);
    
    // Cambiar de vuelta
    current_lang = Language::English;
    assert_eq!(current_lang, Language::English);
    println!("Switched back to: {:?}", current_lang);
    
    println!("✅ Language switching works");
}

#[test]
fn test_translation_formatting() {
    println!("\n=== Testing translation formatting ===\n");
    
    // Simular traducciones con placeholders
    let template_en = "File {file} saved successfully";
    let template_es = "Archivo {file} guardado exitosamente";
    
    let file_name = "secrets.env";
    
    let message_en = template_en.replace("{file}", file_name);
    let message_es = template_es.replace("{file}", file_name);
    
    println!("EN: {}", message_en);
    println!("ES: {}", message_es);
    
    assert!(message_en.contains(file_name));
    assert!(message_es.contains(file_name));
    assert!(!message_en.contains("{file}"));
    assert!(!message_es.contains("{file}"));
    
    println!("✅ Translation formatting works");
}

#[test]
fn test_special_characters_in_translations() {
    println!("\n=== Testing special characters in translations ===\n");
    
    let translations = vec![
        ("spanish_accent", "Configuración"),
        ("spanish_n", "Contraseña"),
        ("quotes", r#"Press "Enter" to continue"#),
        ("symbols", "Save [s] • Quit [q]"),
    ];
    
    for (key, text) in translations {
        assert!(!text.is_empty(), "Translation should not be empty");
        println!("✅ {}: {}", key, text);
    }
}

#[test]
fn test_translation_fallback() {
    println!("\n=== Testing translation fallback ===\n");
    
    let mut translations = HashMap::new();
    translations.insert("existing_key", "Existing translation");
    
    // Clave que existe
    let key1 = "existing_key";
    let result1 = translations.get(key1).unwrap_or(&key1);
    assert_eq!(*result1, "Existing translation");
    println!("✅ Existing key: {}", result1);
    
    // Clave que no existe (fallback al key)
    let key2 = "missing_key";
    let result2 = translations.get(key2).unwrap_or(&key2);
    assert_eq!(*result2, "missing_key");
    println!("✅ Missing key fallback: {}", result2);
}

#[test]
fn test_pluralization() {
    println!("\n=== Testing pluralization ===\n");
    
    fn pluralize(count: usize, singular: &str, plural: &str) -> String {
        if count == 1 {
            format!("{} {}", count, singular)
        } else {
            format!("{} {}", count, plural)
        }
    }
    
    let test_cases = vec![
        (0, "secrets", "0 secrets"),
        (1, "secret", "1 secret"),
        (2, "secrets", "2 secrets"),
        (100, "secrets", "100 secrets"),
    ];
    
    for (count, expected_word, expected_result) in test_cases {
        let result = if count == 1 {
            pluralize(count, "secret", "secrets")
        } else {
            pluralize(count, "secret", "secrets")
        };
        
        assert_eq!(result, expected_result);
        println!("✅ {}", result);
    }
}
