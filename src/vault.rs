use std::{fs, io, path::{Path, PathBuf}, time::SystemTime};
use crate::models::Note;
use crate::config::Config;
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

    if !vault_path.exists() || !vault_path.is_dir() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "Vault path does not exist or is not a directory"));
    }

    let mut files_with_mod_time: Vec<(PathBuf, SystemTime)> = fs::read_dir(&vault_path)?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.is_file() && path.extension().map_or(false, |ext| ext == "md"))
        .filter_map(|path| fs::metadata(&path).ok().and_then(|meta| meta.modified().ok()).map(|mod_time| (path, mod_time)))
        .collect();

    files_with_mod_time.sort_by(|a, b| b.1.cmp(&a.1));

    let notes: Vec<Note> = files_with_mod_time.into_iter()
    // TODO: this is limiting the amount of notes to be loaded
    // for some reason loading all notes is very slow
    // we can load only the 4 notes at a time and then load the next 4
        .take(config.max_notes)
        .filter_map(|(path, _)| {
            let content = fs::read_to_string(&path).ok()?;
            let filename = path.file_stem().and_then(|s| s.to_str()).unwrap_or("Untitled").to_string();
            Some(Note::from_markdown(&content, &filename, config.excerpt_lines, config))
        })
        .collect();
    
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


