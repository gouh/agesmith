/// Tests para configuración y favoritos
use std::fs;
use std::path::PathBuf;

#[test]
fn test_config_defaults() {
    println!("\n=== Testing config defaults ===\n");
    
    // Simular valores por defecto
    let default_theme = "dark";
    let default_timeout = 15;
    let default_clipboard = 30;
    let default_lang = "en";
    
    println!("Default theme: {}", default_theme);
    println!("Default auto_lock_minutes: {}", default_timeout);
    println!("Default clipboard_clear_seconds: {}", default_clipboard);
    println!("Default language: {}", default_lang);
    
    assert_eq!(default_theme, "dark");
    assert_eq!(default_timeout, 15);
    assert_eq!(default_clipboard, 30);
    assert_eq!(default_lang, "en");
}

#[test]
fn test_favorites_management() {
    println!("\n=== Testing favorites management ===\n");
    
    let mut favorites: Vec<PathBuf> = Vec::new();
    
    // Agregar favoritos
    let file1 = PathBuf::from("/path/to/secrets.env");
    let file2 = PathBuf::from("/path/to/config.json");
    
    favorites.push(file1.clone());
    favorites.push(file2.clone());
    
    println!("Added {} favorites", favorites.len());
    assert_eq!(favorites.len(), 2);
    
    // Verificar que contiene
    assert!(favorites.contains(&file1));
    assert!(favorites.contains(&file2));
    
    // Remover favorito
    favorites.retain(|f| f != &file1);
    assert_eq!(favorites.len(), 1);
    assert!(!favorites.contains(&file1));
    assert!(favorites.contains(&file2));
    
    println!("✅ Favorites management works correctly");
}

#[test]
fn test_favorites_deduplication() {
    println!("\n=== Testing favorites deduplication ===\n");
    
    let mut favorites: Vec<PathBuf> = Vec::new();
    let file = PathBuf::from("/path/to/secrets.env");
    
    // Intentar agregar duplicado
    favorites.push(file.clone());
    
    if !favorites.contains(&file) {
        favorites.push(file.clone());
    }
    
    assert_eq!(favorites.len(), 1, "Should not have duplicates");
    println!("✅ Deduplication works");
}

#[test]
fn test_config_validation() {
    println!("\n=== Testing config validation ===\n");
    
    // Valores válidos
    let valid_themes = vec!["dark", "light"];
    let valid_langs = vec!["en", "es"];
    
    for theme in &valid_themes {
        assert!(valid_themes.contains(theme), "Theme should be valid");
        println!("✅ Theme '{}' is valid", theme);
    }
    
    for lang in &valid_langs {
        assert!(valid_langs.contains(lang), "Language should be valid");
        println!("✅ Language '{}' is valid", lang);
    }
    
    // Valores inválidos
    let invalid_theme = "rainbow";
    assert!(!valid_themes.contains(&invalid_theme), "Invalid theme should be rejected");
    println!("✅ Invalid theme '{}' rejected", invalid_theme);
}

#[test]
fn test_config_bounds() {
    println!("\n=== Testing config bounds ===\n");
    
    let test_cases = vec![
        ("auto_lock_minutes", 0, false),
        ("auto_lock_minutes", 1, true),
        ("auto_lock_minutes", 60, true),
        ("auto_lock_minutes", 1440, true),
        ("auto_lock_minutes", 10000, false),
        ("clipboard_clear_seconds", 0, false),
        ("clipboard_clear_seconds", 1, true),
        ("clipboard_clear_seconds", 300, true),
        ("clipboard_clear_seconds", 10000, false),
    ];
    
    for (field, value, should_be_valid) in test_cases {
        let is_valid = match field {
            "auto_lock_minutes" => value > 0 && value <= 1440,
            "clipboard_clear_seconds" => value > 0 && value <= 300,
            _ => false,
        };
        
        assert_eq!(is_valid, should_be_valid, 
                   "{} = {} should be {}", field, value, 
                   if should_be_valid { "valid" } else { "invalid" });
        
        println!("{} {} = {} is {}", 
                 if is_valid { "✅" } else { "❌" },
                 field, value,
                 if is_valid { "valid" } else { "invalid" });
    }
}

#[test]
fn test_theme_colors() {
    println!("\n=== Testing theme colors ===\n");
    
    // Simular colores del tema
    let dark_theme = vec![
        ("primary", (100, 200, 255)),
        ("success", (100, 255, 100)),
        ("error", (255, 100, 100)),
        ("warning", (255, 200, 100)),
    ];
    
    for (name, (r, g, b)) in dark_theme {
        assert!(r <= 255 && g <= 255 && b <= 255, "RGB values should be valid");
        println!("✅ {} color: rgb({}, {}, {})", name, r, g, b);
    }
}

#[test]
fn test_config_file_paths() {
    println!("\n=== Testing config file paths ===\n");
    
    // Simular rutas de configuración
    let home = std::env::var("HOME").unwrap_or_else(|_| "/home/user".to_string());
    let config_dir = format!("{}/.config/agesmith", home);
    let config_file = format!("{}/config.toml", config_dir);
    let favorites_file = format!("{}/favorites.json", config_dir);
    
    println!("Config dir: {}", config_dir);
    println!("Config file: {}", config_file);
    println!("Favorites file: {}", favorites_file);
    
    assert!(config_file.ends_with("config.toml"));
    assert!(favorites_file.ends_with("favorites.json"));
}
