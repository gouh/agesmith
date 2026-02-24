# Problema con Caracteres Especiales en .env y .ini

## Diagnóstico

El problema ocurre cuando contraseñas contienen caracteres especiales que tienen significado especial en formatos .env e .ini:

### Caracteres problemáticos:

1. **`#`** - Inicio de comentario en .env y .ini
2. **`;`** - Inicio de comentario en .ini
3. **`=`** - Separador clave-valor (múltiples `=` pueden confundir parsers)
4. **Espacios** - Pueden requerir comillas
5. **Comillas** - Necesitan escaparse

## Causa raíz

Cuando SOPS convierte de JSON a dotenv/ini, los valores NO se entrecomillan automáticamente. Esto causa que:

```env
PASSWORD=pass#word123
```

Sea interpretado por algunos parsers como:

```env
PASSWORD=pass    # comentario: word123
```

## Solución

### Opción 1: Entrecomillar valores en .env/.ini (RECOMENDADO)

Modificar `save_changes()` para entrecomillar valores que contengan caracteres especiales:

```rust
fn needs_quoting(value: &str) -> bool {
    value.contains('#') || 
    value.contains(';') || 
    value.contains(' ') ||
    value.starts_with('"') ||
    value.starts_with('\'')
}

fn quote_value(value: &str) -> String {
    if needs_quoting(value) {
        // Escapar comillas dobles existentes
        let escaped = value.replace('"', r#"\""#);
        format!(r#""{}""#, escaped)
    } else {
        value.to_string()
    }
}
```

### Opción 2: Usar solo JSON/YAML para secretos complejos

Recomendar a los usuarios que usen JSON o YAML cuando tengan contraseñas con caracteres especiales, ya que estos formatos manejan el escaping correctamente.

### Opción 3: Advertir al usuario

Detectar valores problemáticos y mostrar una advertencia:

```rust
fn validate_env_value(key: &str, value: &str) -> Option<String> {
    if value.contains('#') {
        return Some(format!(
            "⚠️  '{}' contiene '#' que puede ser interpretado como comentario en .env",
            key
        ));
    }
    if value.contains(';') {
        return Some(format!(
            "⚠️  '{}' contiene ';' que puede ser interpretado como comentario en .ini",
            key
        ));
    }
    None
}
```

## Tests para verificar

Ejecuta:

```bash
# Tests unitarios
cargo test --test special_chars_test -- --nocapture
cargo test --test comment_handling_test -- --nocapture

# Diagnóstico con SOPS real
./tests/diagnose_sops.sh
```

## Casos de prueba

Valores que DEBEN funcionar correctamente:

- `pass=word=123` (múltiples =)
- `pass#word#123` (hash)
- `pass;word;123` (punto y coma)
- `pass word 123` (espacios)
- `pass"word"123` (comillas)
- `P@$$w0rd!#%&*()=+[]{}|;:'",.<>?/\~\`` (complejo)

## Implementación recomendada

Ver archivo: `src/sops_value_quoting.rs` (a crear)
