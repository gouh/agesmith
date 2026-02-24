use serde_json::{Map, Value};
use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn get_age_key() -> Option<String> {
    std::env::var("SOPS_AGE_KEY").ok()
}

fn get_age_key_file() -> String {
    std::env::var("SOPS_AGE_KEY_FILE").unwrap_or_else(|_| {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        format!("{}/.config/sops/age/keys.txt", home)
    })
}

fn decrypt_file(file_path: &PathBuf) -> Result<String, Box<dyn std::error::Error>> {
    let mut cmd = Command::new("sops");
    cmd.arg("--decrypt");

    if let Some(key) = get_age_key() {
        cmd.env("SOPS_AGE_KEY", key);
    } else {
        cmd.env("SOPS_AGE_KEY_FILE", get_age_key_file());
    }

    cmd.arg(file_path);

    let output = cmd.output()?;
    if !output.status.success() {
        return Err(format!("Error: {}", String::from_utf8_lossy(&output.stderr)).into());
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn flatten_json(prefix: &str, value: &Value, result: &mut Vec<(String, String)>) {
    match value {
        Value::Object(map) => {
            for (k, v) in map {
                let new_prefix = if prefix.is_empty() {
                    k.clone()
                } else {
                    format!("{}.{}", prefix, k)
                };
                flatten_json(&new_prefix, v, result);
            }
        }
        Value::Array(arr) => {
            for (i, v) in arr.iter().enumerate() {
                let new_prefix = format!("{}[{}]", prefix, i);
                flatten_json(&new_prefix, v, result);
            }
        }
        _ => {
            let str_value = value.to_string().trim_matches('"').to_string();
            let unescaped = str_value
                .replace("\\\"", "\"")
                .replace("\\'", "'")
                .replace("\\\\", "\\");
            result.push((prefix.to_string(), unescaped));
        }
    }
}

#[test]
fn test_read_real_sops_file() {
    let test_files = vec![".env", "secrets.yaml", "config.json", "app.ini"];

    for test_file in test_files {
        let test_dir = std::env::current_dir().unwrap_or_default();
        let file_path = test_dir.join(test_file);

        if file_path.exists() {
            println!("\n=== Testing: {} ===", test_file);

            match decrypt_file(&file_path) {
                Ok(decrypted) => {
                    println!("Decrypted content:\n{}", decrypted);

                    match serde_json::from_str::<Value>(&decrypted) {
                        Ok(json) => {
                            let mut secrets = Vec::new();
                            flatten_json("", &json, &mut secrets);
                            secrets.retain(|(k, _)| !k.starts_with("sops."));

                            for (key, value) in &secrets {
                                println!("{} = {}", key, value);
                                if value.contains('$')
                                    || value.contains('\\')
                                    || value.contains("\"")
                                {
                                    println!("  -> Contains special chars!");
                                }
                            }
                        }
                        Err(e) => {
                            println!("Not JSON format: {}", e);
                            for line in decrypted.lines() {
                                println!("{}", line);
                            }
                        }
                    }
                }
                Err(e) => {
                    println!("Failed to decrypt {}: {}", test_file, e);
                }
            }
        } else {
            println!("File {} not found, skipping", test_file);
        }
    }
}
