use readitnow::models::Note;
use readitnow::config::Config;

#[test]
fn test_note_from_markdown() {
    let markdown_content = r#"
[[ReadItLater]] [[Tweet]]

# [Aadit Sheth](https://twitter.com/aaditsh/status/1909332848152105301)

> This guy literally turned WhatsApp into an AI assistant using Claude and ElevenLabs[pic.twitter.com/f77uIBIQkj](https://t.co/f77uIBIQkj)
> 
> — Aadit Sheth (@aaditsh) [April 7, 2025](https://twitter.com/aaditsh/status/1909332848152105301?ref_src=twsrc%5Etfw)
"#;
    let filename = "Aadit Sheth.md";
    let note = Note::from_markdown(markdown_content, filename, 5, &Config::default());

    assert_eq!(note.title, "Aadit Sheth");
    assert!(note.excerpt.contains("This guy literally turned WhatsApp into an AI assistant"));
    assert!(note.tags.contains(&"ReadItLater".to_string()));
    assert!(note.tags.contains(&"Tweet".to_string()));
    assert_eq!(note.url, Some("https://twitter.com/aaditsh/status/1909332848152105301".to_string()));
    assert!(!note.read);
}

#[test]
fn test_config_load_save() {
    let test_config_path = "/tmp/test_config.yaml";
    let original_config = Config::default();
    original_config.save(test_config_path).unwrap();

    let loaded_config = Config::load(test_config_path).unwrap();

    assert_eq!(original_config.vault_path, loaded_config.vault_path);
    assert_eq!(original_config.max_notes, loaded_config.max_notes);
    assert_eq!(original_config.excerpt_lines, loaded_config.excerpt_lines);
    assert_eq!(original_config.keybindings.quit, loaded_config.keybindings.quit);

    std::fs::remove_file(test_config_path).unwrap();
}


