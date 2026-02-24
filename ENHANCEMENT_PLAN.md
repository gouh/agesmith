# TUI-SOPS Enhancement Plan

## 📋 Project Overview
Mejoras planificadas para TUI-SOPS organizadas por agentes especializados para desarrollo paralelo sin conflictos.

---

## 🎯 Agent 1: Core Editing & File Operations
**Responsabilidad**: Edición de secretos, operaciones CRUD, guardado

### Tasks
- [x] **1.1** Implementar edición in-place de valores ✅
  - Modal de edición con input multilinea
  - Validación de formato (JSON, string, number)
  - Función `edit_secret_value(key, new_value)`
  
- [x] **1.2** Agregar nuevo secreto ✅
  - Modal para key + value
  - Validar que key no exista
  - Función `add_new_secret(key, value)`
  
- [x] **1.3** Eliminar secreto ✅
  - Confirmación antes de eliminar
  - Función `delete_secret(key)`
  
- [x] **1.4** Guardar cambios ✅
  - Re-encriptar archivo con SOPS
  - Mantener recipients originales
  - Función `save_file_with_sops()`
  - Indicador visual de archivo modificado (*)

### Files to Modify
- `src/main.rs`: Agregar `InputMode::Editing`, `InputMode::Confirming`
- Nuevas funciones en `impl App`: `edit_secret()`, `add_secret()`, `delete_secret()`, `save_changes()`

### Dependencies
- Ninguna (puede trabajar independientemente)

---

## 🔐 Agent 2: Secret Generation & Validation
**Responsabilidad**: Generación de secretos, validación, linting

### Tasks
- [x] **2.1** Generador de passwords ✅
  - Modal con opciones: longitud, caracteres especiales, números
  - Usar `rand` crate para generación segura
  - Función `generate_password(length, options)`
  
- [x] **2.2** Generador de tokens/API keys ✅
  - Diferentes formatos: hex, base64, uuid
  - Función `generate_token(format, length)`
  
- [ ] **2.3** Validación de secretos
  - Detectar passwords débiles
  - Validar formato JSON/YAML en valores
  - Función `validate_secret(value) -> Vec<Warning>`
  
- [ ] **2.4** Detectar duplicados
  - Advertir si valor ya existe con otra key
  - Función `find_duplicate_values()`

### Files to Modify
- `src/main.rs`: Agregar `InputMode::Generating`
- Nuevo módulo: `src/generator.rs`
- Nuevo módulo: `src/validator.rs`

### Dependencies
- Agregar a `Cargo.toml`: `rand = "0.8"`, `uuid = "1.0"`

---

## 📁 Agent 3: Multi-File Management & Navigation
**Responsabilidad**: Gestión de múltiples archivos, favoritos, operaciones batch

### Tasks
- [ ] **3.1** Selección múltiple de archivos
  - Marcar/desmarcar con `[m]`
  - Visual indicator en explorador
  - Campo `selected_files: Vec<PathBuf>` en App
  
- [ ] **3.2** Favoritos
  - Guardar en `~/.config/tui-sops/favorites.json`
  - Agregar/quitar con `[f]`
  - Panel de favoritos con `[F]`
  
- [ ] **3.3** Copiar secretos entre archivos
  - Seleccionar secretos origen
  - Elegir archivo destino
  - Función `copy_secrets_to_file(secrets, dest_file)`
  
- [ ] **3.4** Breadcrumbs de navegación
  - Mostrar path completo en header
  - Navegación rápida con clicks

### Files to Modify
- `src/main.rs`: Agregar campos `selected_files`, `favorites`
- Nuevo módulo: `src/favorites.rs`
- Nuevo archivo config: `~/.config/tui-sops/favorites.json`

### Dependencies
- Ninguna adicional

---

## 🔍 Agent 4: Advanced Search & Filtering
**Responsabilidad**: Búsqueda avanzada, filtros, ordenamiento

### Tasks
- [ ] **4.1** Búsqueda con regex
  - Toggle regex mode con `[r]` en búsqueda
  - Función `search_with_regex(pattern)`
  
- [ ] **4.2** Búsqueda en múltiples archivos
  - Buscar en todos los archivos del directorio
  - Mostrar resultados agrupados por archivo
  - Función `search_across_files(query)`
  
- [ ] **4.3** Filtros avanzados
  - Por tipo: string, number, object, array
  - Por estado: encriptado, no encriptado
  - Por tamaño de valor
  - Campo `active_filters: Vec<Filter>` en App
  
- [ ] **4.4** Ordenamiento de tabla
  - Por key (A-Z, Z-A)
  - Por tipo de valor
  - Por longitud
  - Campo `sort_order: SortOrder` en App

### Files to Modify
- `src/main.rs`: Agregar `InputMode::FilterMenu`
- Nuevo módulo: `src/search.rs`
- Modificar `filtered_secrets()` para soportar filtros complejos

### Dependencies
- Agregar a `Cargo.toml`: `regex = "1.10"`

---

## 📊 Agent 5: Visualization & Display
**Responsabilidad**: Mejoras visuales, syntax highlighting, formateo

### Tasks
- [ ] **5.1** Modal de zoom para valores largos
  - `[z]` abre modal con valor completo
  - Scroll vertical para valores grandes
  - Función `show_value_modal(value)`
  
- [ ] **5.2** Pretty print JSON/YAML
  - Detectar formato automáticamente
  - Formatear con indentación
  - Función `format_value(value, format)`
  
- [ ] **5.3** Syntax highlighting
  - Colorear JSON keys/values
  - Colorear URLs, IPs, emails
  - Usar colores del tema
  
- [ ] **5.4** Indicadores visuales mejorados
  - 🔒 Archivo readonly
  - ⚠️ Secreto débil
  - 📅 Próximo a expirar
  - ✏️ Archivo modificado

### Files to Modify
- `src/main.rs`: Agregar `InputMode::ViewingValue`
- Nuevo módulo: `src/formatter.rs`
- Modificar función `ui()` para syntax highlighting

### Dependencies
- Agregar a `Cargo.toml`: `syntect = "5.0"` (opcional, para syntax highlighting avanzado)

---

## 📜 Agent 6: History & Version Control
**Responsabilidad**: Historial, diff, rollback, integración con git

### Tasks
- [ ] **6.1** Ver historial de cambios
  - Integración con `git log` del archivo
  - Mostrar commits con `[h]`
  - Función `get_file_history()`
  
- [ ] **6.2** Diff entre versiones
  - Comparar versión actual vs anterior
  - Mostrar cambios lado a lado
  - Función `diff_versions(v1, v2)`
  
- [ ] **6.3** Rollback a versión anterior
  - Seleccionar commit del historial
  - Restaurar archivo
  - Función `rollback_to_version(commit_hash)`
  
- [ ] **6.4** Auto-commit en cambios
  - Opcional: commit automático al guardar
  - Mensaje de commit descriptivo

### Files to Modify
- `src/main.rs`: Agregar `InputMode::History`, `InputMode::Diff`
- Nuevo módulo: `src/git.rs`

### Dependencies
- Agregar a `Cargo.toml`: `git2 = "0.18"` (para integración git)

---

## 🎨 Agent 7: Templates & Import/Export
**Responsabilidad**: Templates, importar/exportar, conversión de formatos

### Tasks
- [ ] **7.1** Sistema de templates
  - Templates predefinidos: AWS, DB, API keys
  - Guardar en `~/.config/tui-sops/templates/`
  - Función `load_template(name)`
  
- [ ] **7.2** Importar desde .env
  - Parser de archivos .env
  - Convertir a formato SOPS
  - Función `import_from_env(file)`
  
- [ ] **7.3** Importar desde JSON/YAML
  - Detectar formato automáticamente
  - Función `import_from_file(file, format)`
  
- [ ] **7.4** Exportar a diferentes formatos
  - Exportar a .env, JSON, YAML (sin encriptar)
  - Advertencia de seguridad
  - Función `export_to_format(format, output_file)`

### Files to Modify
- `src/main.rs`: Agregar `InputMode::TemplateMenu`, `InputMode::ImportExport`
- Nuevo módulo: `src/templates.rs`
- Nuevo módulo: `src/import_export.rs`
- Nuevo directorio: `~/.config/tui-sops/templates/`

### Dependencies
- Agregar a `Cargo.toml`: `dotenv-parser = "0.1"` (para .env)

---

## 🔒 Agent 8: Security & Audit
**Responsabilidad**: Auto-lock, clipboard security, audit logs

### Tasks
- [ ] **8.1** Auto-lock por inactividad
  - Cerrar aplicación después de N minutos
  - Configurable en settings
  - Función `check_inactivity_timeout()`
  
- [ ] **8.2** Auto-clear clipboard
  - Limpiar clipboard después de 30 segundos
  - Configurable
  - Función `schedule_clipboard_clear()`
  
- [ ] **8.3** Modo presentación
  - Ocultar todos los valores
  - Toggle con `[P]`
  - Campo `presentation_mode: bool` en App
  
- [ ] **8.4** Audit logs
  - Registrar accesos a secretos
  - Guardar en `~/.config/tui-sops/audit.log`
  - Función `log_access(file, key, action)`

### Files to Modify
- `src/main.rs`: Agregar campos de seguridad
- Nuevo módulo: `src/security.rs`
- Nuevo archivo: `~/.config/tui-sops/audit.log`

### Dependencies
- Agregar a `Cargo.toml`: `chrono = "0.4"` (para timestamps)

---

## ⚙️ Agent 9: Configuration & Settings
**Responsabilidad**: Sistema de configuración, temas, atajos personalizables

### Tasks
- [ ] **9.1** Archivo de configuración
  - Crear `~/.config/tui-sops/config.toml`
  - Cargar al inicio
  - Función `load_config()`
  
- [ ] **9.2** Temas personalizables
  - Light, Dark, Custom
  - Definir en config
  - Función `apply_theme(theme)`
  
- [ ] **9.3** Atajos de teclado personalizables
  - Mapear acciones a teclas
  - Definir en config
  - Función `load_keybindings()`
  
- [ ] **9.4** Panel de settings
  - UI para cambiar configuración
  - Guardar cambios
  - Acceso con `[S]`

### Files to Modify
- `src/main.rs`: Agregar `InputMode::Settings`
- Nuevo módulo: `src/config.rs`
- Nuevo archivo: `~/.config/tui-sops/config.toml`

### Dependencies
- Agregar a `Cargo.toml`: `toml = "0.8"`, `serde = { version = "1.0", features = ["derive"] }`

---

## 🚀 Agent 10: UX Enhancements & Polish
**Responsabilidad**: Ayuda contextual, command palette, notificaciones

### Tasks
- [x] **10.1** Panel de ayuda contextual ✅
  - `[?]` muestra comandos disponibles
  - Ayuda específica por modo
  - Función `show_help_for_mode(mode)`
  
- [ ] **10.2** Command palette (Ctrl+P)
  - Búsqueda fuzzy de comandos
  - Ejecutar acciones por nombre
  - Función `show_command_palette()`
  
- [ ] **10.3** Sistema de notificaciones toast
  - Notificaciones no intrusivas
  - Stack de mensajes
  - Función `show_toast(message, type)`
  
- [ ] **10.4** Confirmaciones para acciones destructivas
  - Modal de confirmación
  - Función `confirm_action(message) -> bool`

### Files to Modify
- `src/main.rs`: Agregar `InputMode::Help`, `InputMode::CommandPalette`
- Nuevo módulo: `src/help.rs`
- Nuevo módulo: `src/notifications.rs`

### Dependencies
- Agregar a `Cargo.toml`: `fuzzy-matcher = "0.3"` (para command palette)

---

## 📦 Integration Points & Shared Resources

### Shared State (App struct)
Todos los agentes comparten el struct `App`. Coordinación necesaria:

```rust
struct App {
    // Existing fields...
    
    // Agent 1
    is_modified: bool,
    editing_key: Option<String>,
    
    // Agent 3
    selected_files: Vec<PathBuf>,
    favorites: Vec<PathBuf>,
    
    // Agent 4
    active_filters: Vec<Filter>,
    sort_order: SortOrder,
    
    // Agent 8
    presentation_mode: bool,
    last_activity: Instant,
    
    // Agent 9
    config: Config,
    
    // Agent 10
    notifications: Vec<Notification>,
}
```

### InputMode Extensions
Cada agente puede agregar sus propios modos:

```rust
enum InputMode {
    // Existing...
    Explorer,
    Secrets,
    SelectingKey,
    SearchingKey,
    SearchingSecrets,
    
    // Agent 1
    Editing,
    Confirming,
    
    // Agent 2
    Generating,
    
    // Agent 3
    SelectingMultiple,
    
    // Agent 4
    FilterMenu,
    
    // Agent 5
    ViewingValue,
    
    // Agent 6
    History,
    Diff,
    
    // Agent 7
    TemplateMenu,
    ImportExport,
    
    // Agent 9
    Settings,
    
    // Agent 10
    Help,
    CommandPalette,
}
```

---

## 🗓️ Implementation Phases

### Phase 1: Foundation (Weeks 1-2)
**Priority**: Core functionality
- Agent 1: Editing & CRUD operations
- Agent 9: Configuration system
- Agent 10: Help & UX basics

### Phase 2: Power Features (Weeks 3-4)
**Priority**: Productivity
- Agent 2: Secret generation
- Agent 4: Advanced search
- Agent 5: Better visualization

### Phase 3: Advanced (Weeks 5-6)
**Priority**: Professional features
- Agent 3: Multi-file management
- Agent 6: History & version control
- Agent 7: Templates & import/export

### Phase 4: Security & Polish (Week 7)
**Priority**: Production ready
- Agent 8: Security features
- Agent 10: Final UX polish
- Integration testing

---

## 🔄 Coordination Protocol

### Before Starting Work
1. ✅ Check this file for task status
2. ✅ Mark task as "In Progress" with your name
3. ✅ Review "Integration Points" section
4. ✅ Check for conflicts with other agents

### During Development
1. ✅ Update progress in this file
2. ✅ Document any new shared state
3. ✅ Communicate breaking changes
4. ✅ Write integration tests

### After Completion
1. ✅ Mark task as complete
2. ✅ Update documentation
3. ✅ Notify dependent agents
4. ✅ Create PR with clear description

---

## 📝 Notes & Decisions

### Architecture Decisions
- **Single binary**: Mantener todo en un ejecutable
- **Minimal dependencies**: Solo agregar crates necesarias
- **Backward compatibility**: No romper funcionalidad existente
- **Config location**: `~/.config/tui-sops/`

### Code Style
- **Error handling**: Usar `Result<T>` y `anyhow`
- **Async**: Solo donde sea necesario (file I/O, network)
- **Comments**: En español para consistencia
- **Testing**: Unit tests para lógica crítica

### UI/UX Guidelines
- **Keybindings**: Lowercase para acciones comunes, Uppercase para variantes
- **Colors**: Usar tema definido en Agent 9
- **Modals**: Centrados, con backdrop oscuro
- **Messages**: Temporales (3s) para éxito, permanentes para errores

---

## 🎯 Success Metrics

- [ ] Todas las tareas completadas
- [ ] Tests passing (>80% coverage)
- [ ] Documentation actualizada
- [ ] Performance: <100ms response time
- [ ] Memory: <50MB usage
- [ ] Zero crashes en testing
- [ ] User feedback positivo

---

## 📞 Contact & Support

**Project Lead**: Orquestador
**Repository**: `/Users/hugh/Documents/projects/personal/tui-sops`
**Documentation**: `README.md`
**Issues**: Track en este archivo

---

*Last Updated: 2026-02-23*
*Version: 1.0*
