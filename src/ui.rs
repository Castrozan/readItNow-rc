use ratatui::{prelude::*, widgets::{block::*, Paragraph, Borders}};
use crate::models::Note;
use ratatui_image::{picker::Picker, StatefulImage, protocol::StatefulProtocol};
use std::path::PathBuf;
use std::collections::HashMap;

pub fn render_note_card(
    frame: &mut Frame,
    area: Rect,
    note: &Note,
    is_selected: bool,
    image_cache: &mut HashMap<PathBuf, Box<dyn StatefulProtocol>>,
) {
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

        if image_cache.contains_key(&image_path) {
            let image_protocol = image_cache.get_mut(&image_path).unwrap();
            let image_widget = StatefulImage::new(None);
            frame.render_stateful_widget(image_widget, chunks[0], image_protocol);
        } else if image_path.exists() {
            // TODO: the image should be reloaded on screen update, not on every render
            match image::open(&image_path) {
                Ok(dyn_img) => {
                    let mut picker = Picker::new((8, 12));
                    let mut image_protocol = picker.new_resize_protocol(dyn_img);
                    let image_widget = StatefulImage::new(None);
                    frame.render_stateful_widget(image_widget, chunks[0], &mut image_protocol);
                    image_cache.insert(image_path, image_protocol);
                    println!("Image loaded and cached:");
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
    fn render(self, area: Rect, _buf: &mut Buffer) {
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

        for (i, _note) in notes_to_render.iter().enumerate() {
            let col = i % num_cols;
            let row = i / num_cols;

            let _card_area = Rect::new(
                chunks[col].x,
                chunks[col].y + (row as u16 * card_height),
                chunks[col].width,
                card_height,
            );

            let _is_selected = i == self.selected_note_index;
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
