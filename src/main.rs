mod models;
mod ui;
mod app;
mod vault;
mod keybindings;

use std::{io, time::Duration};
use crossterm::{event::{self, Event}, terminal::{self, EnterAlternateScreen, LeaveAlternateScreen}, ExecutableCommand};
use ratatui::prelude::*;
use crate::models::Config;
use crate::ui::render_note_card;
use crate::app::App;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut terminal = setup_terminal()?;

    let config = load_config();

    let notes = vault::scan_vault(&config).expect("Failed to load notes from the vault");

    let mut app = App::new(notes);

    // Application loop
    loop {
        terminal.draw(|frame| {
            render_app(&mut app, frame);
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

fn render_app(app: &mut App, frame: &mut Frame<'_>) {
    let area = frame.area();
    // TODO: make this configurable
    let num_cols = 2;
    let card_height = 10;
    let notes_to_render = app.notes_on_current_page().to_vec();
    let chunks = Layout::default()
         .direction(Direction::Horizontal)
         .constraints([
             Constraint::Percentage(50),
             Constraint::Percentage(50),
         ])
         .split(area);
    for (i, note) in notes_to_render.iter().enumerate() {
         let col = i % num_cols;
         let row = i / num_cols;
 
         let card_area = Rect::new(
             chunks[col].x,
             chunks[col].y + (row as u16 * card_height),
             chunks[col].width,
             card_height,
         );
 
         let is_selected = i == app.selected_note_index;
         render_note_card(frame, card_area, note, is_selected, &mut app.image_cache);
     }
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
