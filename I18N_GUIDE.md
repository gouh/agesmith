# Guía de Internacionalización (i18n)

## Configuración

El idioma se configura en `~/.config/tui-sops/config.toml`:

```toml
theme = "dark"
auto_lock_minutes = 15
clipboard_clear_seconds = 3
language = "es"  # "es" para español, "en" para inglés
```

## Uso en el Código

### 1. Acceder a traducciones

```rust
// En cualquier función que tenga acceso a `app`
let text = app.i18n.t("copy_value");  // "✓ Valor copiado al portapapeles"
```

### 2. Cambiar idioma en tiempo de ejecución

```rust
use crate::config::Language;

app.i18n.set_language(Language::English);
```

### 3. Agregar nuevas traducciones

Edita `src/i18n.rs` y agrega entradas al HashMap:

```rust
translations.insert("nueva_clave", ("Texto en español", "Text in English"));
```

## Claves Disponibles

| Clave | Español | English |
|-------|---------|---------|
| `explorer` | 📂 Explorador | 📂 Explorer |
| `secrets` | 🔐 Secretos | 🔐 Secrets |
| `keys` | 🔑 Llaves | 🔑 Keys |
| `help` | ⌨️ Atajos de Teclado | ⌨️ Keyboard Shortcuts |
| `search` | 🔍 Buscar | 🔍 Search |
| `copy_value` | ✓ Valor copiado | ✓ Value copied |
| `copy_key` | ✓ Clave copiada | ✓ Key copied |
| `saved` | ✓ Archivo guardado | ✓ File saved |
| `deleted` | ✓ Secreto eliminado | ✓ Secret deleted |
| `added` | ✓ Secreto agregado | ✓ Secret added |
| `updated` | ✓ Secreto actualizado | ✓ Secret updated |

## Ejemplo de Implementación

Para usar traducciones en la UI, reemplaza textos hardcodeados:

### Antes:
```rust
app.set_temp_message("✓ Valor copiado al portapapeles".to_string());
```

### Después:
```rust
app.set_temp_message(app.i18n.t("copy_value").to_string());
```

## Arquitectura

```
┌─────────────┐
│  config.rs  │ ◄─── Define Language enum
└──────┬──────┘
       │
       ▼
┌─────────────┐
│   i18n.rs   │ ◄─── HashMap de traducciones
└──────┬──────┘
       │
       ▼
┌─────────────┐
│   state.rs  │ ◄─── App contiene I18n
└──────┬──────┘
       │
       ▼
┌─────────────┐
│    ui.rs    │ ◄─── Usa app.i18n.t("key")
└─────────────┘
```

## Próximos Pasos

Para completar la internacionalización:

1. **Reemplazar todos los textos hardcodeados** en `ui.rs` con llamadas a `app.i18n.t()`
2. **Agregar atajo de teclado** para cambiar idioma (ej: `Ctrl+L`)
3. **Persistir cambios** de idioma en `config.toml`
4. **Agregar más idiomas** (francés, alemán, etc.)

## Ejemplo Completo

```rust
// En events.rs - agregar handler para cambiar idioma
KeyCode::Char('l') if key.modifiers.contains(KeyModifiers::CONTROL) => {
    let new_lang = match app.i18n.current_language() {
        Language::Spanish => Language::English,
        Language::English => Language::Spanish,
    };
    app.i18n.set_language(new_lang);
    app.set_temp_message(app.i18n.t("language_changed").to_string());
}
```

