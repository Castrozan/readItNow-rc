use ratatui::{backend::TestBackend, Terminal};

use readitnow::app::App;
use readitnow::models::Note;

#[test]
fn test_app_navigation() {
    let mock_notes = vec![
        Note::default(),
        Note::default(),
        Note::default(),
    ];
    let mut app = App::new(mock_notes);

    assert_eq!(app.selected_note_index, 0);

    app.next_note();
    assert_eq!(app.selected_note_index, 1);

    app.next_note();
    assert_eq!(app.selected_note_index, 2);

    app.next_note();
    assert_eq!(app.selected_note_index, 0);

    app.previous_note();
    assert_eq!(app.selected_note_index, 2);

    app.previous_note();
    assert_eq!(app.selected_note_index, 1);

    app.next_page();
    assert_eq!(app.selected_note_index, 1);

    app.previous_page();
    assert_eq!(app.selected_note_index, 1);
}

#[test]
fn test_ui_rendering() {
    let backend = TestBackend::new(100, 50);
    let mut terminal = Terminal::new(backend).unwrap();

    let mock_notes = vec![
        Note {
            title: "Test Note 1".to_string(),
            excerpt: "Excerpt 1".to_string(),
            ..Default::default()
        },
        Note {
            title: "Test Note 2".to_string(),
            excerpt: "Excerpt 2".to_string(),
            ..Default::default()
        },
    ];
    let mut app = App::new(mock_notes);

    terminal.draw(|frame| {
        let mut renderer = readitnow::ui::Renderer::new();
        renderer.render_app(&mut app, frame);
    }).unwrap();
}

