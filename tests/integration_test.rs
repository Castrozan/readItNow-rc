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

    // Test initial state
    assert_eq!(app.selected_note_index, 0);

    // Test next_note
    app.next_note();
    assert_eq!(app.selected_note_index, 1);

    app.next_note();
    assert_eq!(app.selected_note_index, 2);

    app.next_note(); // Should wrap around
    assert_eq!(app.selected_note_index, 0);

    // Test previous_note
    app.previous_note();
    assert_eq!(app.selected_note_index, 2);

    app.previous_note();
    assert_eq!(app.selected_note_index, 1);

    // Test next_page
    app.next_page();
    assert_eq!(app.selected_note_index, 2); // Should be 2 as there are only 3 notes

    // Test previous_page
    app.previous_page();
    assert_eq!(app.selected_note_index, 0);
}

// This test is more complex and would require mocking the terminal and event loop
// For now, we'll keep it as a placeholder.
#[test]
fn test_ui_rendering() {
    // Create a dummy backend for testing
    let backend = TestBackend::new(100, 50); // 100 columns, 50 rows
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
    let app = App::new(mock_notes);

    terminal.draw(|frame| {
        let area = frame.size();
        let note_list = readitnow::ui::NoteList::new(&app.notes, app.selected_note_index);
        frame.render_widget(note_list, area);
    }).unwrap();

    // You can inspect the buffer here to assert on rendered content
    // For example, check if titles are present at expected locations
    let buffer = terminal.backend().buffer();
    // Check if the buffer contains the note titles somewhere
    let buffer_content = buffer.content();
    let has_note1 = buffer_content.iter().any(|cell| cell.symbol().contains("Test Note 1"));
    let has_note2 = buffer_content.iter().any(|cell| cell.symbol().contains("Test Note 2"));
    
    assert!(has_note1, "Test Note 1 should be rendered");
    assert!(has_note2, "Test Note 2 should be rendered");
}

