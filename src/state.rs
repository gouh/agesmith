use crate::config::{Config, Theme};
use crate::generator::TokenFormat;
use crate::i18n::I18n;
use crate::sops::{decrypt_and_parse, get_encrypted_keys, get_sops_recipients, AgeKey};
use anyhow::{Context, Result};
use arboard::Clipboard;
use regex::Regex;
use serde_json::Value;
use std::{
    fs, path::{Path, PathBuf}, process::Command,
    time::{Duration, Instant},
};

#[derive(Debug, PartialEq)]
pub enum InputMode {
    Explorer,
    Secrets,
    SelectingKey,
    SearchingKey,
    SearchingSecrets,
    Generating,
    Editing,
    Confirming,
    ConfirmingKeyDeletion,
    ConfirmingKeyCreation,
    Help,
    ViewingValue,
    AddingSecret,
    Settings,
    ManagingKeys,
    CreatingFolder,
    RenamingFile,
    ConfirmingFileDeletion,
    CreatingSecretFile,
    SelectingFileFormat,
    SelectingSopsKeys,
    EditingSopsConfig,
    SelectingSopsTemplate,
}

pub struct App {
    pub secrets: Vec<(String, String)>,
    pub encrypted_keys: Vec<String>,
    pub table_state: ratatui::widgets::TableState,
    pub age_keys: Vec<AgeKey>,
    pub selected_key_index: Option<usize>,
    pub input_mode: InputMode,
    pub key_list_state: ratatui::widgets::ListState,
    pub show_values: bool,
    pub file_path: Option<PathBuf>,
    pub current_dir: PathBuf,
    pub files: Vec<PathBuf>,
    pub file_list_state: ratatui::widgets::ListState,
    pub error_message: Option<String>,
    pub message_timestamp: Option<Instant>,
    pub clipboard_timestamp: Option<Instant>,
    pub file_recipients: Vec<String>,
    pub key_search_query: String,
    pub secret_search_query: String,
    pub clipboard: Option<Clipboard>,
    pub is_modified: bool,
    pub editing_key_buffer: String,
    pub editing_value_buffer: String,
    pub cursor_position: usize,
    pub editing_field: usize,
    pub gen_selected_option: usize,
    pub gen_length: usize,
    pub gen_use_special: bool,
    pub gen_use_numbers: bool,
    pub gen_token_format: TokenFormat,
    pub use_regex: bool,
    pub viewing_value: Option<String>,
    pub viewing_scroll: u16,
    pub config: Config,
    pub favorites: Vec<PathBuf>,
    pub marked_files: Vec<PathBuf>,
    pub i18n: I18n,
    pub last_activity: Instant,
    pub theme: Theme,
    pub settings_selected: usize,
    pub is_loading: bool,
    pub loading_message: String,
    pub key_manager_selected: usize,
    pub new_key_comment: String,
    pub folder_name_buffer: String,
    pub rename_buffer: String,
    pub new_file_name_buffer: String,
    pub edit_buffer: String,
    pub selected_sops_template: usize,
    pub selected_format: usize,
    pub selected_sops_keys: Vec<bool>,
}

impl App {
    pub fn new(start_dir: PathBuf, config: Config, age_keys: Vec<AgeKey>, favorites: Vec<PathBuf>) -> Result<Self> {
        let files = Self::list_files(&start_dir)?;
        let i18n = I18n::new(config.get_language());
        let theme = config.get_theme();
        let mut app = Self {
            secrets: Vec::new(),
            encrypted_keys: Vec::new(),
            table_state: ratatui::widgets::TableState::default(),
            age_keys,
            selected_key_index: None,
            input_mode: InputMode::Explorer,
            key_list_state: ratatui::widgets::ListState::default(),
            show_values: false,
            file_path: None,
            current_dir: start_dir,
            files,
            file_list_state: ratatui::widgets::ListState::default(),
            error_message: None,
            message_timestamp: None,
            clipboard_timestamp: None,
            file_recipients: Vec::new(),
            key_search_query: String::new(),
            secret_search_query: String::new(),
            clipboard: Clipboard::new().ok(),
            is_modified: false,
            editing_key_buffer: String::new(),
            editing_value_buffer: String::new(),
            cursor_position: 0,
            editing_field: 0,
            gen_length: 16,
            gen_use_special: true,
            gen_use_numbers: true,
            gen_token_format: TokenFormat::Hex,
            gen_selected_option: 0,
            use_regex: false,
            viewing_value: None,
            viewing_scroll: 0,
            config,
            favorites,
            marked_files: Vec::new(),
            i18n,
            last_activity: Instant::now(),
            theme,
            settings_selected: 0,
            is_loading: false,
            loading_message: String::new(),
            key_manager_selected: 0,
            new_key_comment: String::new(),
            folder_name_buffer: String::new(),
            rename_buffer: String::new(),
            new_file_name_buffer: String::new(),
            edit_buffer: String::new(),
            selected_sops_template: 0,
            selected_format: 0,
            selected_sops_keys: Vec::new(),
        };
        if !app.files.is_empty() {
            app.file_list_state.select(Some(0));
        }
        Ok(app)
    }

    pub fn list_files(dir: &PathBuf) -> Result<Vec<PathBuf>> {
        let mut files = Vec::new();
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                files.push(entry.path());
            }
        }
        files.sort();
        // Agregar ".." al principio después de ordenar
        files.insert(0, PathBuf::from(".."));
        Ok(files)
    }

    pub fn next_file(&mut self) {
        if self.files.is_empty() {
            return;
        }
        let i = match self.file_list_state.selected() {
            Some(i) => (i + 1) % self.files.len(),
            None => 0,
        };
        self.file_list_state.select(Some(i));
    }

    pub fn previous_file(&mut self) {
        if self.files.is_empty() {
            return;
        }
        let i = match self.file_list_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.files.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.file_list_state.select(Some(i));
    }

    pub fn copy_selected_value(&mut self) {
        if let Some(idx) = self.table_state.selected() {
            let filtered = self.filtered_secrets();
            if let Some(&real_idx) = filtered.get(idx) {
                if let Some((_, value)) = self.secrets.get(real_idx) {
                    if let Some(clipboard) = &mut self.clipboard {
                        if clipboard.set_text(value.clone()).is_ok() {
                            self.set_temp_message(self.i18n.t("copy_value").to_string());
                            self.clipboard_timestamp = Some(Instant::now());
                        }
                    }
                }
            }
        }
    }

    pub fn copy_selected_key(&mut self) {
        if let Some(idx) = self.table_state.selected() {
            let filtered = self.filtered_secrets();
            if let Some(&real_idx) = filtered.get(idx) {
                if let Some((key, _)) = self.secrets.get(real_idx) {
                    if let Some(clipboard) = &mut self.clipboard {
                        if clipboard.set_text(key.clone()).is_ok() {
                            self.set_temp_message(self.i18n.t("copy_key").to_string());
                            self.clipboard_timestamp = Some(Instant::now());
                        }
                    }
                }
            }
        }
    }

    pub fn set_temp_message(&mut self, msg: String) {
        self.error_message = Some(msg);
        self.message_timestamp = Some(Instant::now());
    }

    pub fn generate_and_copy(&mut self) {
        let result = match self.gen_selected_option {
            0 => crate::generator::generate_password(self.gen_length, self.gen_use_special, self.gen_use_numbers),
            1 => crate::generator::generate_token(self.gen_token_format, self.gen_length),
            _ => return,
        };
        
        // Poner el valor generado en el buffer de edición
        self.editing_value_buffer = result.clone();
        self.cursor_position = self.editing_value_buffer.len();
        
        if let Some(clipboard) = &mut self.clipboard {
            if clipboard.set_text(result.clone()).is_ok() {
                self.set_temp_message(format!("{}: {}", self.i18n.t("generated"), result));
                self.clipboard_timestamp = Some(Instant::now());
            }
        }
        
        // Volver al modo de edición/agregado
        if self.table_state.selected().is_some() {
            self.input_mode = InputMode::Editing;
        } else {
            self.input_mode = InputMode::AddingSecret;
        }
    }

    pub fn clear_expired_message(&mut self) {
        if let Some(timestamp) = self.message_timestamp {
            if timestamp.elapsed() > Duration::from_secs(self.config.message_timeout_seconds) {
                self.error_message = None;
                self.message_timestamp = None;
            }
        }
        
        if let Some(timestamp) = self.clipboard_timestamp {
            if timestamp.elapsed() > Duration::from_secs(self.config.message_timeout_seconds) {
                if let Some(clipboard) = &mut self.clipboard {
                    let _ = clipboard.clear();
                }
                self.clipboard_timestamp = None;
            }
        }
    }

    pub fn clear_message(&mut self) {
        self.error_message = None;
        self.message_timestamp = None;
    }

    pub fn update_activity(&mut self) {
        self.last_activity = Instant::now();
    }

    pub fn check_auto_lock(&self) -> bool {
        if self.config.auto_lock_minutes == 0 {
            return false;
        }
        let timeout = Duration::from_secs(self.config.auto_lock_minutes * 60);
        self.last_activity.elapsed() > timeout
    }

    pub fn lock(&mut self) {
        self.secrets.clear();
        self.show_values = false;
        self.file_path = None;
        self.input_mode = InputMode::Explorer;
    }

    pub fn save_config(&self) -> Result<()> {
        use crate::config::save_config;
        save_config(&self.config)
    }

    pub fn toggle_theme(&mut self) {
        self.config.theme = if self.config.theme == "dark" {
            "light".to_string()
        } else {
            "dark".to_string()
        };
        self.theme = self.config.get_theme();
        let _ = self.save_config();
    }

    pub fn toggle_language(&mut self) {
        self.config.language = if self.config.language == "en" {
            "es".to_string()
        } else {
            "en".to_string()
        };
        self.i18n = I18n::new(self.config.get_language());
        let _ = self.save_config();
    }

    pub fn auto_detect_key(&mut self) -> Option<usize> {
        for (i, age_key) in self.age_keys.iter().enumerate() {
            if let Some(pub_key) = &age_key.public_key {
                if self.file_recipients.contains(pub_key) {
                    return Some(i);
                }
            }
        }
        None
    }

    pub fn filtered_keys(&self) -> Vec<(usize, &AgeKey)> {
        if self.key_search_query.is_empty() {
            self.age_keys.iter().enumerate().collect()
        } else {
            let query = self.key_search_query.to_lowercase();
            self.age_keys
                .iter()
                .enumerate()
                .filter(|(_, k)| {
                    k.comment
                        .as_ref()
                        .map(|c| c.to_lowercase().contains(&query))
                        .unwrap_or(false)
                        || k.public_key
                            .as_ref()
                            .map(|p| p.to_lowercase().contains(&query))
                            .unwrap_or(false)
                })
                .collect()
        }
    }

    pub fn filtered_secrets(&self) -> Vec<usize> {
        if self.secret_search_query.is_empty() {
            (0..self.secrets.len()).collect()
        } else {
            let query = self.secret_search_query.to_lowercase();
            self.secrets
                .iter()
                .enumerate()
                .filter(|(_, (k, v))| {
                    if self.use_regex {
                        if let Ok(re) = Regex::new(&self.secret_search_query) {
                            re.is_match(k) || re.is_match(v)
                        } else {
                            false
                        }
                    } else {
                        k.to_lowercase().contains(&query) || v.to_lowercase().contains(&query)
                    }
                })
                .map(|(i, _)| i)
                .collect()
        }
    }

    pub fn is_encrypted(&self, key: &str) -> bool {
        // Comparar directamente o por la última parte después del último punto (para INI)
        self.encrypted_keys.iter().any(|k| {
            k == key || key.ends_with(&format!(".{}", k))
        })
    }

    pub fn open_value_viewer(&mut self) {
        if let Some(idx) = self.table_state.selected() {
            let filtered = self.filtered_secrets();
            if let Some(&real_idx) = filtered.get(idx) {
                if let Some((_, value)) = self.secrets.get(real_idx) {
                    self.viewing_value = Some(value.clone());
                    self.viewing_scroll = 0;
                    self.input_mode = InputMode::ViewingValue;
                }
            }
        }
    }

    pub fn format_json_value(&self, value: &str) -> String {
        if let Ok(json) = serde_json::from_str::<Value>(value) {
            serde_json::to_string_pretty(&json).unwrap_or_else(|_| value.to_string())
        } else {
            value.to_string()
        }
    }

    pub fn toggle_favorite(&mut self) {
        if let Some(path) = &self.file_path {
            if let Some(pos) = self.favorites.iter().position(|p| p == path) {
                self.favorites.remove(pos);
                self.set_temp_message(self.i18n.t("fav_removed").to_string());
            } else {
                self.favorites.push(path.clone());
                self.set_temp_message(self.i18n.t("fav_added").to_string());
            }
        }
    }

    pub fn init_sops_config(&mut self) -> Result<()> {
        let sops_file = self.current_dir.join(".sops.yaml");
        
        if sops_file.exists() {
            // Abrir con editor externo
            self.open_file_in_editor(&sops_file)?;
            self.files = Self::list_files(&self.current_dir)?;
            return Ok(());
        }

        if self.age_keys.is_empty() {
            self.input_mode = InputMode::ConfirmingKeyCreation;
            return Ok(());
        }

        // Mostrar selector de formato de archivo
        self.selected_format = 0;
        self.new_file_name_buffer.clear();
        self.input_mode = InputMode::SelectingFileFormat;
        
        Ok(())
    }

    pub fn open_file_in_editor(&self, file_path: &PathBuf) -> Result<()> {
        use std::process::Command;
        
        // Intentar editores en orden de preferencia
        let editors = ["nano", "vim", "vi"];
        
        for editor in &editors {
            if Command::new("which")
                .arg(editor)
                .output()
                .map(|o| o.status.success())
                .unwrap_or(false)
            {
                Command::new(editor)
                    .arg(file_path)
                    .status()?;
                return Ok(());
            }
        }
        
        anyhow::bail!("No se encontró ningún editor (nano, vim, vi)")
    }

    pub fn create_sops_config(&mut self) -> Result<()> {
        let sops_file = self.current_dir.join(".sops.yaml");
        
        let public_keys: Vec<String> = self.age_keys
            .iter()
            .enumerate()
            .filter(|(i, _)| self.selected_sops_keys.get(*i).copied().unwrap_or(false))
            .filter_map(|(_, k)| k.public_key.clone())
            .collect();

        if public_keys.is_empty() {
            self.set_temp_message(self.i18n.t("no_keys_selected").to_string());
            return Ok(());
        }

        let age_keys = public_keys.join(",\n        ");
        
        // Obtener nombre del archivo
        let formats = ["env", "json", "yaml", "ini"];
        let ext = formats[self.selected_format];
        
        let filename = if self.new_file_name_buffer.is_empty() {
            format!("secrets.{}", ext)
        } else if self.new_file_name_buffer.ends_with(&format!(".{}", ext)) {
            self.new_file_name_buffer.clone()
        } else {
            format!("{}.{}", self.new_file_name_buffer, ext)
        };

        // Crear regex que coincida con el archivo en cualquier ubicación
        let filename_regex = filename.replace(".", "\\.");

        let config_content = format!(
            "# SOPS configuration for {}\ncreation_rules:\n  - path_regex: (^|/){}$\n    encrypted_regex: '^(password|passwd|pass|secret|key|token|api[_-]?key|private[_-]?key|access[_-]?key|auth|credential|database[_-]?url|.*[Ss]ecret.*|.*[Kk]ey.*|.*[Tt]oken.*)$'\n    age: >\n      {}\n",
            filename, filename_regex, age_keys
        );

        fs::write(&sops_file, config_content)?;
        self.files = Self::list_files(&self.current_dir)?;
        self.set_temp_message(self.i18n.t("sops_initialized").to_string());
        
        Ok(())
    }

    pub fn save_sops_config(&mut self) -> Result<()> {
        let sops_file = self.current_dir.join(".sops.yaml");
        fs::write(&sops_file, &self.edit_buffer)?;
        self.files = Self::list_files(&self.current_dir)?;
        self.set_temp_message(self.i18n.t("sops_saved").to_string());
        self.input_mode = InputMode::Explorer;
        Ok(())
    }

    pub fn generate_age_key(&mut self) -> Result<()> {
        use std::process::Command;
        
        let output = Command::new("age-keygen")
            .output()
            .context("No se pudo ejecutar age-keygen. ¿Está instalado?")?;

        if !output.status.success() {
            anyhow::bail!("Error al generar llave age");
        }

        let key_content = String::from_utf8(output.stdout)?;
        
        // Agregar comentario si se proporcionó, preservando el formato
        let final_content = if !self.new_key_comment.is_empty() {
            // Insertar comentario después de "# created:"
            let lines: Vec<&str> = key_content.lines().collect();
            let mut result = String::new();
            for line in lines {
                result.push_str(line);
                result.push('\n');
                if line.starts_with("# created:") {
                    result.push_str(&format!("# {}\n", self.new_key_comment));
                }
            }
            result.trim_end().to_string()
        } else {
            key_content.trim_end().to_string()
        };

        // Agregar al archivo de llaves
        let keys_path = dirs::home_dir()
            .context("No se pudo obtener directorio home")?
            .join(".config/sops/age/keys.txt");

        if let Some(parent) = keys_path.parent() {
            fs::create_dir_all(parent)?;
        }

        use std::fs::OpenOptions;
        use std::io::Write;
        
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&keys_path)?;
        
        // Agregar separador si el archivo no está vacío
        let metadata = file.metadata()?;
        if metadata.len() > 0 {
            writeln!(file)?;
        }
        writeln!(file, "{}", final_content)?;

        // Recargar llaves
        self.age_keys = crate::sops::load_age_keys()?;
        self.new_key_comment.clear();
        self.set_temp_message(self.i18n.t("key_generated").to_string());
        
        Ok(())
    }

    pub fn delete_selected_age_key(&mut self) -> Result<()> {
        if self.age_keys.is_empty() {
            return Ok(());
        }

        let idx = self.key_manager_selected;
        if idx >= self.age_keys.len() {
            return Ok(());
        }

        // Leer el archivo original
        let keys_path = dirs::home_dir()
            .context("No se pudo obtener directorio home")?
            .join(".config/sops/age/keys.txt");

        let original_content = fs::read_to_string(&keys_path)?;
        
        // Dividir en bloques de llaves (separados por líneas vacías)
        let blocks: Vec<&str> = original_content.split("\n\n").collect();
        
        // Filtrar el bloque que contiene la llave a eliminar
        let key_to_remove = &self.age_keys[idx].key;
        let filtered_blocks: Vec<&str> = blocks
            .into_iter()
            .filter(|block| !block.contains(key_to_remove))
            .collect();
        
        // Reconstruir el archivo
        let new_content = filtered_blocks.join("\n\n").trim_end().to_string();
        fs::write(&keys_path, new_content)?;

        // Actualizar la lista en memoria
        self.age_keys.remove(idx);
        
        if self.key_manager_selected >= self.age_keys.len() && self.key_manager_selected > 0 {
            self.key_manager_selected -= 1;
        }

        self.set_temp_message(self.i18n.t("key_deleted").to_string());
        
        Ok(())
    }

    pub fn create_folder(&mut self) -> Result<()> {
        if self.folder_name_buffer.is_empty() {
            return Ok(());
        }

        let new_folder = self.current_dir.join(&self.folder_name_buffer);
        fs::create_dir_all(&new_folder)?;
        
        self.files = Self::list_files(&self.current_dir)?;
        self.folder_name_buffer.clear();
        self.set_temp_message(self.i18n.t("folder_created").to_string());
        
        Ok(())
    }

    pub fn rename_selected_file(&mut self) -> Result<()> {
        if self.rename_buffer.is_empty() {
            return Ok(());
        }

        if let Some(idx) = self.file_list_state.selected() {
            if let Some(path) = self.files.get(idx) {
                if path.to_str() == Some("..") {
                    return Ok(());
                }

                let new_path = path.parent()
                    .unwrap_or(&self.current_dir)
                    .join(&self.rename_buffer);
                
                fs::rename(path, &new_path)?;
                
                self.files = Self::list_files(&self.current_dir)?;
                self.rename_buffer.clear();
                self.set_temp_message(self.i18n.t("file_renamed").to_string());
            }
        }
        
        Ok(())
    }

    pub fn create_encrypted_file(&mut self) -> Result<()> {
        let formats = ["env", "json", "yaml", "ini"];
        let ext = formats[self.selected_format];
        
        // Si el buffer está vacío, usar "secrets" como nombre por defecto
        let filename = if self.new_file_name_buffer.is_empty() {
            format!("secrets.{}", ext)
        } else if self.new_file_name_buffer.starts_with('.') {
            // Si ya empieza con punto, no agregar extensión
            self.new_file_name_buffer.clone()
        } else if self.new_file_name_buffer.ends_with(&format!(".{}", ext)) {
            self.new_file_name_buffer.clone()
        } else {
            format!("{}.{}", self.new_file_name_buffer, ext)
        };
        
        let file_path = self.current_dir.join(&filename);
        
        if file_path.exists() {
            self.set_temp_message(format!("❌ {}", self.i18n.t("file_exists")));
            return Ok(());
        }

        // Crear plantilla según formato
        let template = match ext {
            "ini" => r#"{"DEFAULT": {"example_key": "example_value"}}"#,
            _ => r#"{"example_key": "example_value"}"#,
        };

        fs::write(&file_path, template)?;

        let mut cmd = Command::new("sops");
        cmd.arg("-e")
           .arg("-i");
        
        // Usar JSON como entrada y convertir al formato deseado
        match ext {
            "yaml" => {
                cmd.arg("--input-type").arg("json")
                   .arg("--output-type").arg("yaml");
            },
            "json" => {
                cmd.arg("--input-type").arg("json")
                   .arg("--output-type").arg("json");
            },
            "env" => {
                cmd.arg("--input-type").arg("json")
                   .arg("--output-type").arg("dotenv");
            },
            "ini" => {
                cmd.arg("--input-type").arg("json")
                   .arg("--output-type").arg("ini");
            },
            _ => {},
        };
        
        cmd.current_dir(&self.current_dir)
           .arg(&filename);  // Usar solo el nombre del archivo, no la ruta completa

        // Asegurar que SOPS encuentre las llaves age
        let age_key_file = std::env::var("SOPS_AGE_KEY_FILE")
            .unwrap_or_else(|_| {
                let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
                format!("{}/.config/sops/age/keys.txt", home)
            });
        cmd.env("SOPS_AGE_KEY_FILE", age_key_file);

        let output = cmd.output()?;

        if !output.status.success() {
            fs::remove_file(&file_path)?;
            let error = String::from_utf8_lossy(&output.stderr);
            self.set_temp_message(format!("❌ SOPS error: {}", error));
            return Ok(());
        }

        self.files = Self::list_files(&self.current_dir)?;
        self.new_file_name_buffer.clear();
        self.set_temp_message(format!("✓ {}", self.i18n.t("file_created")));
        
        Ok(())
    }

    pub fn delete_selected_file(&mut self) -> Result<()> {
        if let Some(idx) = self.file_list_state.selected() {
            if let Some(path) = self.files.get(idx).cloned() {
                if path.to_str() == Some("..") {
                    return Ok(());
                }

                if path.is_dir() {
                    fs::remove_dir_all(&path)?;
                } else {
                    fs::remove_file(&path)?;
                }
                
                self.files = Self::list_files(&self.current_dir)?;
                if self.file_list_state.selected().unwrap_or(0) >= self.files.len() && !self.files.is_empty() {
                    self.file_list_state.select(Some(self.files.len() - 1));
                }
                self.set_temp_message(self.i18n.t("file_deleted").to_string());
            }
        }
        
        Ok(())
    }

    pub fn toggle_mark_file(&mut self) {
        if let Some(idx) = self.file_list_state.selected() {
            if let Some(path) = self.files.get(idx) {
                if path.is_file() {
                    if let Some(pos) = self.marked_files.iter().position(|p| p == path) {
                        self.marked_files.remove(pos);
                    } else {
                        self.marked_files.push(path.clone());
                    }
                }
            }
        }
    }

    pub fn open_selected(&mut self) -> Result<()> {
        if let Some(idx) = self.file_list_state.selected() {
            if let Some(path) = self.files.get(idx).cloned() {
                if path.to_str() == Some("..") {
                    if let Some(parent) = self.current_dir.parent() {
                        self.current_dir = parent.to_path_buf();
                        self.files = Self::list_files(&self.current_dir)?;
                        self.file_list_state.select(Some(0));
                    }
                } else if path.is_dir() {
                    self.current_dir = path;
                    self.files = Self::list_files(&self.current_dir)?;
                    self.file_list_state.select(Some(0));
                } else {
                    self.file_path = Some(path.clone());
                    self.file_recipients = get_sops_recipients(&path).unwrap_or_default();
                    self.encrypted_keys = get_encrypted_keys(&path).unwrap_or_default();

                    if self.selected_key_index.is_none() {
                        self.selected_key_index = self.auto_detect_key();
                    }

                    let key = self.selected_key_index.and_then(|i| self.age_keys.get(i).map(|k| k.key.as_str()));
                    match decrypt_and_parse(&path, key) {
                        Ok(secrets) => {
                            self.secrets = secrets;
                            self.input_mode = InputMode::Secrets;
                            if !self.secrets.is_empty() {
                                self.table_state.select(Some(0));
                            }
                        }
                        Err(_) => {
                            let matching_keys: Vec<String> = self.age_keys
                                .iter()
                                .enumerate()
                                .filter_map(|(i, k)| {
                                    if let Some(pub_key) = &k.public_key {
                                        if self.file_recipients.contains(pub_key) {
                                            return Some(format!("#{} {}", i + 1, k.comment.as_ref().unwrap_or(&self.i18n.t("unnamed").to_string())));
                                        }
                                    }
                                    None
                                })
                                .collect();

                            let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("?");
                            let msg = if matching_keys.is_empty() {
                                format!("{} {}: {}. {}: {}", self.i18n.t("error_no_key_match"), file_name, self.i18n.t("error_no_key_match"), self.i18n.t("recipients"), self.file_recipients.join(", "))
                            } else {
                                format!("✓ {}: {}: {}. {}", file_name, self.i18n.t("available_keys"), matching_keys.join(", "), self.i18n.t("press_k"))
                            };
                            self.error_message = Some(msg);
                        }
                    }
                }
            }
        }
        Ok(())
    }

    pub fn next_secret(&mut self) {
        if self.secrets.is_empty() {
            return;
        }
        let i = match self.table_state.selected() {
            Some(i) => (i + 1) % self.secrets.len(),
            None => 0,
        };
        self.table_state.select(Some(i));
    }

    pub fn previous_secret(&mut self) {
        if self.secrets.is_empty() {
            return;
        }
        let i = match self.table_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.secrets.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.table_state.select(Some(i));
    }

    pub fn next_key(&mut self) {
        if self.age_keys.is_empty() {
            return;
        }
        let i = match self.key_list_state.selected() {
            Some(i) => (i + 1) % self.age_keys.len(),
            None => 0,
        };
        self.key_list_state.select(Some(i));
    }

    pub fn previous_key(&mut self) {
        if self.age_keys.is_empty() {
            return;
        }
        let i = match self.key_list_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.age_keys.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.key_list_state.select(Some(i));
    }

    pub fn edit_secret(&mut self) {
        if let Some(idx) = self.table_state.selected() {
            let filtered = self.filtered_secrets();
            if let Some(&real_idx) = filtered.get(idx) {
                if let Some((key, value)) = self.secrets.get(real_idx) {
                    self.editing_key_buffer = key.clone();
                    self.editing_value_buffer = value.clone();
                    self.editing_field = 0;
                    self.cursor_position = self.editing_key_buffer.len();
                    self.input_mode = InputMode::Editing;
                }
            }
        }
    }

    pub fn add_secret(&mut self) {
        self.editing_key_buffer.clear();
        self.editing_value_buffer.clear();
        self.editing_field = 0;
        self.cursor_position = 0;
        self.input_mode = InputMode::AddingSecret;
    }

    pub fn delete_secret(&mut self) {
        if let Some(idx) = self.table_state.selected() {
            let filtered = self.filtered_secrets();
            if let Some(&real_idx) = filtered.get(idx) {
                if self.secrets.get(real_idx).is_some() {
                    self.input_mode = InputMode::Confirming;
                }
            }
        }
    }

    pub fn confirm_delete(&mut self) {
        if let Some(idx) = self.table_state.selected() {
            let filtered = self.filtered_secrets();
            if let Some(&real_idx) = filtered.get(idx) {
                self.secrets.remove(real_idx);
                self.is_modified = true;
                if self.table_state.selected().unwrap_or(0) >= self.secrets.len() && !self.secrets.is_empty() {
                    self.table_state.select(Some(self.secrets.len() - 1));
                }
                self.set_temp_message(self.i18n.t("deleted").to_string());
            }
        }
        self.input_mode = InputMode::Secrets;
    }

    pub fn save_changes(&mut self) -> Result<()> {
        if let Some(file_path) = &self.file_path {
            let backup_file = file_path.with_extension("bak");
            fs::copy(file_path, &backup_file)?;

            let ext = file_path.extension().and_then(|s| s.to_str());
            let file_name = file_path.file_name().and_then(|s| s.to_str()).unwrap_or("");
            
            // Detectar formato por nombre de archivo o extensión
            let format = if file_name == ".env" || ext == Some("env") {
                "env"
            } else if file_name == ".ini" || ext == Some("ini") {
                "ini"
            } else if ext == Some("yaml") || ext == Some("yml") {
                "yaml"
            } else if ext == Some("json") {
                "json"
            } else {
                "json" // default
            };
            
            // Para ENV e INI, convertir a JSON para SOPS y luego convertir de vuelta
            let content = if format == "env" {
                // Crear JSON para que SOPS lo maneje correctamente
                let mut json_obj = serde_json::Map::new();
                for (k, v) in &self.secrets {
                    json_obj.insert(k.clone(), Value::String(v.clone()));
                }
                let json_value = Value::Object(json_obj);
                serde_json::to_string_pretty(&json_value)?
            } else if format == "ini" {
                // Para INI, crear estructura con sección DEFAULT
                let mut json_obj = serde_json::Map::new();
                let mut default_section = serde_json::Map::new();
                for (k, v) in &self.secrets {
                    let clean_key = k.split('.').last().unwrap_or(k);
                    default_section.insert(clean_key.to_string(), Value::String(v.clone()));
                }
                json_obj.insert("DEFAULT".to_string(), Value::Object(default_section));
                let json_value = Value::Object(json_obj);
                serde_json::to_string_pretty(&json_value)?
            } else if format == "yaml" {
                // Para YAML/JSON, generar JSON estructurado
                let mut json_obj = serde_json::Map::new();

                for (key, value) in &self.secrets {
                    let keys: Vec<&str> = key.split('.').collect();
                    let mut current_map = &mut json_obj;
                    for (i, k) in keys.iter().enumerate() {
                        if i == keys.len() - 1 {
                            // Si el valor está entre comillas, forzar como string
                            let json_value = if value.len() >= 2 && 
                                              ((value.starts_with('"') && value.ends_with('"')) || 
                                               (value.starts_with('\'') && value.ends_with('\''))) {
                                // Quitar las comillas y guardar como string
                                let unquoted = &value[1..value.len()-1];
                                Value::String(unquoted.to_string())
                            } else if value == "true" || value == "false" {
                                Value::Bool(value == "true")
                            } else if let Ok(num) = value.parse::<i64>() {
                                Value::Number(num.into())
                            } else if let Ok(num) = value.parse::<f64>() {
                                serde_json::Number::from_f64(num)
                                    .map(Value::Number)
                                    .unwrap_or_else(|| Value::String(value.clone()))
                            } else if value == "null" {
                                Value::Null
                            } else {
                                Value::String(value.clone())
                            };
                            current_map.insert(k.to_string(), json_value);
                        } else {
                            if !current_map.contains_key(*k) {
                                current_map.insert(k.to_string(), Value::Object(serde_json::Map::new()));
                            }
                            let next_map = current_map.get_mut(*k).and_then(|v| v.as_object_mut())
                                .context("Expected object in nested structure")?;
                            current_map = next_map;
                        }
                    }
                }

                let json_value = Value::Object(json_obj);
                serde_json::to_string_pretty(&json_value)?
            } else {
                // Para JSON, generar JSON estructurado con detección de tipos
                let mut json_obj = serde_json::Map::new();

                for (key, value) in &self.secrets {
                    let keys: Vec<&str> = key.split('.').collect();
                    let mut current_map = &mut json_obj;
                    for (i, k) in keys.iter().enumerate() {
                        if i == keys.len() - 1 {
                            // Si el valor está entre comillas, forzar como string
                            let json_value = if value.len() >= 2 && 
                                              ((value.starts_with('"') && value.ends_with('"')) || 
                                               (value.starts_with('\'') && value.ends_with('\''))) {
                                // Quitar las comillas y guardar como string
                                let unquoted = &value[1..value.len()-1];
                                Value::String(unquoted.to_string())
                            } else if value == "true" || value == "false" {
                                Value::Bool(value == "true")
                            } else if let Ok(num) = value.parse::<i64>() {
                                Value::Number(num.into())
                            } else if let Ok(num) = value.parse::<f64>() {
                                serde_json::Number::from_f64(num)
                                    .map(Value::Number)
                                    .unwrap_or_else(|| Value::String(value.clone()))
                            } else if value == "null" {
                                Value::Null
                            } else {
                                Value::String(value.clone())
                            };
                            current_map.insert(k.to_string(), json_value);
                        } else {
                            if !current_map.contains_key(*k) {
                                current_map.insert(k.to_string(), Value::Object(serde_json::Map::new()));
                            }
                            let next_map = current_map.get_mut(*k).and_then(|v| v.as_object_mut())
                                .context("Expected object in nested structure")?;
                            current_map = next_map;
                        }
                    }
                }

                let json_value = Value::Object(json_obj);
                serde_json::to_string_pretty(&json_value)?
            };

            fs::write(file_path, content)?;

            let key = self.selected_key_index.and_then(|i| self.age_keys.get(i).map(|k| k.key.as_str()));
            let mut sops_dir = file_path.parent();
            while let Some(dir) = sops_dir {
                if dir.join(".sops.yaml").exists() {
                    break;
                }
                sops_dir = dir.parent();
            }

            let work_dir = sops_dir.unwrap_or_else(|| file_path.parent().unwrap_or(Path::new(".")));

            let mut cmd = Command::new("sops");
            cmd.arg("--encrypt")
               .arg("-i");  // In-place para mantener recipients
            
            // Para ENV e INI, usar JSON como formato intermedio
            match format {
                "yaml" => {
                    cmd.arg("--input-type").arg("json")
                       .arg("--output-type").arg("yaml");
                },
                "env" => {
                    cmd.arg("--input-type").arg("json")
                       .arg("--output-type").arg("dotenv");
                },
                "ini" => {
                    cmd.arg("--input-type").arg("json")
                       .arg("--output-type").arg("ini");
                },
                _ => {
                    cmd.arg("--input-type").arg("json")
                       .arg("--output-type").arg("json");
                },
            };
            
            cmd.arg(file_path)
               .current_dir(work_dir);

            if let Some(k) = key {
                cmd.env("SOPS_AGE_KEY", k);
            } else {
                // Asegurar que SOPS encuentre las llaves age
                let age_key_file = std::env::var("SOPS_AGE_KEY_FILE")
                    .unwrap_or_else(|_| {
                        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
                        format!("{}/.config/sops/age/keys.txt", home)
                    });
                cmd.env("SOPS_AGE_KEY_FILE", age_key_file);
            }

            let output = cmd.output()?;

            if output.status.success() {
                // Con -i, SOPS modifica el archivo directamente
                fs::remove_file(&backup_file).ok();

                if let Some(parent) = file_path.parent() {
                    if let Ok(entries) = fs::read_dir(parent) {
                        for entry in entries.flatten() {
                            let path = entry.path();
                            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                                if name.contains(".tmp") && name.contains(file_path.file_name().unwrap().to_str().unwrap()) {
                                    fs::remove_file(&path).ok();
                                }
                            }
                        }
                    }
                }

                self.is_modified = false;
                self.set_temp_message(self.i18n.t("saved").to_string());
                
                // Recargar el archivo para actualizar encrypted_keys
                if let Some(file_path) = &self.file_path.clone() {
                    self.encrypted_keys = get_encrypted_keys(file_path).unwrap_or_default();
                }
            } else {
                fs::copy(&backup_file, file_path)?;
                fs::remove_file(&backup_file).ok();
                anyhow::bail!("{}: {}", self.i18n.t("error_encrypt"), String::from_utf8_lossy(&output.stderr));
            }
        }
        Ok(())
    }
}
