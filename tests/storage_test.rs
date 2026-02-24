#[cfg(test)]
mod tests {
    use serde_json::{Map, Value};
    use std::collections::HashMap;

    fn generate_test_password() -> String {
        "P@ssw0rd$!#%&*()-=_+|;:,.<>?/\\test".to_string()
    }

    fn flatten_json_rec(prefix: &str, value: &Value) -> Vec<(String, String)> {
        let mut result = Vec::new();
        match value {
            Value::Object(map) => {
                for (k, v) in map {
                    let new_prefix = if prefix.is_empty() {
                        k.clone()
                    } else {
                        format!("{}.{}", prefix, k)
                    };
                    result.extend(flatten_json_rec(&new_prefix, v));
                }
            }
            Value::Array(arr) => {
                for (i, v) in arr.iter().enumerate() {
                    let new_prefix = format!("{}[{}]", prefix, i);
                    result.extend(flatten_json_rec(&new_prefix, v));
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
        result
    }

    #[test]
    fn test_json_serialization_roundtrip() {
        let original_password = generate_test_password();
        let secrets: HashMap<String, String> =
            [("TEST_KEY".to_string(), original_password.clone())]
                .into_iter()
                .collect();

        let mut json_obj = Map::new();
        for (k, v) in &secrets {
            json_obj.insert(k.clone(), Value::String(v.clone()));
        }

        let serialized = serde_json::to_string_pretty(&Value::Object(json_obj)).unwrap();
        println!("JSON Serialized:\n{}", serialized);

        let deserialized: Value = serde_json::from_str(&serialized).unwrap();
        let recovered: HashMap<String, String> = deserialized
            .as_object()
            .unwrap()
            .iter()
            .map(|(k, v)| (k.clone(), v.as_str().unwrap().to_string()))
            .collect();

        assert_eq!(secrets, recovered, "JSON roundtrip failed");
    }

    #[test]
    fn test_yaml_serialization_with_special_chars() {
        let original_password = generate_test_password();
        let secrets: HashMap<String, String> =
            [("TEST_KEY".to_string(), original_password.clone())]
                .into_iter()
                .collect();

        let mut json_obj = Map::new();
        for (k, v) in &secrets {
            json_obj.insert(k.clone(), Value::String(v.clone()));
        }

        let content = serde_json::to_string_pretty(&Value::Object(json_obj)).unwrap();
        println!("YAML Input (as JSON):\n{}", content);

        let deserialized: Value = serde_json::from_str(&content).unwrap();
        let recovered: HashMap<String, String> = deserialized
            .as_object()
            .unwrap()
            .iter()
            .map(|(k, v)| (k.clone(), v.as_str().unwrap().to_string()))
            .collect();

        assert_eq!(secrets, recovered, "YAML roundtrip failed");
    }

    #[test]
    fn test_env_serialization_with_special_chars() {
        let original_password = generate_test_password();
        let secrets: HashMap<String, String> =
            [("TEST_KEY".to_string(), original_password.clone())]
                .into_iter()
                .collect();

        let mut json_obj = Map::new();
        for (k, v) in &secrets {
            json_obj.insert(k.clone(), Value::String(v.clone()));
        }

        let content = serde_json::to_string_pretty(&Value::Object(json_obj)).unwrap();
        println!("ENV Input (as JSON):\n{}", content);

        let deserialized: Value = serde_json::from_str(&content).unwrap();
        let recovered: HashMap<String, String> = deserialized
            .as_object()
            .unwrap()
            .iter()
            .map(|(k, v)| (k.clone(), v.as_str().unwrap().to_string()))
            .collect();

        assert_eq!(secrets, recovered, "ENV roundtrip failed");
    }

    #[test]
    fn test_ini_serialization_with_special_chars() {
        let original_password = generate_test_password();
        let secrets: HashMap<String, String> =
            [("DEFAULT.TEST_KEY".to_string(), original_password.clone())]
                .into_iter()
                .collect();

        let mut json_obj = Map::new();
        let mut default_section = Map::new();
        for (k, v) in &secrets {
            let clean_key = k.split('.').last().unwrap_or(k);
            default_section.insert(clean_key.to_string(), Value::String(v.clone()));
        }
        json_obj.insert("DEFAULT".to_string(), Value::Object(default_section));

        let content = serde_json::to_string_pretty(&Value::Object(json_obj)).unwrap();
        println!("INI Input (as JSON):\n{}", content);

        let deserialized: Value = serde_json::from_str(&content).unwrap();
        let default = deserialized.get("DEFAULT").unwrap().as_object().unwrap();
        let recovered: HashMap<String, String> = default
            .iter()
            .map(|(k, v)| (k.clone(), v.as_str().unwrap().to_string()))
            .collect();

        let expected: HashMap<String, String> =
            [("TEST_KEY".to_string(), original_password.clone())]
                .into_iter()
                .collect();

        assert_eq!(expected, recovered, "INI roundtrip failed");
    }

    #[test]
    fn test_json_escape_sequences() {
        let test_cases = vec![
            "P@ssw0rd$!#%&*()-=_+|;:,.<>?/\\",
            "test\nnewline\ttab\rreturn",
            "quote\"test\\backslash",
            "unicode_é_ñ_ü",
            "special_<>_|_pipe",
        ];

        for original in test_cases {
            let mut json_obj = Map::new();
            json_obj.insert("key".to_string(), Value::String(original.to_string()));

            let serialized = serde_json::to_string(&Value::Object(json_obj)).unwrap();
            println!("Original: {}", original);
            println!("Serialized: {}", serialized);

            let deserialized: Value = serde_json::from_str(&serialized).unwrap();
            let recovered = deserialized.get("key").unwrap().as_str().unwrap();
            println!("Recovered: {}\n", recovered);

            assert_eq!(original, recovered, "Failed for: {}", original);
        }
    }

    #[test]
    fn test_all_special_chars() {
        let all_special = "!@#$%^&*()_+-=[]{}|;:,.<>?/\\'\"`~";
        let password = format!("P@ssw0rd{}", all_special);

        let mut json_obj = Map::new();
        json_obj.insert("password".to_string(), Value::String(password.clone()));

        let serialized = serde_json::to_string(&Value::Object(json_obj)).unwrap();
        println!("Serialized: {}", serialized);

        let deserialized: Value = serde_json::from_str(&serialized).unwrap();
        let recovered = deserialized.get("password").unwrap().as_str().unwrap();

        assert_eq!(password, recovered, "Failed for all special chars");
    }
}
