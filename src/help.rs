use crate::i18n::I18n;

pub fn show_help(i18n: &I18n) -> Vec<(String, Vec<(&'static str, String)>)> {
    vec![
        (i18n.t("help_explorer").to_string(), vec![
            ("↑/↓", i18n.t("help_nav_files").to_string()),
            ("Enter", i18n.t("help_open_dir").to_string()),
            ("m", i18n.t("help_mark").to_string()),
            ("i", i18n.t("help_init_sops").to_string()),
            ("Tab", i18n.t("help_change_secrets").to_string()),
            ("k", i18n.t("help_key_selector_open").to_string()),
        ]),
        (i18n.t("help_secrets").to_string(), vec![
            ("↑/↓", i18n.t("help_nav_secrets").to_string()),
            ("v", i18n.t("help_show_hide").to_string()),
            ("z", i18n.t("help_zoom").to_string()),
            ("c", i18n.t("help_copy_value").to_string()),
            ("C", i18n.t("help_copy_key").to_string()),
            ("f", i18n.t("help_favorite").to_string()),
            ("e", i18n.t("help_edit").to_string()),
            ("n", i18n.t("help_new").to_string()),
            ("d", i18n.t("help_delete").to_string()),
            ("s", i18n.t("help_save").to_string()),
            ("g", i18n.t("help_generate").to_string()),
            ("/", i18n.t("help_search_secrets").to_string()),
            ("k", i18n.t("help_key_selector_open").to_string()),
            ("Tab", i18n.t("help_back_explorer").to_string()),
        ]),
        (i18n.t("help_key_selector").to_string(), vec![
            ("↑/↓", i18n.t("help_nav_keys").to_string()),
            ("/", i18n.t("help_search_keys").to_string()),
            ("Enter", i18n.t("help_apply_key").to_string()),
            ("Esc", i18n.t("help_cancel").to_string()),
        ]),
        (i18n.t("help_search").to_string(), vec![
            (if i18n.t("help_type") == "Type" { "Type" } else { "Escribir" }, i18n.t("help_filter").to_string()),
            ("Enter", i18n.t("help_apply_filter").to_string()),
            ("Esc", i18n.t("help_cancel_search").to_string()),
        ]),
        (i18n.t("help_general").to_string(), vec![
            ("?", i18n.t("help_show_help").to_string()),
            ("q", i18n.t("help_quit").to_string()),
        ]),
    ]
}
