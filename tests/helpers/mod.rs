/// Test helpers and utilities
use serde_json::Value;
use std::collections::HashMap;

/// Helper para crear secretos de prueba
pub fn create_test_secrets() -> HashMap<String, String> {
    let mut secrets = HashMap::new();
    secrets.insert("SIMPLE".to_string(), "password123".to_string());
    secrets.insert("WITH_HASH".to_string(), "pass#word".to_string());
    secrets.insert("WITH_SEMICOLON".to_string(), "pass;word".to_string());
    secrets.insert("WITH_SPACE".to_string(), "pass word".to_string());
    secrets.insert("COMPLEX".to_string(), "P@$$w0rd!#%&*()".to_string());
    secrets
}

/// Helper para verificar si un valor necesita entrecomillado
pub fn needs_quoting(value: &str) -> bool {
    value.contains('#')
        || value.contains(';')
        || value.contains('\n')
        || value.starts_with(' ')
        || value.ends_with(' ')
        || (value.contains(' ') && !value.starts_with('"'))
}

/// Helper para entrecomillar valores
pub fn quote_value(value: &str) -> String {
    if needs_quoting(value) {
        let escaped = value
            .replace('\\', r"\\")
            .replace('"', r#"\""#)
            .replace('\n', r"\n");
        format!("\"{}\"", escaped)
    } else {
        value.to_string()
    }
}

/// Helper para desescapar valores
pub fn unquote_value(value: &str) -> String {
    let trimmed = value.trim();
    if (trimmed.starts_with('"') && trimmed.ends_with('"') && trimmed.len() >= 2)
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
    }
}

/// Helper para crear JSON de prueba
pub fn create_test_json(secrets: &HashMap<String, String>) -> Value {
    let mut map = serde_json::Map::new();
    for (k, v) in secrets {
        map.insert(k.clone(), Value::String(v.clone()));
    }
    Value::Object(map)
}

/// Helper para verificar roundtrip
pub fn verify_roundtrip(original: &str) -> bool {
    let quoted = quote_value(original);
    let json_val = Value::String(quoted);
    let extracted = json_val.as_str().unwrap();
    let unquoted = unquote_value(extracted);
    unquoted == original
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_helpers() {
        let secrets = create_test_secrets();
        assert_eq!(secrets.len(), 5);

        assert!(needs_quoting("pass#word"));
        assert!(!needs_quoting("simple"));

        let quoted = quote_value("pass#word");
        assert!(quoted.starts_with('"'));
        assert!(quoted.ends_with('"'));

        assert!(verify_roundtrip("pass#word"));
        assert!(verify_roundtrip("simple"));
    }
}
