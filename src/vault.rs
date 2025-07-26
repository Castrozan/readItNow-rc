use std::{fs, io, path::PathBuf};
use crate::models::{Note, Config};

pub fn scan_vault(config: &Config) -> io::Result<Vec<Note>> {
    let vault_path = PathBuf::from(&config.vault_path);
    let mut notes = Vec::new();

    if !vault_path.exists() || !vault_path.is_dir() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "Vault path does not exist or is not a directory"));
    }

    for entry in fs::read_dir(&vault_path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() && path.extension().map_or(false, |ext| ext == "md") {
            let content = fs::read_to_string(&path)?;
            let filename = path.file_stem().and_then(|s| s.to_str()).unwrap_or("Untitled").to_string();
            notes.push(Note::from_markdown(&content, &filename, config.excerpt_lines));
        }
    }

    // Sort notes by modification time (newest first)
    notes.sort_by(|a, b| {
        let a_path = vault_path.join(format!("{}.md", a.title));
        let b_path = vault_path.join(format!("{}.md", b.title));

        let a_modified = fs::metadata(&a_path).and_then(|m| m.modified()).ok();
        let b_modified = fs::metadata(&b_path).and_then(|m| m.modified()).ok();

        b_modified.cmp(&a_modified)
    });

    Ok(notes)
}




pub fn toggle_read_status(note: &mut Note, config: &Config) -> io::Result<()> {
    let note_path = PathBuf::from(&config.vault_path).join(format!("{}.md", note.title));
    let mut content = fs::read_to_string(&note_path)?;

    if note.read {
        // Mark as unread: remove tag
        content = content.replace("[[readitnow/read]]", "");
        note.read = false;
    } else {
        // Mark as read: add tag
        content.push_str("\n[[readitnow/read]]");
        note.read = true;
    }

    fs::write(&note_path, content)?; // Write back the modified content
    Ok(())
}


