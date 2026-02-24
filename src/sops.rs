use anyhow::{Context, Result};
use serde_json::Value;
use std::{env, fs, path::PathBuf, process::Command};

#[derive(Debug, Clone)]
pub struct AgeKey {
    pub key: String,
    pub comment: Option<String>,
    pub public_key: Option<String>,
}

fn create_age_key(keys_path: &PathBuf) -> Result<()> {
    if let Some(parent) = keys_path.parent() {
        fs::create_dir_all(parent)?;
    }
    
    let output = Command::new("age-keygen")
        .arg("-o")
        .arg(keys_path)
        .output()
        .context("No se pudo ejecutar age-keygen. ¿Está instalado?")?;
    
    if !output.status.success() {
        anyhow::bail!("age-keygen falló: {}", String::from_utf8_lossy(&output.stderr));
    }
    
    Ok(())
}

pub fn create_age_key_file() -> Result<()> {
    let home = env::var("HOME").context("HOME no está definido")?;
    let keys_path = PathBuf::from(home).join(".config/sops/age/keys.txt");
    create_age_key(&keys_path)
}

pub fn load_age_keys() -> Result<Vec<AgeKey>> {
    let home = env::var("HOME").context("HOME no está definido")?;
    let keys_path = PathBuf::from(home).join(".config/sops/age/keys.txt");

    if !keys_path.exists() {
        return Ok(Vec::new());
    }

    let content = fs::read_to_string(&keys_path)
        .context(format!("No se pudo leer {}", keys_path.display()))?;

    let mut keys = Vec::new();
    let mut last_comment: Option<String> = None;

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('#') {
            last_comment = Some(trimmed.trim_start_matches('#').trim().to_string());
        } else if trimmed.starts_with("AGE-SECRET-KEY-") {
            let public_key = age_private_to_public(trimmed).ok();
            keys.push(AgeKey {
                key: trimmed.to_string(),
                comment: last_comment.take(),
                public_key,
            });
        } else if !trimmed.is_empty() {
            last_comment = None;
        }
    }

    Ok(keys)
}

pub fn get_sops_recipients(file_path: &PathBuf) -> Result<Vec<String>> {
    let content = fs::read_to_string(file_path)?;
    let json: Value = serde_json::from_str(&content)?;
    
    let mut recipients = Vec::new();
    if let Some(sops) = json.get("sops") {
        if let Some(age_array) = sops.get("age").and_then(|v| v.as_array()) {
            for entry in age_array {
                if let Some(recipient) = entry.get("recipient").and_then(|v| v.as_str()) {
                    recipients.push(recipient.to_string());
                }
            }
        }
    }
    Ok(recipients)
}

pub fn age_private_to_public(private_key: &str) -> Result<String> {
    let output = Command::new("age-keygen")
        .arg("-y")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .and_then(|mut child| {
            use std::io::Write;
            if let Some(mut stdin) = child.stdin.take() {
                stdin.write_all(private_key.as_bytes())?;
            }
            child.wait_with_output()
        })
        .context("Could not execute age-keygen")?;

    if output.status.success() {
        Ok(String::from_utf8(output.stdout)?.trim().to_string())
    } else {
        anyhow::bail!("Error converting private key to public")
    }
}

pub fn run_sops_command(file_path: &PathBuf, age_key: Option<&str>) -> Result<String> {
    let mut cmd = Command::new("sops");
    cmd.arg("-d");
    
    let ext = file_path.extension().and_then(|s| s.to_str());
    
    // Para todos los archivos, salida en JSON para parsear
    // SOPS detecta automáticamente el formato de entrada
    cmd.arg("--output-type").arg("json");
    
    cmd.arg(file_path);

    if let Some(key) = age_key {
        cmd.env("SOPS_AGE_KEY", key);
    } else {
        // Asegurar que SOPS encuentre las llaves age
        let age_key_file = std::env::var("SOPS_AGE_KEY_FILE")
            .unwrap_or_else(|_| {
                let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
                format!("{}/.config/sops/age/keys.txt", home)
            });
        cmd.env("SOPS_AGE_KEY_FILE", age_key_file);
    }

    let output = cmd.output().context("No se pudo ejecutar sops")?;

    if output.status.success() {
        Ok(String::from_utf8(output.stdout)?)
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("SOPS error: {}", stderr)
    }
}

pub fn flatten_json(prefix: &str, value: &Value, result: &mut Vec<(String, String)>) {
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
            // Desescapar comillas y barras invertidas
            let unescaped = str_value
                .replace("\\\"", "\"")
                .replace("\\'", "'")
                .replace("\\\\", "\\");
            result.push((prefix.to_string(), unescaped));
        }
    }
}

pub fn get_encrypted_keys(file_path: &PathBuf) -> Result<Vec<String>> {
    let content = fs::read_to_string(file_path)?;
    
    // Detectar formato del archivo
    let file_name = file_path.file_name().and_then(|s| s.to_str()).unwrap_or("");
    let ext = file_path.extension().and_then(|s| s.to_str());
    
    let is_dotenv = file_name == ".env" || ext == Some("env");
    let is_ini = file_name == ".ini" || ext == Some("ini");
    
    if is_dotenv || is_ini {
        // Para dotenv/ini, buscar líneas con ENC[
        let encrypted_keys: Vec<String> = content
            .lines()
            .filter_map(|line| {
                let line = line.trim();
                if line.is_empty() || line.starts_with('#') || line.starts_with('[') {
                    return None;
                }
                if let Some((key, value)) = line.split_once('=') {
                    let key = key.trim();
                    let value = value.trim();
                    
                    // Para INI, SOPS usa formato "SECTION.SUBSECTION.key"
                    // Extraer solo la última parte después del último punto
                    let clean_key = if is_ini && key.contains('.') {
                        key.split('.').last().unwrap_or(key)
                    } else {
                        key
                    };
                    
                    if value.contains("ENC[") {
                        return Some(clean_key.to_string());
                    }
                }
                None
            })
            .collect();
        return Ok(encrypted_keys);
    }
    
    // Para JSON/YAML, parsear como JSON
    let json: Value = serde_json::from_str(&content)?;

    let mut encrypted_keys = Vec::new();

    fn find_encrypted(prefix: &str, value: &Value, result: &mut Vec<String>) {
        match value {
            Value::Object(map) => {
                for (k, v) in map {
                    if k == "sops" {
                        continue;
                    }
                    let new_prefix = if prefix.is_empty() {
                        k.clone()
                    } else {
                        format!("{}.{}", prefix, k)
                    };

                    if v.is_string() && v.as_str().unwrap_or("").starts_with("ENC[") {
                        result.push(new_prefix.clone());
                    }
                    find_encrypted(&new_prefix, v, result);
                }
            }
            Value::Array(arr) => {
                for (i, v) in arr.iter().enumerate() {
                    let new_prefix = format!("{}[{}]", prefix, i);
                    if v.is_string() && v.as_str().unwrap_or("").starts_with("ENC[") {
                        result.push(new_prefix.clone());
                    }
                    find_encrypted(&new_prefix, v, result);
                }
            }
            _ => {}
        }
    }

    find_encrypted("", &json, &mut encrypted_keys);
    Ok(encrypted_keys)
}

pub fn decrypt_and_parse(file_path: &PathBuf, age_key: Option<&str>) -> Result<Vec<(String, String)>> {
    let decrypted = run_sops_command(file_path, age_key)?;
    
    // SOPS siempre devuelve JSON, parseamos
    let json: Value = serde_json::from_str(&decrypted)?;
    let mut secrets = Vec::new();
    flatten_json("", &json, &mut secrets);
    secrets.retain(|(k, _)| !k.starts_with("sops."));
    
    // Limpiar prefijos DEFAULT. de archivos INI
    let ext = file_path.extension().and_then(|s| s.to_str());
    let file_name = file_path.file_name().and_then(|s| s.to_str()).unwrap_or("");
    let is_ini = file_name == ".ini" || ext == Some("ini");
    
    if is_ini {
        secrets = secrets.into_iter()
            .map(|(k, v)| {
                let clean_key = k.strip_prefix("DEFAULT.").unwrap_or(&k).to_string();
                (clean_key, v)
            })
            .collect();
    }
    
    Ok(secrets)
}
