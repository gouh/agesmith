use crate::generator::TokenFormat;
use crate::sops::decrypt_and_parse;
use crate::state::{App, InputMode};
use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use regex::Regex;

pub fn handle_key_event(app: &mut App, key: KeyEvent) -> Result<bool> {
    if app.message_timestamp.is_some() {
        app.clear_message();
    }

    // Update activity on any key press
    app.update_activity();

    match app.input_mode {
        InputMode::Explorer => handle_explorer_keys(app, key),
        InputMode::Secrets => handle_secrets_keys(app, key),
        InputMode::SelectingKey => handle_selecting_key_keys(app, key),
        InputMode::SearchingKey => handle_searching_key_keys(app, key),
        InputMode::SearchingSecrets => handle_searching_secrets_keys(app, key),
        InputMode::ViewingValue => handle_viewing_value_keys(app, key),
        InputMode::Editing => handle_editing_keys(app, key),
        InputMode::AddingSecret => handle_adding_secret_keys(app, key),
        InputMode::Confirming => handle_confirming_keys(app, key),
        InputMode::Generating => handle_generating_keys(app, key),
        InputMode::Help => handle_help_keys(app, key),
        InputMode::Settings => handle_settings_keys(app, key),
        InputMode::ManagingKeys => handle_managing_keys_keys(app, key),
        InputMode::ConfirmingKeyDeletion => handle_confirming_key_deletion_keys(app, key),
        InputMode::ConfirmingKeyCreation => handle_confirming_key_creation_keys(app, key),
        InputMode::CreatingFolder => handle_creating_folder_keys(app, key),
        InputMode::RenamingFile => handle_renaming_file_keys(app, key),
        InputMode::ConfirmingFileDeletion => handle_confirming_file_deletion_keys(app, key),
        InputMode::CreatingSecretFile => handle_creating_secret_file_keys(app, key),
        InputMode::SelectingFileFormat => handle_selecting_file_format_keys(app, key),
        InputMode::SelectingSopsKeys => handle_selecting_sops_keys_keys(app, key),
        InputMode::EditingSopsConfig => handle_editing_sops_config_keys(app, key),
        InputMode::SelectingSopsTemplate => handle_selecting_sops_template_keys(app, key),
    }
}

fn handle_explorer_keys(app: &mut App, key: KeyEvent) -> Result<bool> {
    match key.code {
        KeyCode::Char('q') => return Ok(true),
        KeyCode::Char('?') => app.input_mode = InputMode::Help,
        KeyCode::Char('s') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.input_mode = InputMode::Settings;
        }
        KeyCode::Char('i') => {
            if let Err(e) = app.init_sops_config() {
                app.set_temp_message(format!("❌ Error: {}", e));
            }
        }
        KeyCode::Char('K') => {
            app.input_mode = InputMode::ManagingKeys;
            app.key_manager_selected = 0;
        }
        KeyCode::Char('N') => {
            app.input_mode = InputMode::CreatingFolder;
            app.folder_name_buffer.clear();
        }
        KeyCode::Char('n') => {
            app.input_mode = InputMode::SelectingFileFormat;
            app.selected_format = 0;
        }
        KeyCode::Char('r') => {
            if let Some(idx) = app.file_list_state.selected() {
                if let Some(path) = app.files.get(idx) {
                    if path.to_str() != Some("..") {
                        app.input_mode = InputMode::RenamingFile;
                        app.rename_buffer = path
                            .file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or("")
                            .to_string();
                    }
                }
            }
        }
        KeyCode::Char('D') => {
            if let Some(idx) = app.file_list_state.selected() {
                if let Some(path) = app.files.get(idx) {
                    if path.to_str() != Some("..") {
                        app.input_mode = InputMode::ConfirmingFileDeletion;
                    }
                }
            }
        }
        KeyCode::Char('m') => app.toggle_mark_file(),
        KeyCode::Down => app.next_file(),
        KeyCode::Up => app.previous_file(),
        KeyCode::Enter => app.open_selected()?,
        KeyCode::Char('k') => {
            if !app.age_keys.is_empty() {
                app.input_mode = InputMode::SelectingKey;
                app.key_list_state
                    .select(Some(app.selected_key_index.unwrap_or(0)));
            }
        }
        KeyCode::Tab => {
            if !app.secrets.is_empty() {
                app.input_mode = InputMode::Secrets;
            }
        }
        _ => {}
    }
    Ok(false)
}

fn handle_secrets_keys(app: &mut App, key: KeyEvent) -> Result<bool> {
    match key.code {
        KeyCode::Char('q') => return Ok(true),
        KeyCode::Char('?') => app.input_mode = InputMode::Help,
        KeyCode::Char('s') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.input_mode = InputMode::Settings;
        }
        KeyCode::Char('f') => app.toggle_favorite(),
        KeyCode::Char('v') => app.show_values = !app.show_values,
        KeyCode::Char('c') => app.copy_selected_value(),
        KeyCode::Char('C') => app.copy_selected_key(),
        KeyCode::Char('z') => app.open_value_viewer(),
        KeyCode::Char('e') => app.edit_secret(),
        KeyCode::Char('n') => app.add_secret(),
        KeyCode::Char('d') => app.delete_secret(),
        KeyCode::Char('s') => {
            if app.is_modified {
                if let Err(e) = app.save_changes() {
                    app.set_temp_message(format!("{}: {}", app.i18n.t("error_save"), e));
                }
            }
        }
        KeyCode::Char('/') => app.input_mode = InputMode::SearchingSecrets,
        KeyCode::Char('k') => {
            if !app.age_keys.is_empty() {
                app.input_mode = InputMode::SelectingKey;
                app.key_list_state
                    .select(Some(app.selected_key_index.unwrap_or(0)));
            }
        }
        KeyCode::Down => app.next_secret(),
        KeyCode::Up => app.previous_secret(),
        KeyCode::Tab => app.input_mode = InputMode::Explorer,
        _ => {}
    }
    Ok(false)
}

fn handle_selecting_key_keys(app: &mut App, key: KeyEvent) -> Result<bool> {
    match key.code {
        KeyCode::Char('?') => app.input_mode = InputMode::Help,
        KeyCode::Char('/') => app.input_mode = InputMode::SearchingKey,
        KeyCode::Esc => {
            app.key_search_query.clear();
            app.input_mode = if app.secrets.is_empty() {
                InputMode::Explorer
            } else {
                InputMode::Secrets
            };
        }
        KeyCode::Enter => {
            let filtered = app.filtered_keys();
            if let Some(selected) = app.key_list_state.selected() {
                if let Some((original_idx, _)) = filtered.get(selected) {
                    let original_idx = *original_idx;
                    let key = app.age_keys[original_idx].key.clone();
                    app.selected_key_index = Some(original_idx);

                    if let Some(file_path) = &app.file_path {
                        match decrypt_and_parse(file_path, Some(&key)) {
                            Ok(secrets) => {
                                app.secrets = secrets;
                                app.error_message = None;
                                if !app.secrets.is_empty() {
                                    app.table_state.select(Some(0));
                                }
                                app.input_mode = InputMode::Secrets;
                            }
                            Err(_) => {
                                app.error_message =
                                    Some(app.i18n.t("error_decrypt_key").to_string());
                                app.input_mode = InputMode::Explorer;
                            }
                        }
                    } else {
                        app.input_mode = InputMode::Explorer;
                    }
                }
            }
            app.key_search_query.clear();
        }
        KeyCode::Down => app.next_key(),
        KeyCode::Up => app.previous_key(),
        _ => {}
    }
    Ok(false)
}

fn handle_searching_key_keys(app: &mut App, key: KeyEvent) -> Result<bool> {
    match key.code {
        KeyCode::Esc => app.input_mode = InputMode::SelectingKey,
        KeyCode::Enter => {
            app.input_mode = InputMode::SelectingKey;
            app.key_list_state.select(Some(0));
        }
        KeyCode::Backspace => {
            app.key_search_query.pop();
        }
        KeyCode::Char(c) => {
            app.key_search_query.push(c);
        }
        _ => {}
    }
    Ok(false)
}

fn handle_searching_secrets_keys(app: &mut App, key: KeyEvent) -> Result<bool> {
    match key.code {
        KeyCode::Esc => app.input_mode = InputMode::Secrets,
        KeyCode::Enter => {
            app.input_mode = InputMode::Secrets;
            app.table_state.select(Some(0));
        }
        KeyCode::Char('r') => {
            app.use_regex = !app.use_regex;
            if app.use_regex
                && !app.secret_search_query.is_empty()
                && Regex::new(&app.secret_search_query).is_err()
            {
                app.set_temp_message(app.i18n.t("error_regex").to_string());
            }
        }
        KeyCode::Backspace => {
            app.secret_search_query.pop();
        }
        KeyCode::Char(c) => {
            app.secret_search_query.push(c);
        }
        _ => {}
    }
    Ok(false)
}

fn handle_viewing_value_keys(app: &mut App, key: KeyEvent) -> Result<bool> {
    match key.code {
        KeyCode::Esc | KeyCode::Char('z') => {
            app.viewing_value = None;
            app.viewing_scroll = 0;
            app.input_mode = InputMode::Secrets;
        }
        KeyCode::Down => {
            if let Some(value) = &app.viewing_value {
                let lines = value.lines().count();
                if (app.viewing_scroll as usize) < lines.saturating_sub(10) {
                    app.viewing_scroll += 1;
                }
            }
        }
        KeyCode::Up => {
            if app.viewing_scroll > 0 {
                app.viewing_scroll -= 1;
            }
        }
        KeyCode::Char('j') => {
            if let Some(value) = &app.viewing_value {
                app.viewing_value = Some(app.format_json_value(value));
            }
        }
        _ => {}
    }
    Ok(false)
}

fn handle_editing_keys(app: &mut App, key: KeyEvent) -> Result<bool> {
    match key.code {
        KeyCode::Esc => {
            app.editing_key_buffer.clear();
            app.editing_value_buffer.clear();
            app.cursor_position = 0;
            app.editing_field = 0;
            app.input_mode = InputMode::Secrets;
        }
        KeyCode::Tab => {
            app.editing_field = if app.editing_field == 0 { 1 } else { 0 };
            app.cursor_position = if app.editing_field == 0 {
                app.editing_key_buffer.len()
            } else {
                app.editing_value_buffer.len()
            };
        }
        KeyCode::Char('g') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            // Solo si estamos editando el valor (field 1)
            if app.editing_field == 1 {
                // Generar directamente sin abrir modal
                let result = crate::generator::generate_password(16, true, true);
                app.editing_value_buffer = result.clone();
                app.cursor_position = app.editing_value_buffer.len();

                if let Some(clipboard) = &mut app.clipboard {
                    let _ = clipboard.set_text(result.clone());
                }
                app.set_temp_message("✓ Contraseña generada y copiada".to_string());
            }
        }
        KeyCode::Enter => {
            if let Some(idx) = app.table_state.selected() {
                let filtered = app.filtered_secrets();
                if let Some(&real_idx) = filtered.get(idx) {
                    if !app.editing_key_buffer.is_empty() {
                        app.secrets[real_idx].0 = app.editing_key_buffer.clone();
                        app.secrets[real_idx].1 = app.editing_value_buffer.clone();
                        app.is_modified = true;
                        app.set_temp_message(app.i18n.t("updated").to_string());
                        app.editing_key_buffer.clear();
                        app.editing_value_buffer.clear();
                        app.cursor_position = 0;
                        app.editing_field = 0;
                        app.input_mode = InputMode::Secrets;
                    } else {
                        app.set_temp_message(app.i18n.t("error_empty_key").to_string());
                    }
                }
            }
        }
        _ => handle_text_input(app, key),
    }
    Ok(false)
}

fn handle_adding_secret_keys(app: &mut App, key: KeyEvent) -> Result<bool> {
    match key.code {
        KeyCode::Esc => {
            app.editing_key_buffer.clear();
            app.editing_value_buffer.clear();
            app.cursor_position = 0;
            app.editing_field = 0;
            app.input_mode = InputMode::Secrets;
        }
        KeyCode::Tab => {
            app.editing_field = if app.editing_field == 0 { 1 } else { 0 };
            app.cursor_position = if app.editing_field == 0 {
                app.editing_key_buffer.len()
            } else {
                app.editing_value_buffer.len()
            };
        }
        KeyCode::Char('g') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            // Solo si estamos editando el valor (field 1)
            if app.editing_field == 1 {
                // Generar directamente sin abrir modal
                let result = crate::generator::generate_password(16, true, true);
                app.editing_value_buffer = result.clone();
                app.cursor_position = app.editing_value_buffer.len();

                if let Some(clipboard) = &mut app.clipboard {
                    let _ = clipboard.set_text(result.clone());
                }
                app.set_temp_message("✓ Contraseña generada y copiada".to_string());
            }
        }
        KeyCode::Enter => {
            if !app.editing_key_buffer.is_empty() {
                app.secrets.push((
                    app.editing_key_buffer.clone(),
                    app.editing_value_buffer.clone(),
                ));
                app.is_modified = true;
                app.table_state.select(Some(app.secrets.len() - 1));
                app.set_temp_message(app.i18n.t("added").to_string());
                app.editing_key_buffer.clear();
                app.editing_value_buffer.clear();
                app.cursor_position = 0;
                app.editing_field = 0;
                // Ocultar valores automáticamente después de agregar
                app.show_values = false;
                app.input_mode = InputMode::Secrets;
            } else {
                app.set_temp_message(app.i18n.t("error_empty_key").to_string());
            }
        }
        _ => handle_text_input(app, key),
    }
    Ok(false)
}

fn handle_text_input(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Backspace => {
            let buffer = if app.editing_field == 0 {
                &mut app.editing_key_buffer
            } else {
                &mut app.editing_value_buffer
            };
            if app.cursor_position > 0 {
                buffer.remove(app.cursor_position - 1);
                app.cursor_position -= 1;
            }
        }
        KeyCode::Delete => {
            let buffer = if app.editing_field == 0 {
                &mut app.editing_key_buffer
            } else {
                &mut app.editing_value_buffer
            };
            if app.cursor_position < buffer.len() {
                buffer.remove(app.cursor_position);
            }
        }
        KeyCode::Left => {
            if app.cursor_position > 0 {
                app.cursor_position -= 1;
            }
        }
        KeyCode::Right => {
            let buffer_len = if app.editing_field == 0 {
                app.editing_key_buffer.len()
            } else {
                app.editing_value_buffer.len()
            };
            if app.cursor_position < buffer_len {
                app.cursor_position += 1;
            }
        }
        KeyCode::Home => {
            app.cursor_position = 0;
        }
        KeyCode::End => {
            app.cursor_position = if app.editing_field == 0 {
                app.editing_key_buffer.len()
            } else {
                app.editing_value_buffer.len()
            };
        }
        KeyCode::Char(c) => {
            let buffer = if app.editing_field == 0 {
                &mut app.editing_key_buffer
            } else {
                &mut app.editing_value_buffer
            };
            buffer.insert(app.cursor_position, c);
            app.cursor_position += 1;
        }
        _ => {}
    }
}

fn handle_confirming_keys(app: &mut App, key: KeyEvent) -> Result<bool> {
    match key.code {
        KeyCode::Char('y') => app.confirm_delete(),
        KeyCode::Char('n') | KeyCode::Esc => {
            app.input_mode = InputMode::Secrets;
        }
        _ => {}
    }
    Ok(false)
}

fn handle_generating_keys(app: &mut App, key: KeyEvent) -> Result<bool> {
    match key.code {
        KeyCode::Esc => app.input_mode = InputMode::Secrets,
        KeyCode::Enter => app.generate_and_copy(),
        KeyCode::Up => {
            if app.gen_selected_option > 0 {
                app.gen_selected_option -= 1;
            }
        }
        KeyCode::Down => {
            if app.gen_selected_option < 1 {
                app.gen_selected_option += 1;
            }
        }
        KeyCode::Left => match app.gen_selected_option {
            0 => {
                if app.gen_length > 8 {
                    app.gen_length -= 1;
                }
            }
            1 => {
                app.gen_token_format = match app.gen_token_format {
                    TokenFormat::Base64 => TokenFormat::Hex,
                    TokenFormat::Uuid => TokenFormat::Base64,
                    TokenFormat::Hex => TokenFormat::Uuid,
                };
            }
            _ => {}
        },
        KeyCode::Right => match app.gen_selected_option {
            0 => {
                if app.gen_length < 128 {
                    app.gen_length += 1;
                }
            }
            1 => {
                app.gen_token_format = match app.gen_token_format {
                    TokenFormat::Hex => TokenFormat::Base64,
                    TokenFormat::Base64 => TokenFormat::Uuid,
                    TokenFormat::Uuid => TokenFormat::Hex,
                };
            }
            _ => {}
        },
        KeyCode::Char('s') => app.gen_use_special = !app.gen_use_special,
        KeyCode::Char('n') => app.gen_use_numbers = !app.gen_use_numbers,
        _ => {}
    }
    Ok(false)
}

fn handle_help_keys(app: &mut App, key: KeyEvent) -> Result<bool> {
    match key.code {
        KeyCode::Char('?') | KeyCode::Esc => {
            app.input_mode = if app.secrets.is_empty() {
                InputMode::Explorer
            } else {
                InputMode::Secrets
            };
        }
        _ => {}
    }
    Ok(false)
}

fn handle_settings_keys(app: &mut App, key: KeyEvent) -> Result<bool> {
    match key.code {
        KeyCode::Esc => {
            app.input_mode = if app.secrets.is_empty() {
                InputMode::Explorer
            } else {
                InputMode::Secrets
            };
        }
        KeyCode::Up => {
            if app.settings_selected > 0 {
                app.settings_selected -= 1;
            }
        }
        KeyCode::Down => {
            if app.settings_selected < 3 {
                app.settings_selected += 1;
            }
        }
        KeyCode::Left | KeyCode::Right | KeyCode::Enter => match app.settings_selected {
            0 => app.toggle_theme(),
            1 => app.toggle_language(),
            2 => {
                if key.code == KeyCode::Left && app.config.auto_lock_minutes > 0 {
                    app.config.auto_lock_minutes = app.config.auto_lock_minutes.saturating_sub(1);
                    let _ = app.save_config();
                } else if key.code == KeyCode::Right && app.config.auto_lock_minutes < 120 {
                    app.config.auto_lock_minutes += 1;
                    let _ = app.save_config();
                }
            }
            3 => {
                if key.code == KeyCode::Left && app.config.message_timeout_seconds > 1 {
                    app.config.message_timeout_seconds =
                        app.config.message_timeout_seconds.saturating_sub(1);
                    let _ = app.save_config();
                } else if key.code == KeyCode::Right && app.config.message_timeout_seconds < 30 {
                    app.config.message_timeout_seconds += 1;
                    let _ = app.save_config();
                }
            }
            _ => {}
        },
        KeyCode::Char('s') => {
            if let Err(e) = app.save_config() {
                app.set_temp_message(format!("{}: {}", app.i18n.t("error_save_config"), e));
            } else {
                app.set_temp_message(app.i18n.t("config_saved").to_string());
            }
        }
        _ => {}
    }
    Ok(false)
}

fn handle_managing_keys_keys(app: &mut App, key: KeyEvent) -> Result<bool> {
    match key.code {
        KeyCode::Esc => {
            app.input_mode = InputMode::Explorer;
            app.new_key_comment.clear();
        }
        KeyCode::Up => {
            if app.key_manager_selected > 0 {
                app.key_manager_selected -= 1;
            }
        }
        KeyCode::Down => {
            if app.key_manager_selected < app.age_keys.len() {
                app.key_manager_selected += 1;
            }
        }
        KeyCode::Char('n') => {
            if let Err(e) = app.generate_age_key() {
                app.set_temp_message(format!("❌ Error: {}", e));
            }
        }
        KeyCode::Char('d') => {
            if app.key_manager_selected < app.age_keys.len() {
                app.input_mode = InputMode::ConfirmingKeyDeletion;
            }
        }
        KeyCode::Char(c) if app.key_manager_selected == app.age_keys.len() => {
            app.new_key_comment.push(c);
        }
        KeyCode::Backspace if app.key_manager_selected == app.age_keys.len() => {
            app.new_key_comment.pop();
        }
        _ => {}
    }
    Ok(false)
}

fn handle_confirming_key_deletion_keys(app: &mut App, key: KeyEvent) -> Result<bool> {
    match key.code {
        KeyCode::Char('y') | KeyCode::Char('Y') => {
            if let Err(e) = app.delete_selected_age_key() {
                app.set_temp_message(format!("❌ Error: {}", e));
            }
            app.input_mode = InputMode::ManagingKeys;
        }
        KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
            app.input_mode = InputMode::ManagingKeys;
        }
        _ => {}
    }
    Ok(false)
}

fn handle_creating_folder_keys(app: &mut App, key: KeyEvent) -> Result<bool> {
    match key.code {
        KeyCode::Esc => {
            app.input_mode = InputMode::Explorer;
            app.folder_name_buffer.clear();
        }
        KeyCode::Enter => {
            if let Err(e) = app.create_folder() {
                app.set_temp_message(format!("❌ Error: {}", e));
            }
            app.input_mode = InputMode::Explorer;
        }
        KeyCode::Backspace => {
            app.folder_name_buffer.pop();
        }
        KeyCode::Char(c) => {
            app.folder_name_buffer.push(c);
        }
        _ => {}
    }
    Ok(false)
}

fn handle_renaming_file_keys(app: &mut App, key: KeyEvent) -> Result<bool> {
    match key.code {
        KeyCode::Esc => {
            app.input_mode = InputMode::Explorer;
            app.rename_buffer.clear();
        }
        KeyCode::Enter => {
            if let Err(e) = app.rename_selected_file() {
                app.set_temp_message(format!("❌ Error: {}", e));
            }
            app.input_mode = InputMode::Explorer;
        }
        KeyCode::Backspace => {
            app.rename_buffer.pop();
        }
        KeyCode::Char(c) => {
            app.rename_buffer.push(c);
        }
        _ => {}
    }
    Ok(false)
}

fn handle_confirming_file_deletion_keys(app: &mut App, key: KeyEvent) -> Result<bool> {
    match key.code {
        KeyCode::Char('y') | KeyCode::Char('Y') => {
            if let Err(e) = app.delete_selected_file() {
                app.set_temp_message(format!("❌ Error: {}", e));
            }
            app.input_mode = InputMode::Explorer;
        }
        KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
            app.input_mode = InputMode::Explorer;
        }
        _ => {}
    }
    Ok(false)
}

fn handle_creating_secret_file_keys(app: &mut App, key: KeyEvent) -> Result<bool> {
    match key.code {
        KeyCode::Esc => {
            app.input_mode = InputMode::Explorer;
        }
        KeyCode::Enter => {
            // Ir al selector de llaves SOPS
            app.selected_sops_keys = vec![false; app.age_keys.len()];
            if !app.age_keys.is_empty() {
                app.selected_sops_keys[0] = true;
            }
            app.key_list_state.select(Some(0));
            app.input_mode = InputMode::SelectingSopsKeys;
        }
        KeyCode::Backspace => {
            app.new_file_name_buffer.pop();
        }
        KeyCode::Char(c) => {
            app.new_file_name_buffer.push(c);
        }
        _ => {}
    }
    Ok(false)
}

fn handle_selecting_file_format_keys(app: &mut App, key: KeyEvent) -> Result<bool> {
    match key.code {
        KeyCode::Esc => {
            app.input_mode = InputMode::Explorer;
        }
        KeyCode::Up => {
            if app.selected_format > 0 {
                app.selected_format -= 1;
            }
        }
        KeyCode::Down => {
            if app.selected_format < 3 {
                app.selected_format += 1;
            }
        }
        KeyCode::Enter => {
            // Ir a pedir nombre del archivo
            app.input_mode = InputMode::CreatingSecretFile;
        }
        _ => {}
    }
    Ok(false)
}

fn handle_selecting_sops_keys_keys(app: &mut App, key: KeyEvent) -> Result<bool> {
    match key.code {
        KeyCode::Esc => {
            app.input_mode = InputMode::Explorer;
        }
        KeyCode::Up => {
            if let Some(selected) = app.key_list_state.selected() {
                if selected > 0 {
                    app.key_list_state.select(Some(selected - 1));
                }
            }
        }
        KeyCode::Down => {
            if let Some(selected) = app.key_list_state.selected() {
                if selected < app.age_keys.len() - 1 {
                    app.key_list_state.select(Some(selected + 1));
                }
            }
        }
        KeyCode::Char(' ') | KeyCode::Enter => {
            if let Some(selected) = app.key_list_state.selected() {
                // Desmarcar todas las llaves
                for i in 0..app.selected_sops_keys.len() {
                    app.selected_sops_keys[i] = false;
                }
                // Marcar solo la seleccionada
                if let Some(checked) = app.selected_sops_keys.get_mut(selected) {
                    *checked = true;
                }

                // Si es Enter, crear el config y el archivo
                if key.code == KeyCode::Enter {
                    if let Err(e) = app.create_sops_config() {
                        app.set_temp_message(format!("❌ Error: {}", e));
                        app.input_mode = InputMode::Explorer;
                    } else if let Err(e) = app.create_encrypted_file() {
                        app.set_temp_message(format!("❌ Error: {}", e));
                        app.input_mode = InputMode::Explorer;
                    } else {
                        app.input_mode = InputMode::Explorer;
                    }
                }
            }
        }
        _ => {}
    }
    Ok(false)
}

fn handle_editing_sops_config_keys(app: &mut App, key: KeyEvent) -> Result<bool> {
    match key.code {
        KeyCode::Esc => {
            app.input_mode = InputMode::Explorer;
            app.edit_buffer.clear();
        }
        KeyCode::Enter if key.modifiers.contains(KeyModifiers::CONTROL) => {
            if let Err(e) = app.save_sops_config() {
                app.set_temp_message(format!("❌ Error: {}", e));
            }
        }
        KeyCode::Enter => {
            app.edit_buffer.insert(app.cursor_position, '\n');
            app.cursor_position += 1;
        }
        KeyCode::Char(c) => {
            app.edit_buffer.insert(app.cursor_position, c);
            app.cursor_position += 1;
        }
        KeyCode::Backspace => {
            if app.cursor_position > 0 {
                app.edit_buffer.remove(app.cursor_position - 1);
                app.cursor_position -= 1;
            }
        }
        KeyCode::Left => {
            if app.cursor_position > 0 {
                app.cursor_position -= 1;
            }
        }
        KeyCode::Right => {
            if app.cursor_position < app.edit_buffer.len() {
                app.cursor_position += 1;
            }
        }
        _ => {}
    }
    Ok(false)
}

fn handle_confirming_key_creation_keys(app: &mut App, key: KeyEvent) -> Result<bool> {
    match key.code {
        KeyCode::Char('y') | KeyCode::Char('s') => {
            if let Err(e) = crate::sops::create_age_key_file() {
                app.set_temp_message(format!("❌ Error: {}", e));
            } else {
                app.age_keys = crate::sops::load_age_keys()?;
                app.set_temp_message(app.i18n.t("key_created").to_string());
            }
            app.input_mode = InputMode::Explorer;
        }
        KeyCode::Char('n') | KeyCode::Esc => {
            app.input_mode = InputMode::Explorer;
        }
        _ => {}
    }
    Ok(false)
}

fn handle_selecting_sops_template_keys(app: &mut App, key: KeyEvent) -> Result<bool> {
    match key.code {
        KeyCode::Esc => {
            app.input_mode = InputMode::Explorer;
        }
        KeyCode::Up => {
            if app.selected_sops_template > 0 {
                app.selected_sops_template -= 1;
            }
        }
        KeyCode::Down => {
            if app.selected_sops_template < 3 {
                app.selected_sops_template += 1;
            }
        }
        KeyCode::Enter => {
            // Ir al selector de llaves
            app.selected_sops_keys = vec![false; app.age_keys.len()];
            if !app.age_keys.is_empty() {
                app.selected_sops_keys[0] = true;
            }
            app.key_list_state.select(Some(0));
            app.input_mode = InputMode::SelectingSopsKeys;
        }
        _ => {}
    }
    Ok(false)
}
