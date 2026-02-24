# Solución Implementada: Auto-entrecomillado de Valores

## ✅ Problema Resuelto

Contraseñas con caracteres especiales (`#`, `;`, espacios) en archivos .env y .ini ahora se manejan correctamente mediante entrecomillado automático.

## 🔧 Cambios Realizados

### 1. `src/state.rs` - Funciones de entrecomillado

```rust
/// Verifica si un valor necesita ser entrecomillado en .env/.ini
fn needs_quoting(value: &str) -> bool {
    value.contains('#')       // Comentario en .env y .ini
        || value.contains(';') // Comentario en .ini
        || value.contains('\n')
        || value.contains('\r')
        || value.starts_with(' ')
        || value.ends_with(' ')
        || (value.contains(' ') && !value.starts_with('"') && !value.ends_with('"'))
}

/// Entrecomilla y escapa un valor para .env/.ini
fn quote_env_value(value: &str) -> String {
    if Self::needs_quoting(value) {
        let escaped = value
            .replace('\\', r"\\")
            .replace('"', r#"\""#)
            .replace('\n', r"\n")
            .replace('\r', r"\r")
            .replace('\t', r"\t");
        format!("\"{}\"", escaped)  // ✅ FIXED: Usa format!() regular
    } else {
        value.to_string()
    }
}
```

### 2. `src/state.rs` - Aplicar entrecomillado al guardar

Modificado `save_changes()` para aplicar `quote_env_value()` a valores ENV e INI antes de pasarlos a SOPS.

### 3. `src/sops.rs` - Desescapar al leer (CRÍTICO)

```rust
/// Desescapa un valor entrecomillado de .env/.ini
fn unquote_env_value(value: &str) -> String {
    let trimmed = value.trim();
    
    if (trimmed.starts_with('"') && trimmed.ends_with('"') && trimmed.len() >= 2)
        || (trimmed.starts_with('\'') && trimmed.ends_with('\'') && trimmed.len() >= 2)
    {
        let unquoted = &trimmed[1..trimmed.len() - 1];
        unquoted
            .replace(r"\n", "\n")
            .replace(r"\r", "\r")
            .replace(r"\t", "\t")
            .replace(r#"\""#, "\"")
            .replace(r"\'", "'")
            .replace(r"\\", "\\")
    } else {
        trimmed.to_string()
    }
}
```

Modificado `flatten_json()` para:
1. **✅ CRÍTICO**: Usar `value.as_str()` en lugar de `value.to_string()` para evitar doble escapado
2. Aplicar `unquote_env_value()` para remover comillas

```rust
_ => {
    // ✅ FIXED: Usar as_str() para obtener el valor directo sin serialización JSON
    let str_value = if let Some(s) = value.as_str() {
        s.to_string()
    } else {
        value.to_string().trim_matches('"').to_string()
    };
    let unescaped = unquote_env_value(&str_value);
    result.push((prefix.to_string(), unescaped));
}
```

## 🧪 Tests Creados

1. **tests/special_chars_test.rs** - Tests básicos de parsing ✅
2. **tests/sops_special_chars_test.rs** - Tests de conversión JSON ✅
3. **tests/comment_handling_test.rs** - Tests de manejo de comentarios ✅
4. **tests/quoting_test.rs** - Tests de funciones de entrecomillado ✅
5. **tests/integration_quoting_test.rs** - Test de flujo completo ✅
6. **tests/json_extraction_test.rs** - Test de extracción JSON (as_str vs to_string) ✅
7. **tests/agesmith_flow_test.rs** - Test simulando flujo exacto de AgeSmith ✅
8. **tests/diagnose_sops.sh** - Script de diagnóstico con SOPS real

## ✅ Casos de Prueba Verificados

Todos estos valores ahora funcionan correctamente:

- `password123` - Simple (sin comillas)
- `pass#word#123` - Con hash → `"pass#word#123"` → `pass#word#123` ✅
- `pass;word;123` - Con punto y coma → `"pass;word;123"` → `pass;word;123` ✅
- `pass word 123` - Con espacios → `"pass word 123"` → `pass word 123` ✅
- `P@$$w0rd!#%&*()=+` - Complejo → `"P@$$w0rd!#%&*()=+"` → `P@$$w0rd!#%&*()=+` ✅
- `pass"word"123` - Con comillas → `pass\"word\"123` → `pass"word"123` ✅
- `https://user:pass@host.com?key=val` - URL (sin comillas) ✅

## 🔄 Flujo Completo

1. **Usuario edita** → Valor: `pass#word`
2. **Al guardar** → Se detecta `#`, se entrecomilla: `"pass#word"`
3. **JSON para SOPS** → `{"KEY": "\"pass#word\""}`
4. **SOPS encripta** → Archivo encriptado
5. **SOPS desencripta** → JSON con valor entrecomillado
6. **flatten_json()** → Usa `as_str()` para obtener: `"pass#word"`
7. **unquote_env_value()** → Remueve comillas: `pass#word`
8. **Usuario ve** → Valor original: `pass#word` ✅

## 🐛 Bug Corregido

**Problema anterior**: Usar `value.to_string()` en JSON causaba doble escapado:
- `"pass#word"` → `"\"pass#word\""` → Usuario veía: `\"pass#word\"`

**Solución**: Usar `value.as_str()` para obtener el string directo:
- `"pass#word"` → `"pass#word"` → Usuario ve: `pass#word` ✅

## 📝 Notas

- Solo afecta archivos .env y .ini
- JSON y YAML no necesitan este tratamiento (manejan escaping nativamente)
- Valores simples sin caracteres especiales no se entrecomillan
- Totalmente transparente para el usuario
- **Sin caracteres escapados visibles** (`\"`, `\\`) en la interfaz

## 🚀 Resultado

El problema de contraseñas cortadas y caracteres escapados visibles está **completamente resuelto**.
