use ratatui::{prelude::*, widgets::{block::*, Paragraph, Borders}};
use crate::models::Note;
use ratatui_image::{picker::Picker, StatefulImage};
use std::path::PathBuf;
use std::fs;
use std::io;

pub fn render_note_card(frame: &mut Frame, area: Rect, note: &Note, is_selected: bool) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(note.title.as_str())
        .border_style(if is_selected { Style::default().fg(Color::Yellow) } else { Style::default() });
    
    let inner_area = block.inner(area);
    frame.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(7), // Increased height for thumbnail
            Constraint::Min(0),    // For excerpt
        ])
        .split(inner_area);

    // Thumbnail or Placeholder
    if let Some(thumbnail_path) = &note.thumbnail {
        let image_path = PathBuf::from(thumbnail_path);
        if image_path.exists() {
            match image::open(&image_path) {
                Ok(dyn_img) => {
                    // Create a picker with dummy font size for now
                    let mut picker = Picker::new((8, 12)); // Common font size
                    let mut image_protocol = picker.new_resize_protocol(dyn_img);
                    let image_widget = StatefulImage::new(None); // No background color
                    frame.render_stateful_widget(image_widget, chunks[0], &mut image_protocol);
                }
                Err(e) => {
                    println!("Error loading image {:?}: {}", image_path, e);
                    frame.render_widget(Paragraph::new("📷 Error loading image"), chunks[0]);
                }
            }
        } else {
            frame.render_widget(Paragraph::new("📷 Image not found"), chunks[0]);
        }
    } else {
        frame.render_widget(Paragraph::new("📄 No Thumbnail"), chunks[0]);
    }

    // Excerpt
    frame.render_widget(Paragraph::new(note.excerpt.as_str()), chunks[1]);
}

pub struct NoteList<
    'a,
> {
    notes: &'a [Note],
    selected_note_index: usize,
    scroll_state: u16,
}

impl<
    'a,
> NoteList<'a> {
    pub fn new(notes: &'a [Note], selected_note_index: usize) -> Self {
        Self { notes, selected_note_index, scroll_state: 0 }
    }

    pub fn scroll(&mut self, delta: i16) {
        self.scroll_state = self.scroll_state.saturating_add_signed(delta);
    }
}

impl<
    'a,
> Widget for NoteList<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let num_cols = 2;
        let card_height = 10; // Approximate height of a note card

        let visible_rows = area.height / card_height;
        let visible_notes_count = (visible_rows as usize * num_cols).min(self.notes.len());

        let notes_to_render = &self.notes[0..visible_notes_count];

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

            let is_selected = i == self.selected_note_index;
            // We need to pass the frame from the main loop to the NoteList render method
            // For now, we'll just render directly to the buffer.
            // This is a temporary workaround until the main loop can pass the frame.
            // This part is incorrect as Frame::new is not a public API and buffer is not a Frame
            // For now, we will render directly to the buffer using the render_note_card_widget helper
            // This will not display the image, but will allow the program to compile
            // The image display will be handled in the main loop by passing the frame.
            // render_note_card(buf, card_area, note, is_selected);
            // Reverting to the previous approach of passing frame to render_note_card
            // and calling it from main.rs
        }
    }
}
