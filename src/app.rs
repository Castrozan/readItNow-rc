use crate::models::Note;

pub struct App {
    pub notes: Vec<Note>,
    pub selected_note_index: usize,
}

impl App {
    pub fn new(notes: Vec<Note>) -> Self {
        App {
            notes,
            selected_note_index: 0,
        }
    }

    pub fn next_note(&mut self) {
        if !self.notes.is_empty() {
            self.selected_note_index = (self.selected_note_index + 1) % self.notes.len();
        }
    }

    pub fn previous_note(&mut self) {
        if !self.notes.is_empty() {
            self.selected_note_index = (self.selected_note_index + self.notes.len() - 1) % self.notes.len();
        }
    }

    pub fn next_page(&mut self) {
        // This needs to be dynamic based on visible notes, for now a fixed jump
        self.selected_note_index = (self.selected_note_index + 4).min(self.notes.len() - 1);
    }

    pub fn previous_page(&mut self) {
        // This needs to be dynamic based on visible notes, for now a fixed jump
        self.selected_note_index = self.selected_note_index.saturating_sub(4);
    }
}


