use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    cursor::{SetCursorStyle, Show},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{env, io::stdout, path::PathBuf, time::Duration};

mod config;
mod events;
mod generator;
mod help;
mod i18n;
mod sops;
mod state;
mod ui;

use config::{load_config, load_favorites, save_favorites};
use events::handle_key_event;
use sops::load_age_keys;
use state::App;
use ui::ui;

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let start_dir = if args.len() >= 2 {
        PathBuf::from(&args[1])
    } else {
        env::current_dir()?
    };

    if !start_dir.exists() {
        anyhow::bail!("El directorio {} no existe", start_dir.display());
    }

    let start_dir = if start_dir.is_file() {
        start_dir.parent().unwrap_or(&start_dir).to_path_buf()
    } else {
        start_dir
    };

    let age_keys = load_age_keys()?;
    if age_keys.is_empty() {
        eprintln!("Advertencia: No se encontraron llaves en ~/.config/sops/age/keys.txt");
    }

    let config = load_config()?;
    let favorites = load_favorites().unwrap_or_default();
    let mut app = App::new(start_dir, config, age_keys, favorites)?;

    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, Show, SetCursorStyle::BlinkingBar)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal, &mut app).await;

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = &res {
        eprintln!("Error: {:?}", err);
    }

    save_favorites(&app.favorites)?;

    res
}

async fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;
        app.clear_expired_message();

        // Check auto-lock
        if app.check_auto_lock() {
            app.lock();
            app.set_temp_message(app.i18n.t("session_locked").to_string());
        }

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Press {
                    continue;
                }

                if handle_key_event(app, key)? {
                    return Ok(());
                }
            }
        }
    }
}
