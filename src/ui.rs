use crate::config::Theme;
use crate::help::show_help;
use crate::state::{App, InputMode};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Row, Table},
    Frame,
};

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn render_banner(f: &mut Frame, area: Rect, theme: &Theme, i18n: &crate::i18n::I18n) {
    let banner_text = vec![Line::from(vec![
        Span::styled(
            "🔐  ",
            Style::default().fg(Color::Rgb(
                theme.primary.0,
                theme.primary.1,
                theme.primary.2,
            )),
        ),
        Span::styled(
            "AGESMITH",
            Style::default()
                .fg(Color::Rgb(
                    theme.primary.0,
                    theme.primary.1,
                    theme.primary.2,
                ))
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            "  v",
            Style::default().fg(Color::Rgb(
                theme.primary.0,
                theme.primary.1,
                theme.primary.2,
            )),
        ),
        Span::styled(
            VERSION,
            Style::default().fg(Color::Rgb(
                theme.primary.0,
                theme.primary.1,
                theme.primary.2,
            )),
        ),
        Span::styled(
            "  │  ",
            Style::default().fg(Color::Rgb(
                theme.primary.0,
                theme.primary.1,
                theme.primary.2,
            )),
        ),
        Span::styled(
            i18n.t("app_tagline"),
            Style::default()
                .fg(Color::Rgb(
                    theme.primary.0,
                    theme.primary.1,
                    theme.primary.2,
                ))
                .add_modifier(Modifier::ITALIC),
        ),
    ])];

    let banner = Paragraph::new(banner_text).alignment(Alignment::Center);

    f.render_widget(banner, area);
}

pub fn ui(f: &mut Frame, app: &mut App) {
    let theme = &app.theme;
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // Banner
            Constraint::Min(0),    // Content
            Constraint::Length(7), // Footer (más líneas para comandos)
        ])
        .split(f.area());

    render_banner(f, main_layout[0], theme, &app.i18n);

    let footer_text = get_footer_text(app);

    if app.input_mode == InputMode::Settings {
        render_settings_modal(f, app, main_layout[1]);
    } else {
        let content_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
            .split(main_layout[1]);

        render_file_explorer(f, app, content_chunks[0]);
        render_secrets_panel(f, app, content_chunks[1]);
    }

    render_footer(f, app, main_layout[2], &footer_text);

    if app.input_mode == InputMode::SelectingKey || app.input_mode == InputMode::SearchingKey {
        render_key_selector_modal(f, app);
    }

    if app.input_mode == InputMode::ViewingValue {
        render_value_viewer_modal(f, app);
    }

    if app.input_mode == InputMode::Editing || app.input_mode == InputMode::AddingSecret {
        render_edit_modal(f, app);
    }

    if app.input_mode == InputMode::Confirming {
        render_confirm_modal(f, app);
    }

    if app.input_mode == InputMode::Help {
        render_help_modal(f, app);
    }

    if app.input_mode == InputMode::Settings {
        // Already rendered in main layout
    }

    if app.input_mode == InputMode::ManagingKeys {
        render_key_manager_modal(f, app);
    }

    if app.input_mode == InputMode::ConfirmingKeyDeletion {
        render_confirm_key_deletion_modal(f, app);
    }

    if app.input_mode == InputMode::ConfirmingKeyCreation {
        render_confirm_key_creation_modal(f, app);
    }

    if app.input_mode == InputMode::CreatingFolder {
        render_creating_folder_modal(f, app);
    }

    if app.input_mode == InputMode::RenamingFile {
        render_renaming_file_modal(f, app);
    }

    if app.input_mode == InputMode::ConfirmingFileDeletion {
        render_confirm_file_deletion_modal(f, app);
    }

    if app.input_mode == InputMode::SelectingFileFormat {
        render_selecting_file_format_modal(f, app);
    }

    if app.input_mode == InputMode::CreatingSecretFile {
        render_creating_secret_file_modal(f, app);
    }

    if app.input_mode == InputMode::SelectingSopsKeys {
        render_selecting_sops_keys_modal(f, app);
    }

    if app.input_mode == InputMode::EditingSopsConfig {
        render_editing_sops_config_modal(f, app);
    }

    if app.input_mode == InputMode::SelectingSopsTemplate {
        render_selecting_sops_template_modal(f, app);
    }
}

fn render_settings_modal(f: &mut Frame, app: &App, area: Rect) {
    let theme = &app.theme;
    let area = centered_rect(70, 60, area);
    f.render_widget(Clear, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(0),
        ])
        .split(area);

    let block = Block::default()
        .borders(Borders::ALL)
        .title("⚙️  Settings [Ctrl+S]")
        .title_style(
            Style::default()
                .fg(Color::Rgb(
                    theme.primary.0,
                    theme.primary.1,
                    theme.primary.2,
                ))
                .add_modifier(Modifier::BOLD),
        )
        .border_style(Style::default().fg(Color::Rgb(
            theme.success.0,
            theme.success.1,
            theme.success.2,
        )))
        .style(Style::default().bg(Color::Rgb(theme.bg.0, theme.bg.1, theme.bg.2)));

    f.render_widget(block, area);

    // Theme
    let theme_style = if app.settings_selected == 0 {
        Style::default()
            .fg(Color::Rgb(
                theme.warning.0,
                theme.warning.1,
                theme.warning.2,
            ))
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Rgb(theme.fg.0, theme.fg.1, theme.fg.2))
    };
    let theme_text = vec![
        Line::from(format!(
            "{}: {} {}",
            app.i18n.t("settings_theme"),
            app.config.theme,
            app.i18n.t("settings_change")
        )),
        Line::from(Span::styled(
            format!("  {}", app.i18n.t("settings_theme_help")),
            Style::default()
                .fg(Color::Rgb(theme.fg.0 / 2, theme.fg.1 / 2, theme.fg.2 / 2))
                .add_modifier(Modifier::ITALIC),
        )),
    ];
    let theme_widget = Paragraph::new(theme_text).style(theme_style);
    f.render_widget(theme_widget, chunks[0]);

    // Language
    let lang_style = if app.settings_selected == 1 {
        Style::default()
            .fg(Color::Rgb(
                theme.warning.0,
                theme.warning.1,
                theme.warning.2,
            ))
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Rgb(theme.fg.0, theme.fg.1, theme.fg.2))
    };
    let lang_text = vec![
        Line::from(format!(
            "{}: {} {}",
            app.i18n.t("settings_language"),
            app.config.language,
            app.i18n.t("settings_change")
        )),
        Line::from(Span::styled(
            format!("  {}", app.i18n.t("settings_language_help")),
            Style::default()
                .fg(Color::Rgb(theme.fg.0 / 2, theme.fg.1 / 2, theme.fg.2 / 2))
                .add_modifier(Modifier::ITALIC),
        )),
    ];
    let lang = Paragraph::new(lang_text).style(lang_style);
    f.render_widget(lang, chunks[1]);

    // Auto-lock
    let lock_style = if app.settings_selected == 2 {
        Style::default()
            .fg(Color::Rgb(
                theme.warning.0,
                theme.warning.1,
                theme.warning.2,
            ))
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Rgb(theme.fg.0, theme.fg.1, theme.fg.2))
    };
    let lock_value = if app.config.auto_lock_minutes == 0 {
        format!(
            "{}: {} {}",
            app.i18n.t("settings_autolock"),
            app.i18n.t("settings_disabled"),
            app.i18n.t("settings_adjust")
        )
    } else {
        format!(
            "{}: {} {} {}",
            app.i18n.t("settings_autolock"),
            app.config.auto_lock_minutes,
            app.i18n.t("settings_minutes"),
            app.i18n.t("settings_adjust")
        )
    };
    let lock_text = vec![
        Line::from(lock_value),
        Line::from(Span::styled(
            format!("  {}", app.i18n.t("settings_autolock_help")),
            Style::default()
                .fg(Color::Rgb(theme.fg.0 / 2, theme.fg.1 / 2, theme.fg.2 / 2))
                .add_modifier(Modifier::ITALIC),
        )),
    ];
    let lock = Paragraph::new(lock_text).style(lock_style);
    f.render_widget(lock, chunks[2]);

    // Message timeout
    let msg_style = if app.settings_selected == 3 {
        Style::default()
            .fg(Color::Rgb(
                theme.warning.0,
                theme.warning.1,
                theme.warning.2,
            ))
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Rgb(theme.fg.0, theme.fg.1, theme.fg.2))
    };
    let msg_text = vec![
        Line::from(format!(
            "{}: {} {} {}",
            app.i18n.t("settings_timeout"),
            app.config.message_timeout_seconds,
            app.i18n.t("settings_seconds"),
            app.i18n.t("settings_adjust")
        )),
        Line::from(Span::styled(
            format!("  {}", app.i18n.t("settings_timeout_help")),
            Style::default()
                .fg(Color::Rgb(theme.fg.0 / 2, theme.fg.1 / 2, theme.fg.2 / 2))
                .add_modifier(Modifier::ITALIC),
        )),
    ];
    let msg = Paragraph::new(msg_text).style(msg_style);
    f.render_widget(msg, chunks[3]);

    // Help text
    let help_text = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "[↑↓] ",
                Style::default().fg(Color::Rgb(
                    theme.success.0,
                    theme.success.1,
                    theme.success.2,
                )),
            ),
            Span::raw(format!(
                "{}  ",
                app.i18n.t("cmd_navigate").replace("[↑↓] ", "")
            )),
            Span::styled(
                "[←→] ",
                Style::default().fg(Color::Rgb(
                    theme.success.0,
                    theme.success.1,
                    theme.success.2,
                )),
            ),
            Span::raw(format!(
                "{}  ",
                if app.i18n.t("settings_change").contains("change") {
                    "Change"
                } else {
                    "Cambiar"
                }
            )),
            Span::styled(
                "[s] ",
                Style::default().fg(Color::Rgb(
                    theme.success.0,
                    theme.success.1,
                    theme.success.2,
                )),
            ),
            Span::raw(format!("{}  ", app.i18n.t("cmd_save").replace("[s] ", ""))),
            Span::styled(
                "[Esc] ",
                Style::default().fg(Color::Rgb(theme.error.0, theme.error.1, theme.error.2)),
            ),
            Span::raw(if app.i18n.t("cmd_cancel").contains("Cancel") {
                "Close"
            } else {
                "Cerrar"
            }),
        ]),
    ];
    let help = Paragraph::new(help_text)
        .style(Style::default().fg(Color::Rgb(
            (theme.fg.0 as u16 * 3 / 5) as u8,
            (theme.fg.1 as u16 * 3 / 5) as u8,
            (theme.fg.2 as u16 * 3 / 5) as u8,
        )))
        .alignment(Alignment::Center);
    f.render_widget(help, chunks[4]);
}

fn render_file_explorer(f: &mut Frame, app: &mut App, area: Rect) {
    // Split area for breadcrumb and file list
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // Breadcrumb
            Constraint::Min(0),    // File list
        ])
        .split(area);

    // Render breadcrumb
    let current_path = app.current_dir.to_string_lossy();
    let breadcrumb =
        Paragraph::new(format!("📂 {}", current_path)).style(Style::default().fg(Color::Rgb(
            (app.theme.primary.0 as u16 * 3 / 4) as u8,
            (app.theme.primary.1 as u16 * 3 / 4) as u8,
            (app.theme.primary.2 as u16 * 3 / 4) as u8,
        )));
    f.render_widget(breadcrumb, chunks[0]);

    let file_items: Vec<ListItem> = app
        .files
        .iter()
        .map(|path| {
            let name = if path.to_str() == Some("..") {
                "..".to_string()
            } else {
                path.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("?")
                    .to_string()
            };

            let mut prefix = String::new();
            let mut style = Style::default();

            if app.favorites.contains(path) {
                prefix.push_str("⭐ ");
            }

            if app.marked_files.contains(path) {
                prefix.push_str("✓ ");
            }

            if path.is_dir() || path.to_str() == Some("..") {
                prefix.push_str("📁 ");
                style = style.fg(Color::Rgb(100, 181, 246));
            } else {
                prefix.push_str("📄 ");
                style = style.fg(Color::Rgb(178, 235, 242));
            }

            ListItem::new(format!("{}{}", prefix, name)).style(style)
        })
        .collect();

    let file_list = List::new(file_items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!(
                    "{} ({} {}, {} {})",
                    app.i18n.t("explorer"),
                    app.marked_files.len(),
                    app.i18n.t("marked"),
                    app.favorites.len(),
                    app.i18n.t("favorites")
                ))
                .title_style(
                    Style::default()
                        .fg(Color::Rgb(129, 212, 250))
                        .add_modifier(Modifier::BOLD),
                )
                .border_style(if app.input_mode == InputMode::Explorer {
                    Style::default().fg(Color::Rgb(102, 187, 106))
                } else {
                    Style::default().fg(Color::Rgb(66, 66, 66))
                }),
        )
        .highlight_style(
            Style::default()
                .bg(Color::Rgb(38, 50, 56))
                .fg(Color::Rgb(255, 213, 79))
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("▶ ");

    f.render_stateful_widget(file_list, chunks[1], &mut app.file_list_state);
}

fn render_secrets_panel(f: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(area);

    let search_input = Paragraph::new(app.secret_search_query.as_str())
        .style(Style::default().fg(Color::Rgb(255, 255, 255)))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(if app.input_mode == InputMode::SearchingSecrets {
                    if app.use_regex {
                        format!(
                            "{} ({}, {})",
                            app.i18n.t("search_secret_regex"),
                            app.i18n.t("search_cancel"),
                            app.i18n.t("search_regex")
                        )
                    } else {
                        format!(
                            "{} ({}, {})",
                            app.i18n.t("search_secret_normal"),
                            app.i18n.t("search_cancel"),
                            app.i18n.t("search_regex")
                        )
                    }
                } else if app.use_regex {
                    format!(
                        "{} {}",
                        app.i18n.t("press_search"),
                        app.i18n.t("regex_active")
                    )
                } else {
                    app.i18n.t("press_search").to_string()
                })
                .title_style(
                    Style::default()
                        .fg(Color::Rgb(255, 167, 38))
                        .add_modifier(Modifier::BOLD),
                )
                .border_style(if app.input_mode == InputMode::SearchingSecrets {
                    Style::default().fg(Color::Rgb(255, 167, 38))
                } else {
                    Style::default().fg(Color::Rgb(66, 66, 66))
                }),
        );
    f.render_widget(search_input, chunks[0]);

    if app.input_mode == InputMode::SearchingSecrets {
        f.set_cursor_position((
            chunks[0].x + app.secret_search_query.len() as u16 + 1,
            chunks[0].y + 1,
        ));
    }

    let header = Row::new(vec!["🔑 Key", "🔐 Value"]).style(
        Style::default()
            .fg(Color::Rgb(171, 71, 188))
            .add_modifier(Modifier::BOLD),
    );

    let filtered_indices = app.filtered_secrets();
    let rows: Vec<Row> = filtered_indices
        .iter()
        .filter_map(|&i| app.secrets.get(i))
        .map(|(k, v)| {
            let display_value = if app.is_encrypted(k) {
                if app.show_values {
                    v.clone()
                } else {
                    "••••••••".to_string()
                }
            } else {
                v.clone()
            };
            Row::new(vec![k.clone(), display_value])
        })
        .collect();

    let title = if let Some(path) = &app.file_path {
        let count_info = if app.secret_search_query.is_empty() {
            format!("{}", app.secrets.len())
        } else {
            format!("{}/{}", filtered_indices.len(), app.secrets.len())
        };
        let modified = if app.is_modified {
            app.i18n.t("modified")
        } else {
            ""
        };
        format!(
            "{}: {} ({}){}",
            app.i18n.t("secrets"),
            path.file_name().and_then(|n| n.to_str()).unwrap_or("?"),
            count_info,
            modified
        )
    } else {
        format!("{}: {}", app.i18n.t("secrets"), app.i18n.t("no_file"))
    };

    let table = Table::new(
        rows,
        [Constraint::Percentage(40), Constraint::Percentage(60)],
    )
    .header(header)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title(title)
            .title_style(
                Style::default()
                    .fg(Color::Rgb(129, 212, 250))
                    .add_modifier(Modifier::BOLD),
            )
            .border_style(
                if app.input_mode == InputMode::Secrets
                    || app.input_mode == InputMode::SearchingSecrets
                {
                    Style::default().fg(Color::Rgb(102, 187, 106))
                } else {
                    Style::default().fg(Color::Rgb(66, 66, 66))
                },
            ),
    )
    .row_highlight_style(
        Style::default()
            .bg(Color::Rgb(38, 50, 56))
            .fg(Color::Rgb(255, 213, 79))
            .add_modifier(Modifier::BOLD),
    )
    .highlight_symbol("▶ ");

    f.render_stateful_widget(table, chunks[1], &mut app.table_state);
}

fn render_footer(f: &mut Frame, app: &App, area: Rect, footer_text: &str) {
    let footer_style = if app.error_message.is_some() {
        if app.error_message.as_ref().unwrap().starts_with("✓") {
            Style::default()
                .fg(Color::Rgb(102, 187, 106))
                .add_modifier(Modifier::BOLD)
        } else if app.error_message.as_ref().unwrap().starts_with("❌") {
            Style::default()
                .fg(Color::Rgb(239, 83, 80))
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default()
                .fg(Color::Rgb(255, 167, 38))
                .add_modifier(Modifier::BOLD)
        }
    } else {
        Style::default().fg(Color::Rgb(129, 212, 250))
    };

    let footer = Paragraph::new(footer_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Rgb(66, 66, 66))),
        )
        .style(footer_style)
        .wrap(ratatui::widgets::Wrap { trim: false });

    f.render_widget(footer, area);
}

fn render_key_selector_modal(f: &mut Frame, app: &mut App) {
    let area = centered_rect(80, 70, f.area());
    f.render_widget(Clear, area);

    let modal_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(area);

    let search_input = Paragraph::new(app.key_search_query.as_str())
        .style(
            Style::default()
                .fg(Color::Rgb(255, 255, 255))
                .bg(Color::Rgb(38, 50, 56)),
        )
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(if app.input_mode == InputMode::SearchingKey {
                    format!(
                        "{} ({})",
                        app.i18n.t("search_key_title"),
                        app.i18n.t("search_cancel")
                    )
                } else {
                    app.i18n.t("press_search").to_string()
                })
                .title_style(
                    Style::default()
                        .fg(Color::Rgb(255, 167, 38))
                        .add_modifier(Modifier::BOLD),
                )
                .border_style(if app.input_mode == InputMode::SearchingKey {
                    Style::default().fg(Color::Rgb(255, 167, 38))
                } else {
                    Style::default().fg(Color::Rgb(102, 187, 106))
                })
                .style(Style::default().bg(Color::Rgb(38, 50, 56))),
        );
    f.render_widget(search_input, modal_chunks[0]);

    if app.input_mode == InputMode::SearchingKey {
        f.set_cursor_position((
            modal_chunks[0].x + app.key_search_query.len() as u16 + 1,
            modal_chunks[0].y + 1,
        ));
    }

    let filtered = app.filtered_keys();
    let items: Vec<ListItem> = filtered
        .iter()
        .map(|(i, key)| {
            let label = key.comment.as_deref().unwrap_or(app.i18n.t("unnamed"));
            let pub_key_short = key
                .public_key
                .as_ref()
                .map(|p| {
                    if p.len() > 20 {
                        format!("{}...", &p[..20])
                    } else {
                        p.clone()
                    }
                })
                .unwrap_or_else(|| "?".to_string());

            let mut prefix = String::new();
            if Some(*i) == app.selected_key_index {
                prefix.push_str(&format!("{} ", app.i18n.t("active")));
            }

            if let Some(pub_key) = &key.public_key {
                if app.file_recipients.contains(pub_key) {
                    prefix.push_str("✓ ");
                }
            }

            ListItem::new(format!("{}{} | {}", prefix, label, pub_key_short))
        })
        .collect();

    let title = if app.file_recipients.is_empty() {
        format!(
            "{} ({}) {}",
            app.i18n.t("keys"),
            filtered.len(),
            app.i18n.t("keys_search_apply")
        )
    } else {
        format!(
            "{} ({}) - {} {}",
            app.i18n.t("keys"),
            filtered.len(),
            app.i18n.t("keys_matches"),
            app.i18n.t("keys_search_apply")
        )
    };

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(title)
                .title_style(
                    Style::default()
                        .fg(Color::Rgb(171, 71, 188))
                        .add_modifier(Modifier::BOLD),
                )
                .border_style(Style::default().fg(Color::Rgb(102, 187, 106)))
                .style(Style::default().bg(Color::Rgb(38, 50, 56))),
        )
        .highlight_style(
            Style::default()
                .bg(Color::Rgb(66, 66, 66))
                .fg(Color::Rgb(255, 213, 79))
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("▶ ");

    f.render_stateful_widget(list, modal_chunks[1], &mut app.key_list_state);
}

fn render_value_viewer_modal(f: &mut Frame, app: &mut App) {
    if let Some(value) = &app.viewing_value {
        let area = centered_rect(90, 90, f.area());
        f.render_widget(Clear, area);

        let formatted_value = app.format_json_value(value);
        let lines: Vec<&str> = formatted_value.lines().collect();
        let scroll_offset = app.viewing_scroll as usize;
        let visible_lines: Vec<String> = lines
            .iter()
            .skip(scroll_offset)
            .take((area.height as usize).saturating_sub(4))
            .map(|s| s.to_string())
            .collect();

        let content = visible_lines.join("\n");
        let paragraph = Paragraph::new(content)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(format!(
                        "{} ({} {}-{}/{}) {}",
                        app.i18n.t("full_value"),
                        app.i18n.t("lines"),
                        scroll_offset + 1,
                        (scroll_offset + visible_lines.len()).min(lines.len()),
                        lines.len(),
                        app.i18n.t("value_help")
                    ))
                    .title_style(
                        Style::default()
                            .fg(Color::Rgb(129, 212, 250))
                            .add_modifier(Modifier::BOLD),
                    )
                    .border_style(Style::default().fg(Color::Rgb(102, 187, 106)))
                    .style(Style::default().bg(Color::Rgb(38, 50, 56))),
            )
            .style(
                Style::default()
                    .fg(Color::Rgb(255, 255, 255))
                    .bg(Color::Rgb(38, 50, 56)),
            );

        f.render_widget(paragraph, area);
    }
}

fn render_edit_modal(f: &mut Frame, app: &mut App) {
    let area = centered_rect(70, 30, f.area());
    f.render_widget(Clear, area);

    let title = if app.input_mode == InputMode::Editing {
        format!(
            "{} {} | {} | {}",
            app.i18n.t("edit_secret"),
            app.i18n.t("tab_switch"),
            app.i18n.t("enter_save"),
            app.i18n.t("esc_cancel")
        )
    } else {
        format!(
            "{} {} | {} | {}",
            app.i18n.t("add_secret"),
            app.i18n.t("tab_switch"),
            app.i18n.t("enter_save"),
            app.i18n.t("esc_cancel")
        )
    };

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(0),
        ])
        .split(area);

    let key_input = Paragraph::new(app.editing_key_buffer.as_str())
        .style(
            Style::default()
                .fg(Color::Rgb(255, 255, 255))
                .bg(Color::Rgb(38, 50, 56)),
        )
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(title)
                .title_style(
                    Style::default()
                        .fg(Color::Rgb(255, 167, 38))
                        .add_modifier(Modifier::BOLD),
                )
                .border_style(if app.editing_field == 0 {
                    Style::default().fg(Color::Rgb(102, 187, 106))
                } else {
                    Style::default().fg(Color::Rgb(66, 66, 66))
                })
                .style(Style::default().bg(Color::Rgb(38, 50, 56))),
        );
    f.render_widget(key_input, chunks[0]);

    let value_title = if app.editing_field == 1 {
        "Valor | [Ctrl+g] Generar | Usa \"texto\" para forzar string".to_string()
    } else {
        app.i18n.t("value_field").to_string()
    };

    let value_input = Paragraph::new(app.editing_value_buffer.as_str())
        .style(
            Style::default()
                .fg(Color::Rgb(255, 255, 255))
                .bg(Color::Rgb(38, 50, 56)),
        )
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(value_title)
                .title_style(
                    Style::default()
                        .fg(Color::Rgb(129, 212, 250))
                        .add_modifier(Modifier::BOLD),
                )
                .border_style(if app.editing_field == 1 {
                    Style::default().fg(Color::Rgb(102, 187, 106))
                } else {
                    Style::default().fg(Color::Rgb(66, 66, 66))
                })
                .style(Style::default().bg(Color::Rgb(38, 50, 56))),
        );
    f.render_widget(value_input, chunks[1]);

    if app.editing_field == 0 {
        f.set_cursor_position((
            chunks[0].x + app.cursor_position as u16 + 1,
            chunks[0].y + 1,
        ));
    } else {
        f.set_cursor_position((
            chunks[1].x + app.cursor_position as u16 + 1,
            chunks[1].y + 1,
        ));
    }
}

fn render_confirm_modal(f: &mut Frame, app: &App) {
    let area = centered_rect(50, 15, f.area());
    f.render_widget(Clear, area);

    let confirm = Paragraph::new(app.i18n.t("confirm_delete"))
        .style(
            Style::default()
                .fg(Color::Rgb(255, 255, 255))
                .bg(Color::Rgb(38, 50, 56)),
        )
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(app.i18n.t("confirm"))
                .title_style(
                    Style::default()
                        .fg(Color::Rgb(239, 83, 80))
                        .add_modifier(Modifier::BOLD),
                )
                .border_style(Style::default().fg(Color::Rgb(239, 83, 80)))
                .style(Style::default().bg(Color::Rgb(38, 50, 56))),
        );
    f.render_widget(confirm, area);
}

fn render_help_modal(f: &mut Frame, app: &App) {
    let area = centered_rect(80, 80, f.area());
    f.render_widget(Clear, area);

    let help_sections = show_help(&app.i18n);
    let mut help_items = Vec::new();

    for (section_title, commands) in help_sections {
        help_items.push(
            ListItem::new(format!("\n{}", section_title)).style(
                Style::default()
                    .fg(Color::Rgb(102, 187, 106))
                    .add_modifier(Modifier::BOLD),
            ),
        );

        for (key, desc) in commands {
            help_items.push(
                ListItem::new(format!("  {} - {}", key, desc))
                    .style(Style::default().fg(Color::Rgb(255, 255, 255))),
            );
        }
    }

    let help_list = List::new(help_items).block(
        Block::default()
            .borders(Borders::ALL)
            .title(format!(
                "{} {}",
                app.i18n.t("help"),
                app.i18n.t("close_help")
            ))
            .title_style(
                Style::default()
                    .fg(Color::Rgb(129, 212, 250))
                    .add_modifier(Modifier::BOLD),
            )
            .border_style(Style::default().fg(Color::Rgb(102, 187, 106)))
            .style(Style::default().bg(Color::Rgb(38, 50, 56))),
    );

    f.render_widget(help_list, area);
}

fn get_footer_text(app: &App) -> String {
    let key_info = if let Some(idx) = app.selected_key_index {
        if let Some(key) = app.age_keys.get(idx) {
            format!(
                " | {}: {}",
                app.i18n.t("key_info"),
                key.comment
                    .as_ref()
                    .unwrap_or(&app.i18n.t("unnamed").to_string())
            )
        } else {
            format!(
                " | {}: {}",
                app.i18n.t("key_info"),
                app.i18n.t("key_info_none")
            )
        }
    } else {
        format!(
            " | {}: {}",
            app.i18n.t("key_info"),
            app.i18n.t("key_info_auto")
        )
    };

    if let Some(err) = &app.error_message {
        err.clone()
    } else {
        match app.input_mode {
            InputMode::Explorer => {
                let selected_file = if let Some(idx) = app.file_list_state.selected() {
                    app.files
                        .get(idx)
                        .and_then(|p| p.file_name())
                        .and_then(|n| n.to_str())
                        .map(|s| format!(" '{}'", s))
                        .unwrap_or_default()
                } else {
                    String::new()
                };

                let rename_cmd = if selected_file.is_empty() || selected_file == " '..'" {
                    String::new()
                } else {
                    format!(" | [r] {}{}", app.i18n.t("cmd_rename_file"), selected_file)
                };

                let delete_cmd = if selected_file.is_empty() || selected_file == " '..'" {
                    String::new()
                } else {
                    format!(" | [D] {}{}", app.i18n.t("cmd_delete_item"), selected_file)
                };

                format!(
                    "{}: {} | {}\n{}: {} | {} | {}\n{}: {} | {}{}{}{}",
                    app.i18n.t("cat_navigation"),
                    app.i18n.t("cmd_navigate"),
                    app.i18n.t("cmd_open"),
                    app.i18n.t("cat_files"),
                    app.i18n.t("cmd_mark"),
                    app.i18n.t("cmd_new_folder"),
                    app.i18n.t("cmd_new_file"),
                    app.i18n.t("cat_management"),
                    app.i18n.t("cmd_init"),
                    app.i18n.t("cmd_key_manager"),
                    rename_cmd,
                    delete_cmd,
                    key_info
                )
            }
            InputMode::Secrets => {
                let selected_secret = if let Some(idx) = app.table_state.selected() {
                    let filtered = app.filtered_secrets();
                    filtered
                        .get(idx)
                        .and_then(|&real_idx| app.secrets.get(real_idx))
                        .map(|(key, _)| format!(" '{}'", key))
                        .unwrap_or_default()
                } else {
                    String::new()
                };

                let edit_cmd = if selected_secret.is_empty() {
                    app.i18n.t("cmd_edit").to_string()
                } else {
                    format!("[e] {}{}", app.i18n.t("cmd_edit_item"), selected_secret)
                };

                let delete_cmd = if selected_secret.is_empty() {
                    app.i18n.t("cmd_delete").to_string()
                } else {
                    format!("[d] {}{}", app.i18n.t("cmd_delete_item"), selected_secret)
                };

                let edit_cmds = if app.is_modified {
                    format!(" | {}", app.i18n.t("cmd_save"))
                } else {
                    String::new()
                };
                let fav_indicator = if app
                    .file_path
                    .as_ref()
                    .map(|p| app.favorites.contains(p))
                    .unwrap_or(false)
                {
                    " ⭐"
                } else {
                    ""
                };
                format!(
                    "{}: {} | {}: [v] {} | {} | {}{}\n{}: {} | {} | {}{}\n{}: {} | {} | {} | {} | {}{}",
                    app.i18n.t("cat_navigation"),
                    app.i18n.t("cmd_navigate"),
                    app.i18n.t("cat_view"),
                    if app.show_values { app.i18n.t("cmd_hide") } else { app.i18n.t("cmd_show") },
                    app.i18n.t("cmd_zoom"),
                    app.i18n.t("cmd_favorite"),
                    fav_indicator,
                    app.i18n.t("cat_editing"),
                    edit_cmd,
                    app.i18n.t("cmd_new"),
                    delete_cmd,
                    edit_cmds,
                    app.i18n.t("cat_tools"),
                    app.i18n.t("cmd_copy"),
                    app.i18n.t("cmd_copy_key"),
                    app.i18n.t("cmd_search"),
                    app.i18n.t("cmd_generate"),
                    app.i18n.t("cmd_key_selector"),
                    key_info
                )
            }
            InputMode::SelectingKey => app.i18n.t("selecting_key").to_string(),
            InputMode::SearchingKey => app.i18n.t("searching_key").to_string(),
            InputMode::SearchingSecrets => {
                if app.use_regex {
                    format!(
                        "{} [REGEX] - {}...",
                        app.i18n.t("searching_secret"),
                        app.i18n.t("search_regex")
                    )
                } else {
                    format!(
                        "{} - {}...",
                        app.i18n.t("searching_secret"),
                        app.i18n.t("search_regex")
                    )
                }
            }
            InputMode::ViewingValue => {
                format!(
                    "{} - {}",
                    app.i18n.t("viewing_value"),
                    app.i18n.t("scroll_json")
                )
            }
            InputMode::Editing => {
                format!(
                    "{} | {} | {} | {}",
                    app.i18n.t("cmd_apply"),
                    app.i18n.t("cmd_cancel"),
                    app.i18n.t("move_cursor"),
                    app.i18n.t("home_end")
                )
            }
            InputMode::AddingSecret => {
                format!(
                    "{} | {} | {} | {}",
                    app.i18n.t("cmd_apply"),
                    app.i18n.t("cmd_cancel"),
                    app.i18n.t("move_cursor"),
                    app.i18n.t("home_end")
                )
            }
            InputMode::Confirming => {
                format!("{} | {}", app.i18n.t("confirm_y"), app.i18n.t("cancel_n"))
            }
            InputMode::Generating => app.i18n.t("generating").to_string(),
            InputMode::Help => format!(
                "{} | [?] {}",
                app.i18n.t("close_help"),
                app.i18n.t("cmd_help")
            ),
            InputMode::Settings => format!(
                "{} | [←→] Change | {} | {}",
                app.i18n.t("cmd_navigate"),
                app.i18n.t("cmd_save"),
                app.i18n.t("cmd_cancel")
            ),
            InputMode::ManagingKeys => {
                let selected_key = if app.key_manager_selected < app.age_keys.len() {
                    app.age_keys
                        .get(app.key_manager_selected)
                        .and_then(|k| k.comment.as_ref())
                        .map(|c| format!(" '{}'", c))
                        .unwrap_or_else(|| format!(" '{}'", app.i18n.t("unnamed_key")))
                } else {
                    String::new()
                };

                let delete_cmd = if selected_key.is_empty() {
                    String::new()
                } else {
                    format!(" | {}{}", app.i18n.t("delete_key_cmd"), selected_key)
                };

                format!(
                    "{}{} | {}",
                    app.i18n.t("footer_key_manager"),
                    delete_cmd,
                    app.i18n.t("footer_close")
                )
            }
            InputMode::ConfirmingKeyDeletion => app.i18n.t("confirm_deletion_help").to_string(),
            InputMode::ConfirmingKeyCreation => app.i18n.t("confirm_key_creation_help").to_string(),
            InputMode::CreatingFolder => app.i18n.t("footer_create_folder").to_string(),
            InputMode::RenamingFile => app.i18n.t("footer_rename").to_string(),
            InputMode::ConfirmingFileDeletion => app.i18n.t("confirm_deletion_help").to_string(),
            InputMode::SelectingFileFormat => app.i18n.t("footer_select_format").to_string(),
            InputMode::CreatingSecretFile => app.i18n.t("footer_create_file").to_string(),
            InputMode::SelectingSopsKeys => app.i18n.t("footer_select_sops_keys").to_string(),
            InputMode::EditingSopsConfig => app.i18n.t("footer_edit_sops").to_string(),
            InputMode::SelectingSopsTemplate => app.i18n.t("footer_select_template").to_string(),
        }
    }
}

fn render_key_manager_modal(f: &mut Frame, app: &App) {
    let area = centered_rect(80, 70, f.area());
    f.render_widget(Clear, area);

    let block = Block::default()
        .borders(Borders::ALL)
        .title(app.i18n.t("key_manager_title"))
        .title_style(
            Style::default()
                .fg(Color::Rgb(
                    app.theme.primary.0,
                    app.theme.primary.1,
                    app.theme.primary.2,
                ))
                .add_modifier(Modifier::BOLD),
        )
        .border_style(Style::default().fg(Color::Rgb(
            app.theme.success.0,
            app.theme.success.1,
            app.theme.success.2,
        )));

    f.render_widget(block, area);

    let inner = area.inner(ratatui::layout::Margin::new(2, 2));

    let mut items: Vec<ListItem> = app
        .age_keys
        .iter()
        .enumerate()
        .map(|(i, key)| {
            let name = key.comment.as_deref().unwrap_or("Unnamed");
            let pub_key = key.public_key.as_deref().unwrap_or("N/A");
            let pub_key_short = if pub_key.len() > 20 {
                format!("{}...", &pub_key[..20])
            } else {
                pub_key.to_string()
            };

            let style = if i == app.key_manager_selected {
                Style::default()
                    .fg(Color::Rgb(
                        app.theme.warning.0,
                        app.theme.warning.1,
                        app.theme.warning.2,
                    ))
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Rgb(app.theme.fg.0, app.theme.fg.1, app.theme.fg.2))
            };

            ListItem::new(format!("  {} | {}", name, pub_key_short)).style(style)
        })
        .collect();

    // Add "New key" option
    let new_key_style = if app.key_manager_selected == app.age_keys.len() {
        Style::default()
            .fg(Color::Rgb(
                app.theme.success.0,
                app.theme.success.1,
                app.theme.success.2,
            ))
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Rgb(
            app.theme.success.0,
            app.theme.success.1,
            app.theme.success.2,
        ))
    };

    items.push(
        ListItem::new(format!("  ➕ New key: {}_", app.new_key_comment)).style(new_key_style),
    );

    let list = List::new(items);
    f.render_widget(list, inner);
}

fn render_confirm_key_deletion_modal(f: &mut Frame, app: &App) {
    let area = centered_rect(50, 25, f.area());
    f.render_widget(Clear, area);

    let key_name = if app.key_manager_selected < app.age_keys.len() {
        app.age_keys[app.key_manager_selected]
            .comment
            .as_deref()
            .unwrap_or("Unnamed")
    } else {
        "Unknown"
    };

    let text = vec![
        Line::from(""),
        Line::from(Span::styled(
            format!("{} '{}'?", app.i18n.t("delete_key_question"), key_name),
            Style::default()
                .fg(Color::Rgb(255, 255, 255))
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(Span::styled(
            app.i18n.t("action_irreversible"),
            Style::default().fg(Color::Rgb(239, 83, 80)),
        )),
        Line::from(""),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "[y] ",
                Style::default()
                    .fg(Color::Rgb(239, 83, 80))
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(format!(
                "{}     ",
                if app.i18n.t("confirm_deletion_help").contains("Confirmar") {
                    "Confirmar eliminación"
                } else {
                    "Confirm deletion"
                }
            )),
            Span::styled(
                "[n] ",
                Style::default()
                    .fg(Color::Rgb(102, 187, 106))
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(if app.i18n.t("cmd_cancel").contains("Cancelar") {
                "Cancelar"
            } else {
                "Cancel"
            }),
        ]),
    ];

    let confirm = Paragraph::new(text)
        .style(
            Style::default()
                .fg(Color::Rgb(255, 255, 255))
                .bg(Color::Rgb(38, 50, 56)),
        )
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(app.i18n.t("confirm_key_deletion"))
                .title_style(
                    Style::default()
                        .fg(Color::Rgb(239, 83, 80))
                        .add_modifier(Modifier::BOLD),
                )
                .border_style(Style::default().fg(Color::Rgb(239, 83, 80))),
        );

    f.render_widget(confirm, area);
}

fn render_confirm_key_creation_modal(f: &mut Frame, app: &App) {
    let area = centered_rect(60, 30, f.area());
    f.render_widget(Clear, area);

    let text = vec![
        Line::from(""),
        Line::from(Span::styled(
            app.i18n.t("no_keys_found"),
            Style::default()
                .fg(Color::Rgb(255, 255, 255))
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(Span::styled(
            app.i18n.t("keys_file_missing"),
            Style::default().fg(Color::Rgb(189, 189, 189)),
        )),
        Line::from(""),
        Line::from(Span::styled(
            app.i18n.t("create_key_question"),
            Style::default().fg(Color::Rgb(255, 255, 255)),
        )),
        Line::from(""),
        Line::from(""),
        Line::from(Span::styled(
            app.i18n.t("confirm_key_creation_help"),
            Style::default().fg(Color::Rgb(189, 189, 189)),
        )),
    ];

    let confirm = Paragraph::new(text)
        .style(
            Style::default()
                .fg(Color::Rgb(255, 255, 255))
                .bg(Color::Rgb(38, 50, 56)),
        )
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(app.i18n.t("confirm_key_creation"))
                .title_style(
                    Style::default()
                        .fg(Color::Rgb(102, 187, 106))
                        .add_modifier(Modifier::BOLD),
                )
                .border_style(Style::default().fg(Color::Rgb(102, 187, 106))),
        );

    f.render_widget(confirm, area);
}

fn render_creating_folder_modal(f: &mut Frame, app: &App) {
    let area = centered_rect(50, 15, f.area());
    f.render_widget(Clear, area);

    let input = Paragraph::new(format!(
        "{}\n\n{}_",
        app.i18n.t("enter_folder_name"),
        app.folder_name_buffer
    ))
    .style(
        Style::default()
            .fg(Color::Rgb(255, 255, 255))
            .bg(Color::Rgb(38, 50, 56)),
    )
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title(app.i18n.t("new_folder_title"))
            .title_style(
                Style::default()
                    .fg(Color::Rgb(
                        app.theme.primary.0,
                        app.theme.primary.1,
                        app.theme.primary.2,
                    ))
                    .add_modifier(Modifier::BOLD),
            )
            .border_style(Style::default().fg(Color::Rgb(
                app.theme.success.0,
                app.theme.success.1,
                app.theme.success.2,
            ))),
    );

    f.render_widget(input, area);
}

fn render_renaming_file_modal(f: &mut Frame, app: &App) {
    let area = centered_rect(50, 15, f.area());
    f.render_widget(Clear, area);

    let input = Paragraph::new(format!(
        "{}\n\n{}_",
        app.i18n.t("enter_new_name"),
        app.rename_buffer
    ))
    .style(
        Style::default()
            .fg(Color::Rgb(255, 255, 255))
            .bg(Color::Rgb(38, 50, 56)),
    )
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title(app.i18n.t("rename_title"))
            .title_style(
                Style::default()
                    .fg(Color::Rgb(
                        app.theme.primary.0,
                        app.theme.primary.1,
                        app.theme.primary.2,
                    ))
                    .add_modifier(Modifier::BOLD),
            )
            .border_style(Style::default().fg(Color::Rgb(
                app.theme.warning.0,
                app.theme.warning.1,
                app.theme.warning.2,
            ))),
    );

    f.render_widget(input, area);
}

fn render_confirm_file_deletion_modal(f: &mut Frame, app: &App) {
    let area = centered_rect(50, 20, f.area());
    f.render_widget(Clear, area);

    let file_name = if let Some(idx) = app.file_list_state.selected() {
        app.files
            .get(idx)
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
            .unwrap_or("?")
    } else {
        "?"
    };

    let text = vec![
        Line::from(""),
        Line::from(Span::styled(
            format!("{} '{}'?", app.i18n.t("delete_file_question"), file_name),
            Style::default()
                .fg(Color::Rgb(255, 255, 255))
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(Span::styled(
            app.i18n.t("action_irreversible"),
            Style::default().fg(Color::Rgb(239, 83, 80)),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "[y] ",
                Style::default()
                    .fg(Color::Rgb(239, 83, 80))
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(format!(
                "{}     ",
                if app.i18n.t("confirm_deletion_help").contains("Confirmar") {
                    "Confirmar"
                } else {
                    "Confirm"
                }
            )),
            Span::styled(
                "[n] ",
                Style::default()
                    .fg(Color::Rgb(102, 187, 106))
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(if app.i18n.t("cmd_cancel").contains("Cancelar") {
                "Cancelar"
            } else {
                "Cancel"
            }),
        ]),
    ];

    let confirm = Paragraph::new(text)
        .style(
            Style::default()
                .fg(Color::Rgb(255, 255, 255))
                .bg(Color::Rgb(38, 50, 56)),
        )
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(app.i18n.t("confirm_file_deletion"))
                .title_style(
                    Style::default()
                        .fg(Color::Rgb(239, 83, 80))
                        .add_modifier(Modifier::BOLD),
                )
                .border_style(Style::default().fg(Color::Rgb(239, 83, 80))),
        );

    f.render_widget(confirm, area);
}

fn render_creating_secret_file_modal(f: &mut Frame, app: &App) {
    let area = centered_rect(60, 25, f.area());
    f.render_widget(Clear, area);

    let formats = ["yaml", "json", "env", "ini"];
    let ext = formats[app.selected_format];

    let display_text = if app.new_file_name_buffer.is_empty() {
        format!("secrets.{}", ext)
    } else if app.new_file_name_buffer.starts_with('.')
        || app.new_file_name_buffer.ends_with(&format!(".{}", ext))
    {
        app.new_file_name_buffer.clone()
    } else {
        format!("{}.{}", app.new_file_name_buffer, ext)
    };

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Min(0),
        ])
        .split(area);

    let block = Block::default()
        .borders(Borders::ALL)
        .title("🔐 Crear archivo encriptado con SOPS")
        .title_style(
            Style::default()
                .fg(Color::Rgb(102, 187, 106))
                .add_modifier(Modifier::BOLD),
        )
        .border_style(Style::default().fg(Color::Rgb(102, 187, 106)))
        .style(Style::default().bg(Color::Rgb(38, 50, 56)));

    f.render_widget(block, area);

    let label = Paragraph::new(format!("Nombre del archivo (Enter para secrets.{})", ext))
        .style(Style::default().fg(Color::Rgb(189, 189, 189)));
    f.render_widget(label, chunks[0]);

    let input = Paragraph::new(app.new_file_name_buffer.as_str())
        .style(Style::default().fg(Color::Rgb(255, 255, 255)));
    f.render_widget(input, chunks[1]);

    let preview = Paragraph::new(format!("→ {}", display_text))
        .style(Style::default().fg(Color::Rgb(102, 187, 106)))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Rgb(66, 66, 66))),
        );
    f.render_widget(preview, chunks[2]);
}

fn render_selecting_file_format_modal(f: &mut Frame, app: &App) {
    let area = centered_rect(50, 35, f.area());
    f.render_widget(Clear, area);

    let formats = [
        (app.i18n.t("format_env"), "Environment variables (.env)"),
        (app.i18n.t("format_json"), "JSON configuration (.json)"),
        (app.i18n.t("format_yaml"), "YAML configuration (.yaml/.yml)"),
        (app.i18n.t("format_ini"), "INI configuration (.ini)"),
    ];

    let items: Vec<ListItem> = formats
        .iter()
        .enumerate()
        .map(|(i, (name, desc))| {
            let style = if i == app.selected_format {
                Style::default()
                    .fg(Color::Rgb(102, 187, 106))
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Rgb(189, 189, 189))
            };

            let content = vec![
                Line::from(vec![
                    Span::styled(
                        if i == app.selected_format {
                            "▶ "
                        } else {
                            "  "
                        },
                        style,
                    ),
                    Span::styled(*name, style),
                ]),
                Line::from(vec![
                    Span::styled("    ", style),
                    Span::styled(*desc, Style::default().fg(Color::Rgb(150, 150, 150))),
                ]),
            ];

            ListItem::new(content).style(style)
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("📄 Seleccionar formato de archivo para SOPS")
                .title_style(
                    Style::default()
                        .fg(Color::Rgb(102, 187, 106))
                        .add_modifier(Modifier::BOLD),
                )
                .border_style(Style::default().fg(Color::Rgb(102, 187, 106))),
        )
        .style(Style::default().bg(Color::Rgb(38, 50, 56)));

    f.render_widget(list, area);
}

fn render_selecting_sops_keys_modal(f: &mut Frame, app: &App) {
    let area = centered_rect(70, 60, f.area());
    f.render_widget(Clear, area);

    let items: Vec<ListItem> = app
        .age_keys
        .iter()
        .enumerate()
        .map(|(i, key)| {
            let checked = app.selected_sops_keys.get(i).copied().unwrap_or(false);
            let radio = if checked { "(•) " } else { "( ) " };
            let name = key.comment.as_deref().unwrap_or("Unnamed");
            let pub_key = key.public_key.as_ref().map(|k| &k[..16]).unwrap_or("???");

            let style = if Some(i) == app.key_list_state.selected() {
                Style::default()
                    .fg(Color::Rgb(102, 187, 106))
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Rgb(189, 189, 189))
            };

            ListItem::new(format!("{}{} ({}...)", radio, name, pub_key)).style(style)
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(app.i18n.t("select_sops_keys_title"))
                .title_style(
                    Style::default()
                        .fg(Color::Rgb(102, 187, 106))
                        .add_modifier(Modifier::BOLD),
                )
                .border_style(Style::default().fg(Color::Rgb(102, 187, 106))),
        )
        .style(Style::default().bg(Color::Rgb(38, 50, 56)))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD));

    f.render_stateful_widget(list, area, &mut app.key_list_state.clone());
}

pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

fn render_editing_sops_config_modal(f: &mut Frame, app: &App) {
    let area = centered_rect(80, 80, f.area());
    f.render_widget(Clear, area);

    let lines: Vec<Line> = app
        .edit_buffer
        .lines()
        .map(|line| Line::from(line.to_string()))
        .collect();

    let paragraph = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!(" {} .sops.yaml ", app.i18n.t("editing_sops")))
                .title_style(
                    Style::default()
                        .fg(Color::Rgb(102, 187, 106))
                        .add_modifier(Modifier::BOLD),
                )
                .border_style(Style::default().fg(Color::Rgb(102, 187, 106))),
        )
        .style(
            Style::default()
                .bg(Color::Rgb(38, 50, 56))
                .fg(Color::Rgb(189, 189, 189)),
        );

    f.render_widget(paragraph, area);
}

fn render_selecting_sops_template_modal(f: &mut Frame, app: &App) {
    let area = centered_rect(70, 50, f.area());
    f.render_widget(Clear, area);

    let templates = [
        (
            app.i18n.t("template_simple"),
            app.i18n.t("template_simple_desc"),
        ),
        (
            app.i18n.t("template_by_type"),
            app.i18n.t("template_by_type_desc"),
        ),
        (
            app.i18n.t("template_regex"),
            app.i18n.t("template_regex_desc"),
        ),
        (app.i18n.t("template_k8s"), app.i18n.t("template_k8s_desc")),
    ];

    let items: Vec<ListItem> = templates
        .iter()
        .enumerate()
        .map(|(i, (name, desc))| {
            let style = if i == app.selected_sops_template {
                Style::default()
                    .fg(Color::Rgb(102, 187, 106))
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Rgb(189, 189, 189))
            };

            let content = vec![
                Line::from(vec![
                    Span::styled(
                        if i == app.selected_sops_template {
                            "▶ "
                        } else {
                            "  "
                        },
                        style,
                    ),
                    Span::styled(*name, style),
                ]),
                Line::from(vec![
                    Span::styled("    ", style),
                    Span::styled(*desc, Style::default().fg(Color::Rgb(150, 150, 150))),
                ]),
            ];

            ListItem::new(content).style(style)
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(app.i18n.t("select_template_title"))
                .title_style(
                    Style::default()
                        .fg(Color::Rgb(102, 187, 106))
                        .add_modifier(Modifier::BOLD),
                )
                .border_style(Style::default().fg(Color::Rgb(102, 187, 106))),
        )
        .style(Style::default().bg(Color::Rgb(38, 50, 56)));

    f.render_widget(list, area);
}
