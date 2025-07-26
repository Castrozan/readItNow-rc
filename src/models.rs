use serde::{Deserialize, Serialize};
use regex::Regex;
use std::fs;
use std::io;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    pub title: String,
    pub excerpt: String,
    pub tags: Vec<String>,
    pub url: Option<String>,
    pub thumbnail: Option<String>,
    pub read: bool,
}

impl Default for Note {
    fn default() -> Self {
        Note {
            title: "Untitled".to_string(),
            excerpt: "No content available".to_string(),
            tags: Vec::new(),
            url: None,
            thumbnail: None,
            read: false,
        }
    }
}

impl Note {
    pub fn from_markdown(content: &str, filename: &str, excerpt_lines: usize) -> Self {
        let mut note = Note::default();

        // Title from filename
        note.title = filename.replace(".md", "");

        // Excerpt
        let mut lines = content.lines().filter(|l| !l.trim().is_empty());
        note.excerpt = lines.by_ref().take(excerpt_lines).collect::<Vec<&str>>().join("\n");
        if note.excerpt.is_empty() {
            note.excerpt = "No content available".to_string();
        }

        // Tags
        let tag_re = Regex::new(r"\[\[([^\]]+)\]\]").unwrap();
        note.tags = tag_re.captures_iter(content)
            .map(|cap| cap[1].to_string())
            .collect();

        // URL
        let url_re = Regex::new(r"\[[^\]]+\]\(([^)]+)\)").unwrap();
        if let Some(cap) = url_re.captures(content) {
            note.url = Some(cap[1].to_string());
        }

        // Thumbnail Detection
        if let Some(url) = &note.url {
            if url.contains("twitter.com") || url.contains("t.co") {
                // For Twitter, we'll just use a placeholder for now, actual image fetching will be complex
                note.thumbnail = Some("twitter_placeholder.png".to_string());
            } else if url.contains("youtube.com") || url.contains("youtu.be") {
                let youtube_re = Regex::new(r"(?:https?://)?(?:www\.)?(?:m\.)?(?:youtube\.com|youtu\.be)/(?:watch\?v=|embed/|v/|)([^\s&]+)").unwrap();
                if let Some(cap) = youtube_re.captures(url) {
                    if let Some(video_id) = cap.get(1) {
                        note.thumbnail = Some(format!("https://img.youtube.com/vi/{}/mqdefault.jpg", video_id.as_str()));
                    }
                }
            }
        }

        // Read status
        if content.contains("[[readitnow/read]]") {
            note.read = true;
        }

        note
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub vault_path: String,
    pub max_notes: usize,
    pub excerpt_lines: usize,
    pub keybindings: Keybindings,
    pub thumbnail_cache: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Keybindings {
    pub open_link: String,
    pub open_file: String,
    pub up: String,
    pub down: String,
    pub left: String,
    pub right: String,
    pub page_up: String,
    pub page_down: String,
    pub quit: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            vault_path: "/home/zanoni/vault/ReadItLater Inbox".to_string(),
            max_notes: 20,
            excerpt_lines: 5,
            keybindings: Keybindings {
                open_link: "enter".to_string(),
                open_file: "shift+enter".to_string(),
                up: "up".to_string(),
                down: "down".to_string(),
                left: "left".to_string(),
                right: "right".to_string(),
                page_up: "pageup".to_string(),
                page_down: "pagedown".to_string(),
                quit: "q".to_string(),
            },
            thumbnail_cache: "~/.cache/readitnow/thumbnails".to_string(),
        }
    }
}

impl Config {
    pub fn load<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let config_str = fs::read_to_string(path)?;
        serde_yaml::from_str(&config_str)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        let config_str = serde_yaml::to_string(&self)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        fs::write(path, config_str)
    }
}


