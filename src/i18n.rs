use std::collections::HashMap;
use crate::config::Language;

pub struct I18n {
    lang: Language,
    translations: HashMap<&'static str, (&'static str, &'static str)>,
}

impl I18n {
    pub fn new(lang: Language) -> Self {
        let mut translations = HashMap::new();
        
        // Títulos principales
        translations.insert("app_tagline", ("Forjando secretos seguros con age", "Forging secure secrets with age"));
        translations.insert("explorer", ("📂 Explorador", "📂 Explorer"));
        translations.insert("secrets", ("🔐 Secretos", "🔐 Secrets"));
        translations.insert("keys", ("🔑 Llaves", "🔑 Keys"));
        translations.insert("help", ("⌨️ Atajos de Teclado", "⌨️ Keyboard Shortcuts"));
        translations.insert("search", ("🔍 Buscar", "🔍 Search"));
        translations.insert("search_secret", ("🔍 Buscar secreto", "🔍 Search secret"));
        translations.insert("search_key", ("🔍 Buscar llave", "🔍 Search key"));
        translations.insert("confirm", ("⚠️ Confirmar", "⚠️ Confirm"));
        
        // Mensajes de éxito
        translations.insert("copy_value", ("✓ Valor copiado al portapapeles", "✓ Value copied to clipboard"));
        translations.insert("copy_key", ("✓ Clave copiada al portapapeles", "✓ Key copied to clipboard"));
        translations.insert("saved", ("✓ Archivo guardado", "✓ File saved"));
        translations.insert("deleted", ("✓ Secreto eliminado", "✓ Secret deleted"));
        translations.insert("added", ("✓ Secreto agregado", "✓ Secret added"));
        translations.insert("updated", ("✓ Secreto actualizado", "✓ Secret updated"));
        translations.insert("generated", ("✓ Secreto generado y copiado", "✓ Secret generated and copied"));
        translations.insert("fav_added", ("⭐ Agregado a favoritos", "⭐ Added to favorites"));
        translations.insert("fav_removed", ("⭐ Removido de favoritos", "⭐ Removed from favorites"));
        
        // Mensajes de error
        translations.insert("error_save", ("❌ Error al guardar", "❌ Error saving"));
        translations.insert("error_decrypt", ("❌ Error al desencriptar", "❌ Error decrypting"));
        translations.insert("error_regex", ("❌ Regex inválido", "❌ Invalid regex"));
        translations.insert("error_empty_key", ("❌ La clave no puede estar vacía", "❌ Key cannot be empty"));
        translations.insert("error_no_key_match", ("❌ Ninguna llave coincide", "❌ No matching key"));
        translations.insert("error_decrypt_key", ("Error: No se pudo desencriptar con esta llave", "Error: Could not decrypt with this key"));
        translations.insert("error_encrypt", ("Error al encriptar", "Error encrypting"));
        translations.insert("error_convert_key", ("Error al convertir llave privada a pública", "Error converting private key to public"));
        translations.insert("error_save_config", ("❌ Error al guardar configuración", "❌ Error saving config"));
        translations.insert("config_saved", ("✓ Configuración guardada", "✓ Configuration saved"));
        translations.insert("session_locked", ("🔒 Sesión bloqueada por inactividad", "🔒 Session locked due to inactivity"));
        translations.insert("sops_initialized", ("✓ Archivo .sops.yaml creado", "✓ .sops.yaml file created"));
        translations.insert("sops_exists", ("⚠️ .sops.yaml ya existe", "⚠️ .sops.yaml already exists"));
        translations.insert("no_keys", ("❌ No hay llaves age disponibles", "❌ No age keys available"));
        translations.insert("key_generated", ("✓ Nueva llave age generada", "✓ New age key generated"));
        translations.insert("key_deleted", ("✓ Llave eliminada", "✓ Key deleted"));
        translations.insert("key_manager_title", ("🔑 Gestor de Llaves Age", "🔑 Age Key Manager"));
        translations.insert("confirm_key_deletion", ("⚠️ Confirmar Eliminación", "⚠️ Confirm Deletion"));
        translations.insert("confirm_key_creation", ("✓ Crear Clave Age", "✓ Create Age Key"));
        translations.insert("no_keys_found", ("No se encontraron claves age", "No age keys found"));
        translations.insert("keys_file_missing", ("El archivo ~/.config/sops/age/keys.txt no existe.", "The file ~/.config/sops/age/keys.txt does not exist."));
        translations.insert("create_key_question", ("¿Deseas crear una nueva clave age?", "Do you want to create a new age key?"));
        translations.insert("create_key_confirm", ("[y/s] Crear clave", "[y/s] Create key"));
        translations.insert("key_created", ("✓ Clave age creada exitosamente", "✓ Age key created successfully"));
        translations.insert("confirm_key_creation_help", ("[y/s] Crear | [n/Esc] Cancelar", "[y/s] Create | [n/Esc] Cancel"));
        translations.insert("delete_key_question", ("¿Eliminar la llave", "Delete key"));
        translations.insert("action_irreversible", ("Esta acción no se puede deshacer.", "This action cannot be undone."));
        translations.insert("key_manager_help", ("[↑↓] Navegar | [n] Nueva llave | [d] Eliminar | [Esc] Cerrar", "[↑↓] Navigate | [n] New key | [d] Delete | [Esc] Close"));
        translations.insert("confirm_deletion_help", ("[y] Confirmar eliminación | [n] Cancelar", "[y] Confirm deletion | [n] Cancel"));
        translations.insert("folder_created", ("✓ Carpeta creada", "✓ Folder created"));
        translations.insert("file_renamed", ("✓ Archivo renombrado", "✓ File renamed"));
        translations.insert("file_deleted", ("✓ Eliminado", "✓ Deleted"));
        translations.insert("file_exists", ("El archivo ya existe", "File already exists"));
        translations.insert("file_created", ("Archivo encriptado creado", "Encrypted file created"));
        translations.insert("new_folder_title", ("📁 Nueva Carpeta", "📁 New Folder"));
        translations.insert("new_file_title", ("🔐 Nuevo Archivo Encriptado", "🔐 New Encrypted File"));
        translations.insert("select_format_title", ("📄 Seleccionar Formato", "📄 Select Format"));
        translations.insert("enter_file_name", ("Nombre del archivo:", "File name:"));
        translations.insert("rename_title", ("✏️ Renombrar", "✏️ Rename"));
        translations.insert("confirm_file_deletion", ("⚠️ Confirmar Eliminación", "⚠️ Confirm Deletion"));
        translations.insert("delete_file_question", ("¿Eliminar", "Delete"));
        translations.insert("enter_folder_name", ("Nombre de la carpeta:", "Folder name:"));
        translations.insert("enter_new_name", ("Nuevo nombre:", "New name:"));
        
        // Etiquetas y campos
        translations.insert("key", ("🔑 Key", "🔑 Key"));
        translations.insert("value", ("🔐 Value", "🔐 Value"));
        translations.insert("value_field", ("Valor", "Value"));
        translations.insert("marked", ("marcados", "marked"));
        translations.insert("favorites", ("favoritos", "favorites"));
        translations.insert("no_file", ("Ningún archivo seleccionado", "No file selected"));
        translations.insert("active", ("[ACTIVA]", "[ACTIVE]"));
        translations.insert("matches", ("coincide", "matches"));
        
        // Instrucciones
        translations.insert("press_search", ("Presiona [/] para buscar", "Press [/] to search"));
        translations.insert("search_cancel", ("Esc: cancelar, Enter: aplicar", "Esc: cancel, Enter: apply"));
        translations.insert("search_regex", ("[r] toggle regex", "[r] toggle regex"));
        translations.insert("regex_active", ("[REGEX activo]", "[REGEX active]"));
        translations.insert("close_help", ("[Esc: Cerrar]", "[Esc: Close]"));
        translations.insert("press_k", ("Presiona [k]", "Press [k]"));
        translations.insert("search_secret_regex", ("🔎 Buscar secreto [REGEX]", "🔎 Search secret [REGEX]"));
        translations.insert("search_secret_normal", ("🔍 Buscar secreto", "🔍 Search secret"));
        translations.insert("search_key_title", ("🔍 Buscar llave", "🔍 Search key"));
        translations.insert("keys_search_apply", ("[/: Buscar | Enter: Aplicar | Esc: Cancelar]", "[/: Search | Enter: Apply | Esc: Cancel]"));
        translations.insert("keys_matches", ("✓ = coincide", "✓ = matches"));
        translations.insert("scroll_json", ("[↑↓: scroll | j: toggle JSON]", "[↑↓: scroll | j: toggle JSON]"));
        translations.insert("tab_switch", ("[Tab: cambiar campo]", "[Tab: switch field]"));
        translations.insert("enter_save", ("[Enter: guardar]", "[Enter: save]"));
        translations.insert("esc_cancel", ("[Esc: cancelar]", "[Esc: cancel]"));
        translations.insert("esc_close", ("[Esc/z: cerrar]", "[Esc/z: close]"));
        translations.insert("confirm_y", ("[y] Confirmar eliminación", "[y] Confirm deletion"));
        translations.insert("cancel_n", ("[n] Cancelar", "[n] Cancel]"));
        translations.insert("move_cursor", ("[←→] Mover cursor", "[←→] Move cursor"));
        translations.insert("home_end", ("[Home/End] Inicio/Fin", "[Home/End] Home/End"));
        translations.insert("settings_cmd", ("[Ctrl+S] Settings", "[Ctrl+S] Settings"));
        
        // Modales
        translations.insert("edit_secret", ("✏️ Editar Secreto", "✏️ Edit Secret"));
        translations.insert("add_secret", ("➕ Agregar Nuevo Secreto", "➕ Add New Secret"));
        translations.insert("modal_help", ("[Tab: cambiar campo | Enter: guardar | Esc: cancelar]", "[Tab: switch field | Enter: save | Esc: cancel]"));
        translations.insert("confirm_delete", ("¿Eliminar este secreto?", "Delete this secret?"));
        translations.insert("full_value", ("📄 Valor Completo", "📄 Full Value"));
        translations.insert("value_help", ("[Esc/z: cerrar | ↑↓: scroll | j: toggle JSON]", "[Esc/z: close | ↑↓: scroll | j: toggle JSON]"));
        translations.insert("lines", ("líneas", "lines"));
        
        // Comandos del footer
        translations.insert("cmd_navigate", ("[↑↓] Navegar", "[↑↓] Navigate"));
        translations.insert("cmd_open", ("[Enter] Abrir", "[Enter] Open"));
        translations.insert("cmd_mark", ("[m] Marcar", "[m] Mark"));
        translations.insert("cmd_tab_secrets", ("[Tab] Secretos", "[Tab] Secrets"));
        translations.insert("cmd_tab_explorer", ("[Tab] Explorador", "[Tab] Explorer"));
        translations.insert("cmd_help", ("[?] Ayuda", "[?] Help"));
        translations.insert("cmd_quit", ("[q] Salir", "[q] Quit"));
        translations.insert("cmd_show", ("[v] Ver", "[v] Show"));
        translations.insert("cmd_hide", ("[v] Ocultar", "[v] Hide"));
        translations.insert("cmd_zoom", ("[z] Zoom", "[z] Zoom"));
        translations.insert("cmd_copy", ("[c] Copiar", "[c] Copy"));
        translations.insert("cmd_copy_key", ("[C] Copiar clave", "[C] Copy key"));
        translations.insert("cmd_favorite", ("[f] Favorito", "[f] Favorite"));
        translations.insert("cmd_edit", ("[e] Editar", "[e] Edit"));
        translations.insert("cmd_new", ("[n] Nuevo", "[n] New"));
        translations.insert("cmd_delete", ("[d] Eliminar", "[d] Delete"));
        translations.insert("cmd_search", ("[/] Buscar", "[/] Search"));
        translations.insert("cmd_generate", ("[g] Generar", "[g] Generate"));
        translations.insert("cmd_save", ("[s] Guardar", "[s] Save"));
        translations.insert("cmd_apply", ("[Enter] Aplicar", "[Enter] Apply"));
        translations.insert("cmd_cancel", ("[Esc] Cancelar", "[Esc] Cancel"));
        translations.insert("cmd_confirm", ("[y] Confirmar eliminación", "[y] Confirm deletion"));
        translations.insert("cmd_key_selector", ("[k] Selector", "[k] Selector"));
        translations.insert("cmd_key_manager", ("[K] Gestionar llaves", "[K] Manage keys"));
        translations.insert("cmd_new_folder", ("[N] Nueva carpeta", "[N] New folder"));
        translations.insert("cmd_new_file", ("[n] New secrets file", "[n] New secrets file"));
        translations.insert("cmd_rename", ("[r] Renombrar", "[r] Rename"));
        translations.insert("cmd_rename_file", ("Renombrar", "Rename"));
        translations.insert("cmd_delete_file", ("[D] Eliminar", "[D] Delete"));
        translations.insert("cmd_delete_item", ("Eliminar", "Delete"));
        translations.insert("cmd_edit_item", ("Editar", "Edit"));
        translations.insert("cmd_init", ("[i] Inicializar SOPS", "[i] Init SOPS"));
        translations.insert("cmd_cancel_n", ("[n] Cancelar", "[n] Cancel"));
        
        // Footers
        translations.insert("footer_create_folder", ("[Enter] Crear | [Esc] Cancelar", "[Enter] Create | [Esc] Cancel"));
        translations.insert("footer_rename", ("[Enter] Renombrar | [Esc] Cancelar", "[Enter] Rename | [Esc] Cancel"));
        translations.insert("footer_select_format", ("[↑↓] Navegar | [Enter] Continuar | [Esc] Cancelar", "[↑↓] Navigate | [Enter] Continue | [Esc] Cancel"));
        translations.insert("footer_create_file", ("[Enter] Crear | [Esc] Cancelar", "[Enter] Create | [Esc] Cancel"));
        translations.insert("footer_key_manager", ("[↑↓] Navegar | [n] Nueva llave", "[↑↓] Navigate | [n] New key"));
        translations.insert("footer_close", ("[Esc] Cerrar", "[Esc] Close"));
        translations.insert("delete_key_cmd", ("[d] Eliminar", "[d] Delete"));
        translations.insert("format_yaml", ("YAML (.yaml)", "YAML (.yaml)"));
        translations.insert("format_json", ("JSON (.json)", "JSON (.json)"));
        translations.insert("format_env", ("ENV (.env)", "ENV (.env)"));
        translations.insert("format_ini", ("INI (.ini)", "INI (.ini)"));
        translations.insert("unnamed_key", ("Sin nombre", "Unnamed"));
        translations.insert("no_keys_selected", ("Debes seleccionar al menos una llave", "You must select at least one key"));
        translations.insert("sops_not_initialized", ("Primero inicializa SOPS con [i]", "First initialize SOPS with [i]"));
        translations.insert("select_sops_keys_title", ("🔑 Seleccionar Llaves para SOPS", "🔑 Select Keys for SOPS"));
        translations.insert("footer_select_sops_keys", ("[↑↓] Navegar | [Espacio/Enter] Seleccionar | [Esc] Cancelar", "[↑↓] Navigate | [Space/Enter] Select | [Esc] Cancel"));
        translations.insert("footer_edit_sops", ("[Ctrl+Enter] Guardar | [Esc] Cancelar", "[Ctrl+Enter] Save | [Esc] Cancel"));
        translations.insert("footer_select_template", ("[↑↓] Navegar | [Enter] Seleccionar | [Esc] Cancelar", "[↑↓] Navigate | [Enter] Select | [Esc] Cancel"));
        translations.insert("editing_sops", ("Editando", "Editing"));
        translations.insert("sops_saved", ("✓ .sops.yaml guardado", "✓ .sops.yaml saved"));
        translations.insert("select_template_title", ("📋 Seleccionar Template SOPS", "📋 Select SOPS Template"));
        translations.insert("template_simple", ("Por formato - .env/.json/.yaml/.ini", "By format - .env/.json/.yaml/.ini"));
        translations.insert("template_simple_desc", ("Reglas separadas para cada tipo de archivo", "Separate rules for each file type"));
        translations.insert("template_by_type", ("Por entorno - dev/staging/prod", "By environment - dev/staging/prod"));
        translations.insert("template_by_type_desc", ("Diferentes llaves según el entorno", "Different keys based on environment"));
        translations.insert("template_regex", ("Solo valores sensibles - encrypted_regex", "Sensitive values only - encrypted_regex"));
        translations.insert("template_regex_desc", ("Encripta password, secret, key, token, etc.", "Encrypts password, secret, key, token, etc."));
        translations.insert("template_k8s", ("Kubernetes - data/stringData", "Kubernetes - data/stringData"));
        translations.insert("template_k8s_desc", ("Para archivos YAML de Kubernetes (data/stringData)", "For Kubernetes YAML files (data/stringData)"));
        
        // Categorías del footer
        translations.insert("cat_navigation", ("Navegación", "Navigation"));
        translations.insert("cat_files", ("Archivos", "Files"));
        translations.insert("cat_management", ("Gestión", "Management"));
        translations.insert("cat_view", ("Vista", "View"));
        translations.insert("cat_editing", ("Edición", "Editing"));
        translations.insert("cat_tools", ("Herramientas", "Tools"));
        
        // Estados
        translations.insert("selecting_key", ("Seleccionando llave...", "Selecting key..."));
        translations.insert("searching_key", ("Buscando llave...", "Searching key..."));
        translations.insert("searching_secret", ("Buscando secreto", "Searching secret"));
        translations.insert("viewing_value", ("Viendo valor", "Viewing value"));
        translations.insert("generating", ("Generando...", "Generating..."));
        translations.insert("modified", (" *", " *"));
        
        // Help sections
        translations.insert("help_explorer", ("📂 Explorador", "📂 Explorer"));
        translations.insert("help_secrets", ("🔐 Secretos", "🔐 Secrets"));
        translations.insert("help_key_selector", ("🔑 Selector de Llaves", "🔑 Key Selector"));
        translations.insert("help_search", ("🔍 Búsqueda", "🔍 Search"));
        translations.insert("help_general", ("⌨️ General", "⌨️ General"));
        
        // Help commands
        translations.insert("help_nav_files", ("Navegar por archivos y carpetas", "Navigate through files and folders"));
        translations.insert("help_open_dir", ("Abrir carpeta o cargar archivo encriptado", "Open folder or load encrypted file"));
        translations.insert("help_mark", ("Marcar/desmarcar archivo para operaciones", "Mark/unmark file for batch operations"));
        translations.insert("help_init_sops", ("Crear archivo .sops.yaml en el directorio", "Create .sops.yaml file in directory"));
        translations.insert("help_change_secrets", ("Ir al panel de secretos", "Go to secrets panel"));
        translations.insert("help_key_selector_open", ("Abrir selector de llaves age", "Open age key selector"));
        translations.insert("help_nav_secrets", ("Navegar por la lista de secretos", "Navigate through secrets list"));
        translations.insert("help_show_hide", ("Mostrar u ocultar valores encriptados", "Show or hide encrypted values"));
        translations.insert("help_zoom", ("Ver valor completo en modal con scroll", "View full value in modal with scroll"));
        translations.insert("help_copy_value", ("Copiar valor del secreto al portapapeles", "Copy secret value to clipboard"));
        translations.insert("help_copy_key", ("Copiar nombre de la clave al portapapeles", "Copy key name to clipboard"));
        translations.insert("help_favorite", ("Agregar/quitar archivo de favoritos", "Add/remove file from favorites"));
        translations.insert("help_edit", ("Editar el secreto seleccionado", "Edit selected secret"));
        translations.insert("help_new", ("Agregar un nuevo secreto al archivo", "Add a new secret to file"));
        translations.insert("help_delete", ("Eliminar el secreto seleccionado", "Delete selected secret"));
        translations.insert("help_save", ("Guardar cambios al archivo (re-encripta)", "Save changes to file (re-encrypts)"));
        translations.insert("help_generate", ("Generar contraseña o token seguro", "Generate secure password or token"));
        translations.insert("help_search_secrets", ("Buscar secretos por clave o valor", "Search secrets by key or value"));
        translations.insert("help_back_explorer", ("Regresar al explorador de archivos", "Return to file explorer"));
        translations.insert("help_nav_keys", ("Navegar por las llaves disponibles", "Navigate through available keys"));
        translations.insert("help_search_keys", ("Buscar llaves por nombre o clave pública", "Search keys by name or public key"));
        translations.insert("help_apply_key", ("Usar la llave seleccionada para desencriptar", "Use selected key to decrypt"));
        translations.insert("help_cancel", ("Cancelar y cerrar", "Cancel and close"));
        translations.insert("help_filter", ("Escribir para filtrar resultados", "Type to filter results"));
        translations.insert("help_type", ("Escribir", "Type"));
        translations.insert("help_apply_filter", ("Aplicar el filtro de búsqueda", "Apply search filter"));
        translations.insert("help_cancel_search", ("Cancelar búsqueda y limpiar filtro", "Cancel search and clear filter"));
        translations.insert("help_show_help", ("Mostrar u ocultar este panel de ayuda", "Show or hide this help panel"));
        translations.insert("help_quit", ("Salir de la aplicación", "Quit application"));
        
        // Misc
        translations.insert("key_info_none", ("Ninguna", "None"));
        translations.insert("key_info_auto", ("Auto", "Auto"));
        translations.insert("key_info", ("Llave", "Key"));
        translations.insert("available_keys", ("Llaves disponibles", "Available keys"));
        translations.insert("recipients", ("Recipients", "Recipients"));
        translations.insert("unnamed", ("Sin nombre", "Unnamed"));
        translations.insert("loading", ("Cargando", "Loading"));
        translations.insert("decrypting", ("Desencriptando", "Decrypting"));
        translations.insert("saving", ("Guardando", "Saving"));
        translations.insert("current_path", ("Ruta actual", "Current path"));
        
        // Settings
        translations.insert("settings_theme", ("Tema", "Theme"));
        translations.insert("settings_language", ("Idioma", "Language"));
        translations.insert("settings_autolock", ("Auto-bloqueo", "Auto-lock"));
        translations.insert("settings_timeout", ("Tiempo de mensaje", "Message timeout"));
        translations.insert("settings_disabled", ("deshabilitado", "disabled"));
        translations.insert("settings_minutes", ("minutos", "minutes"));
        translations.insert("settings_seconds", ("segundos", "seconds"));
        translations.insert("settings_change", ("[←/→ para cambiar]", "[←/→ to change]"));
        translations.insert("settings_adjust", ("[←/→ para ajustar]", "[←/→ to adjust]"));
        translations.insert("settings_theme_help", ("Cambia los colores de la interfaz (dark/light)", "Change interface colors (dark/light)"));
        translations.insert("settings_language_help", ("Cambia el idioma de la aplicación (es/en)", "Change application language (es/en)"));
        translations.insert("settings_autolock_help", ("Cierra archivos automáticamente tras inactividad (0=deshabilitado)", "Auto-close files after inactivity (0=disabled)"));
        translations.insert("settings_timeout_help", ("Tiempo antes de limpiar portapapeles y ocultar mensajes", "Time before clearing clipboard and hiding messages"));
        
        Self { lang, translations }
    }

    pub fn t<'a>(&'a self, key: &'a str) -> &'a str {
        self.translations
            .get(key)
            .map(|(es, en)| match self.lang {
                Language::Spanish => *es,
                Language::English => *en,
            })
            .unwrap_or(key)
    }

    pub fn set_language(&mut self, lang: Language) {
        self.lang = lang;
    }

    pub fn current_language(&self) -> Language {
        self.lang
    }
}

impl Default for I18n {
    fn default() -> Self {
        Self::new(Language::English)
    }
}
