use crate::app::App;
use ratatui::{prelude::*, widgets::{block::*, Borders, Paragraph, Wrap}};
use crate::models::Note;
use ratatui_image::{picker::Picker, StatefulImage, protocol::StatefulProtocol};
use std::path::PathBuf;
use std::collections::HashMap;

pub struct Renderer {}

impl Renderer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn render_app(&mut self, app: &mut App, frame: &mut Frame<'_>) {
        let area = frame.area();
        // TODO: make this configurable
        let num_cols = 2;
        let num_rows = 2;
        let card_height = 10;
        let notes_to_render = app.notes_on_current_page().to_vec();
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ])
            .split(area);
        for (i, note) in notes_to_render.iter().enumerate().take(num_cols * num_rows) {
            let col = i % num_cols;
            let row = i / num_cols;

            let card_area = Rect::new(
                chunks[col].x,
                chunks[col].y + (row as u16 * card_height),
                chunks[col].width,
                card_height,
            );

            let is_selected = i == app.selected_note_index;
            self.render_note_card(frame, card_area, note, is_selected, &mut app.image_cache);
        }
    }

    fn render_note_card(
        &mut self,
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
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50),
                Constraint::Percentage(50),
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
                    }
                    Err(_) => {
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
        let excerpt = Paragraph::new(note.excerpt.as_str()).wrap(Wrap { trim: true });
        frame.render_widget(excerpt, chunks[1]);
    }
}
