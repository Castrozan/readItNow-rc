mod models;
mod ui;
mod app;
mod vault;
mod keybindings;

use std::{io, time::Duration};
use crossterm::{event::{self, Event}, terminal::{self, EnterAlternateScreen, LeaveAlternateScreen}, ExecutableCommand};
use ratatui::prelude::*;
use crate::models::Config;
use crate::ui::Renderer;
use crate::app::App;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut terminal = setup_terminal()?;

    let config = load_config();

    let notes = vault::scan_vault(&config).expect("Failed to load notes from the vault");

    let mut app = App::new(notes);

    let mut renderer = Renderer::new();

    // Application loop
    loop {
        terminal.draw(|frame| {
            renderer.render_app(&mut app, frame);
        })?;

        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                if keybindings::handle_key_event(key, &mut app, &config)
                    == keybindings::AppAction::Quit
                {
                    break;
                }
            }
        }
    }

    restore_terminal()?;
    Ok(())
}

fn load_config() -> Config {
    // TODO: look for the config on $XDG_CONFIG_HOME
    let config_path = "/home/zanoni/.config/readitnow/config.yaml";
    let config = Config::load(config_path).unwrap_or_else(|_| {
        let default_config = Config::default();
        default_config.save(config_path).expect("Failed to save default config");
        default_config
    });
    config
}

fn setup_terminal() -> Result<Terminal<impl Backend>, Box<dyn std::error::Error>> {
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    Ok(Terminal::new(CrosstermBackend::new(stdout))?)
}

fn restore_terminal() -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout = io::stdout();
    terminal::disable_raw_mode()?;
    stdout.execute(LeaveAlternateScreen)?;
    Ok(())
}
