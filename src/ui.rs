use ratatui::{prelude::*, widgets::{block::*, Paragraph, Borders}};
use crate::models::Note;

pub fn render_note_card(buf: &mut Buffer, area: Rect, note: &Note, is_selected: bool) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(note.title.as_str())
        .border_style(if is_selected { Style::default().fg(Color::Yellow) } else { Style::default() });
    
    let inner_area = block.inner(area);
    block.render(area, buf);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // For thumbnail/placeholder
            Constraint::Min(0),    // For excerpt
        ])
        .split(inner_area);

    // Thumbnail or Placeholder
    if note.thumbnail.is_some() {
        Paragraph::new("📷 Thumbnail").render(chunks[0], buf);
    } else {
        Paragraph::new("📄 No Thumbnail").render(chunks[0], buf);
    }

    // Excerpt
    Paragraph::new(note.excerpt.as_str()).render(chunks[1], buf);
}

pub struct NoteList<'a> {
    notes: &'a [Note],
    selected_note_index: usize,
    scroll_state: u16,
}

impl<'a> NoteList<'a> {
    pub fn new(notes: &'a [Note], selected_note_index: usize) -> Self {
        Self { notes, selected_note_index, scroll_state: 0 }
    }

    pub fn scroll(&mut self, delta: i16) {
        self.scroll_state = self.scroll_state.saturating_add_signed(delta);
    }
}

impl<'a> Widget for NoteList<'a> {
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
            render_note_card(buf, card_area, note, is_selected);
        }
    }
}

