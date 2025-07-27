use crate::app::App;
use crate::config::Config;
use crate::vault;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use open;

#[derive(PartialEq)]
pub enum AppAction {
    Continue,
    Quit,
}

// TODO: use the keybindings from the config
pub fn handle_key_event(key: KeyEvent, app: &mut App, config: &Config) -> AppAction {
    if key.kind == KeyEventKind::Press {
        match key.code {
            KeyCode::Char(c) if c.to_string() == config.keybindings.quit => {
                return AppAction::Quit;
            }
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
                    if let Some(note) = app.selected_note() {
                        let _ = open::that(format!("{}/{}.md", config.vault_path, note.title));
                    }
                } else if let Some(note) = app.selected_note() {
                    if let Some(url) = &note.url {
                        let _ = open::that(url);
                    }
                }
            }
            KeyCode::Char(c) if c.to_string() == "r" => {
                // TODO: test this
                if let Some(note) = app.selected_note_mut() {
                    let _ = vault::toggle_read_status(note, config);
                }
            }
            _ => {}
        }
    }

    AppAction::Continue
} 