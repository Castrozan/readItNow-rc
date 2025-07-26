use ratatui::{prelude::*, widgets::{block::*, Borders, Paragraph}};
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
