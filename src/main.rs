mod models;
mod ui;
mod app;
mod vault;
mod kitty;

use std::{io, time::Duration};
use crossterm::{event::{self, Event, KeyCode, KeyEventKind, KeyModifiers}, terminal::{self, EnterAlternateScreen, LeaveAlternateScreen}, ExecutableCommand};
use ratatui::{prelude::*, widgets::{block::*, Paragraph}};
use crate::models::{Note, Config};
use crate::ui::{NoteList, render_note_card};
use crate::app::App;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup terminal
    let mut terminal = setup_terminal()?;

    // Load config
    let config_path = "/home/zanoni/.config/readitnow/config.yaml"; // TODO: Make this configurable
    let config = Config::load(config_path).unwrap_or_else(|_| {
        let default_config = Config::default();
        default_config.save(config_path).expect("Failed to save default config");
        default_config
    });

    // Load notes from the vault
    let notes = vault::scan_vault(&config).expect("Failed to load notes from the vault");

    let mut app = App::new(notes);

    // Application loop
    loop {
        terminal.draw(|frame| {
            let area = frame.size();
            
            // Render notes in a grid layout
            let num_cols = 2;
            let card_height = 10;
            let visible_rows = area.height / card_height;
            let visible_notes_count = (visible_rows as usize * num_cols).min(app.notes.len());
            let notes_to_render = &app.notes[0..visible_notes_count];

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
        })?;

        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char(c) if c.to_string() == config.keybindings.quit => break,
                        KeyCode::Char(c) if c.to_string() == "j" => app.next_two_notes(),
                        KeyCode::Char(c) if c.to_string() == "k" => app.previous_two_notes(),
                        KeyCode::Char(c) if c.to_string() == "h" => app.previous_note(),
                        KeyCode::Char(c) if c.to_string() == "l" => app.next_note(),
                        KeyCode::Up => app.previous_two_notes(),
                        KeyCode::Down => app.next_two_notes(),
                        KeyCode::Left => app.previous_note(),
                        KeyCode::Right => app.next_note(),
                        KeyCode::PageDown => app.next_page(),
                        KeyCode::PageUp => app.previous_page(),
                        KeyCode::Enter => {
                            if key.modifiers.contains(KeyModifiers::SHIFT) {
                                // Shift+Enter: Open file
                                // TODO: open file in default editor, now its opening in kitty
                                if let Some(note) = app.notes.get(app.selected_note_index) {
                                    let _ = open::that(format!("{}/{}.md", config.vault_path, note.title));
                                }
                            } else {
                                if let Some(note) = app.notes.get(app.selected_note_index) {
                                    if let Some(url) = &note.url {
                                        let _ = open::that(url);
                                    }
                                }
                            }
                        },
                        KeyCode::Char(c) if c.to_string() == "r" => {
                            // TODO: test this
                            if let Some(note) = app.notes.get_mut(app.selected_note_index) {
                                let _ = vault::toggle_read_status(note, &config);
                            }
                        },
                        _ => {},
                    }
                }
            }
        }
    }

    // Restore terminal
    restore_terminal()?;
    Ok(())
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
