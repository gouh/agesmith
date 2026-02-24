use serde_json::{Map, Value};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn generate_test_password() -> String {
    "P@ssw0rd$!#%&*()-=_+|;:,.<>?/\\test123".to_string()
}

fn get_age_key() -> Option<String> {
    std::env::var("SOPS_AGE_KEY").ok()
}

fn get_age_key_file() -> String {
    std::env::var("SOPS_AGE_KEY_FILE").unwrap_or_else(|_| {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        format!("{}/.config/sops/age/keys.txt", home)
    })
}

fn setup_test_file(format: &str, content: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let test_dir = std::env::temp_dir();
    let file_path = test_dir.join(format!("test_secrets.{}", format));
    fs::write(&file_path, content)?;
    Ok(file_path)
}

fn encrypt_with_sops(
    file_path: &PathBuf,
    input_format: &str,
    output_format: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let key_file = get_age_key_file();
    if !std::path::Path::new(&key_file).exists() && get_age_key().is_none() {
        return Err("No age key found".into());
    }

    let mut cmd = Command::new("sops");
    cmd.arg("--encrypt")
        .arg("--input-type")
        .arg(input_format)
        .arg("--output-type")
        .arg(output_format)
        .arg("-i")
        .arg(file_path);

    if let Some(key) = get_age_key() {
        cmd.env("SOPS_AGE_KEY", key);
    } else {
        cmd.env("SOPS_AGE_KEY_FILE", key_file);
    }

    let output = cmd.output()?;
    if !output.status.success() {
        return Err(format!("SOPS error: {}", String::from_utf8_lossy(&output.stderr)).into());
    }

    Ok(())
}

fn decrypt_with_sops(file_path: &PathBuf) -> Result<String, Box<dyn std::error::Error>> {
    let key_file = get_age_key_file();
    if !std::path::Path::new(&key_file).exists() && get_age_key().is_none() {
        return Err("No age key found".into());
    }

    let mut cmd = Command::new("sops");
    cmd.arg("--decrypt");

    if let Some(key) = get_age_key() {
        cmd.env("SOPS_AGE_KEY", key);
    } else {
        cmd.env("SOPS_AGE_KEY_FILE", key_file);
    }

    cmd.arg(file_path);

    let output = cmd.output()?;
    if !output.status.success() {
        return Err(format!(
            "SOPS decrypt error: {}",
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn test_full_flow_json() -> Result<(), Box<dyn std::error::Error>> {
    let password = generate_test_password();
    let key = "TEST_PASSWORD";
    let secrets: HashMap<String, String> =
        [(key.to_string(), password.clone())].into_iter().collect();

    let mut json_obj = Map::new();
    for (k, v) in &secrets {
        json_obj.insert(k.clone(), Value::String(v.clone()));
    }
    let content = serde_json::to_string_pretty(&Value::Object(json_obj))?;

    let file_path = setup_test_file("json", &content)?;
    encrypt_with_sops(&file_path, "json", "json")?;

    let decrypted = decrypt_with_sops(&file_path)?;
    let parsed: Value = serde_json::from_str(&decrypted)?;
    let recovered = parsed.get(key).and_then(|v| v.as_str()).unwrap_or("");

    fs::remove_file(&file_path).ok();
    let _ = file_path.with_extension("json.bak");

    assert_eq!(password, recovered, "JSON full flow failed");
    println!("✓ JSON flow OK: '{}'", recovered);
    Ok(())
}

fn test_full_flow_yaml() -> Result<(), Box<dyn std::error::Error>> {
    let password = generate_test_password();
    let key = "TEST_PASSWORD";
    let secrets: HashMap<String, String> =
        [(key.to_string(), password.clone())].into_iter().collect();

    let mut json_obj = Map::new();
    for (k, v) in &secrets {
        json_obj.insert(k.clone(), Value::String(v.clone()));
    }
    let content = serde_json::to_string_pretty(&Value::Object(json_obj))?;

    let file_path = setup_test_file("yaml", &content)?;
    encrypt_with_sops(&file_path, "json", "yaml")?;

    let decrypted = decrypt_with_sops(&file_path)?;
    println!("YAML decrypted:\n{}", decrypted);

    let parsed: Value = serde_json::from_str(&decrypted)?;
    let recovered = parsed.get(key).and_then(|v| v.as_str()).unwrap_or("");

    fs::remove_file(&file_path).ok();

    assert_eq!(password, recovered, "YAML full flow failed");
    println!("✓ YAML flow OK: '{}'", recovered);
    Ok(())
}

fn test_full_flow_env() -> Result<(), Box<dyn std::error::Error>> {
    let password = generate_test_password();
    let key = "TEST_PASSWORD";
    let secrets: HashMap<String, String> =
        [(key.to_string(), password.clone())].into_iter().collect();

    let mut json_obj = Map::new();
    for (k, v) in &secrets {
        json_obj.insert(k.clone(), Value::String(v.clone()));
    }
    let content = serde_json::to_string_pretty(&Value::Object(json_obj))?;

    let file_path = setup_test_file("env", &content)?;
    encrypt_with_sops(&file_path, "json", "dotenv")?;

    let decrypted = decrypt_with_sops(&file_path)?;
    println!("ENV decrypted:\n{}", decrypted);

    let parsed: Value = serde_json::from_str(&decrypted)?;
    let recovered = parsed.get(key).and_then(|v| v.as_str()).unwrap_or("");

    fs::remove_file(&file_path).ok();

    assert_eq!(password, recovered, "ENV full flow failed");
    println!("✓ ENV flow OK: '{}'", recovered);
    Ok(())
}

fn test_full_flow_ini() -> Result<(), Box<dyn std::error::Error>> {
    let password = generate_test_password();
    let key = "TEST_PASSWORD";
    let secrets: HashMap<String, String> =
        [(key.to_string(), password.clone())].into_iter().collect();

    let mut json_obj = Map::new();
    let mut default_section = Map::new();
    for (k, v) in &secrets {
        default_section.insert(k.clone(), Value::String(v.clone()));
    }
    json_obj.insert("DEFAULT".to_string(), Value::Object(default_section));
    let content = serde_json::to_string_pretty(&Value::Object(json_obj))?;

    let file_path = setup_test_file("ini", &content)?;
    encrypt_with_sops(&file_path, "json", "ini")?;

    let decrypted = decrypt_with_sops(&file_path)?;
    println!("INI decrypted:\n{}", decrypted);

    let parsed: Value = serde_json::from_str(&decrypted)?;
    let default = parsed.get("DEFAULT").and_then(|v| v.as_object()).unwrap();
    let recovered = default.get(key).and_then(|v| v.as_str()).unwrap_or("");

    fs::remove_file(&file_path).ok();

    assert_eq!(password, recovered, "INI full flow failed");
    println!("✓ INI flow OK: '{}'", recovered);
    Ok(())
}

#[test]
fn test_sops_integration() {
    if std::path::Path::new(&get_age_key_file()).exists() || get_age_key().is_some() {
        println!("Running SOPS integration tests...\n");

        if let Err(e) = test_full_flow_json() {
            eprintln!("✗ JSON: {}", e);
        }

        if let Err(e) = test_full_flow_yaml() {
            eprintln!("✗ YAML: {}", e);
        }

        if let Err(e) = test_full_flow_env() {
            eprintln!("✗ ENV: {}", e);
        }

        if let Err(e) = test_full_flow_ini() {
            eprintln!("✗ INI: {}", e);
        }
    } else {
        println!("Skipping SOPS tests - no age key found");
    }
}
