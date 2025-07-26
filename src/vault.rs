use std::{fs, io, path::{Path, PathBuf}};
use crate::models::{Note, Config};
use reqwest::blocking::get;
use image::{ImageOutputFormat,io::Reader};

pub fn download_and_cache_thumbnail(url: &str, cache_dir: &Path) -> io::Result<String> {
    let response = get(url).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    let bytes = response.bytes().map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    let img = Reader::new(io::Cursor::new(bytes))
        .with_guessed_format()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?
        .decode()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        
    fs::create_dir_all(cache_dir)?;

    let file_name = format!("{:x}.jpeg", md5::compute(url));
    let file_path = cache_dir.join(file_name);
    
    let mut file = fs::File::create(&file_path)?;
    img.write_to(&mut file, ImageOutputFormat::Jpeg(80))
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    Ok(file_path.to_string_lossy().to_string())
}

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
            notes.push(Note::from_markdown(&content, &filename, config.excerpt_lines, config));
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


