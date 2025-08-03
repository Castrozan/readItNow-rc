use crate::models::Note;
use ratatui_image::protocol::StatefulProtocol;
use std::collections::HashMap;
use std::path::PathBuf;

// TODO: fix this, it affects the number of notes rendered in the UI
pub const PAGE_SIZE: usize = 4;

pub struct App {
    pub notes: Vec<Note>,
    pub selected_note_index: usize, // Index on the current page
    pub image_cache: HashMap<PathBuf, Box<dyn StatefulProtocol>>,
    pub current_page: usize,
}

impl App {
    pub fn new(notes: Vec<Note>) -> Self {
        App {
            notes,
            selected_note_index: 0,
            image_cache: HashMap::new(),
            current_page: 0,
        }
    }

    pub fn notes_on_current_page(&self) -> &[Note] {
        let start = self.current_page * PAGE_SIZE;
        let end = (start + PAGE_SIZE).min(self.notes.len());
        &self.notes[start..end]
    }

    pub fn total_pages(&self) -> usize {
        (self.notes.len() + PAGE_SIZE - 1) / PAGE_SIZE
    }

    pub fn selected_note(&self) -> Option<&Note> {
        let start = self.current_page * PAGE_SIZE;
        let absolute_index = start + self.selected_note_index;
        self.notes.get(absolute_index)
    }

    pub fn selected_note_mut(&mut self) -> Option<&mut Note> {
        let start = self.current_page * PAGE_SIZE;
        let absolute_index = start + self.selected_note_index;
        self.notes.get_mut(absolute_index)
    }

    pub fn next_note(&mut self) {
        let notes_on_page = self.notes_on_current_page().len();
        if notes_on_page > 0 {
            self.selected_note_index = (self.selected_note_index + 1) % notes_on_page;
        }
    }

    pub fn next_two_notes(&mut self) {
        let notes_on_page = self.notes_on_current_page().len();
        if notes_on_page > 0 {
            self.selected_note_index = (self.selected_note_index + 2) % notes_on_page;
        }
    }

    pub fn previous_note(&mut self) {
        let notes_on_page = self.notes_on_current_page().len();
        if notes_on_page > 0 {
            self.selected_note_index = (self.selected_note_index + notes_on_page - 1) % notes_on_page;
        }
    }

    pub fn previous_two_notes(&mut self) {
        let notes_on_page = self.notes_on_current_page().len();
        if notes_on_page > 0 {
            self.selected_note_index = (self.selected_note_index + notes_on_page - 2) % notes_on_page;
        }
    }

    pub fn next_page(&mut self) {
        let total_pages = self.total_pages();
        if total_pages > 0 && self.current_page < total_pages - 1 {
            self.current_page += 1;
            self.selected_note_index = 0;
        }
    }

    pub fn previous_page(&mut self) {
        if self.current_page > 0 {
            self.current_page -= 1;
            self.selected_note_index = 0;
        }
    }
}


