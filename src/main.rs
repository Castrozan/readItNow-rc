mod models;
mod ui;
mod app;
mod vault;
mod kitty;

use std::{io, time::Duration};
use crossterm::{event::{self, Event, KeyCode, KeyEventKind, KeyModifiers}, terminal::{self, EnterAlternateScreen, LeaveAlternateScreen}, ExecutableCommand};
use ratatui::{prelude::*, widgets::{block::*, Paragraph}};
use crate::models::{Note, Config};
use crate::ui::NoteList;
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

    // Mock notes for testing
    let mock_notes = vec![
        Note {
            title: "My First Note".to_string(),
            excerpt: "This is the first note excerpt. It has some content to display.".to_string(),
            tags: vec!["rust".to_string(), "ratatui".to_string()],
            url: Some("https://example.com/first".to_string()),
            thumbnail: None,
            read: false,
        },
        Note {
            title: "Another Interesting Read".to_string(),
            excerpt: "Here\"s another note with a slightly longer excerpt to see how it wraps.".to_string(),
            tags: vec!["programming".to_string()],
            url: Some("https://example.com/second".to_string()),
            thumbnail: None,
            read: true,
        },
        Note {
            title: "A Third Note".to_string(),
            excerpt: "Short and sweet.".to_string(),
            tags: vec!["quick_read".to_string()],
            url: None,
            thumbnail: None,
            read: false,
        },
        Note {
            title: "Fourth Note".to_string(),
            excerpt: "This is the fourth note excerpt.".to_string(),
            tags: vec!["test".to_string()],
            url: None,
            thumbnail: None,
            read: false,
        },
        Note {
            title: "Fifth Note".to_string(),
            excerpt: "This is the fifth note excerpt.".to_string(),
            tags: vec!["example".to_string()],
            url: None,
            thumbnail: None,
            read: false,
        },
        Note {
            title: "Sixth Note".to_string(),
            excerpt: "This is the sixth note excerpt.".to_string(),
            tags: vec!["demo".to_string()],
            url: None,
            thumbnail: None,
            read: false,
        },
        Note {
            title: "Seventh Note".to_string(),
            excerpt: "This is the seventh note excerpt.".to_string(),
            tags: vec!["new".to_string()],
            url: None,
            thumbnail: None,
            read: false,
        },
        Note {
            title: "Eighth Note".to_string(),
            excerpt: "This is the eighth note excerpt.".to_string(),
            tags: vec!["latest".to_string()],
            url: None,
            thumbnail: None,
            read: false,
        },
    ];

    let mut app = App::new(mock_notes);

    // Application loop
    loop {
        terminal.draw(|frame| {
            let area = frame.size();
            let note_list = NoteList::new(&app.notes, app.selected_note_index);
            frame.render_widget(note_list, area);
        })?;

        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char(c) if c.to_string() == config.keybindings.quit => break,
                        KeyCode::Char(c) if c.to_string() == "j" => app.next_note(),
                        KeyCode::Char(c) if c.to_string() == "k" => app.previous_note(),
                        KeyCode::Char(c) if c.to_string() == "h" => app.previous_note(), // For now, left/right act as up/down
                        KeyCode::Char(c) if c.to_string() == "l" => app.next_note(), // For now, left/right act as up/down
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


