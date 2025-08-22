use directories::{ProjectDirs, UserDirs};
use serde::{Deserialize, Serialize};
use serde_yaml;
use std::{fs, io, path::Path};

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
        let vault_path = if let Some(user_dirs) = UserDirs::new() {
            user_dirs.home_dir().join("vault").join("ReadItLater Inbox")
                .to_string_lossy().to_string()
        } else {
            "vault/ReadItLater Inbox".to_string()
        };

        let thumbnail_cache = if let Some(proj_dirs) = ProjectDirs::from("com", "readitnow", "readitnow") {
            proj_dirs.cache_dir().join("thumbnails")
                .to_string_lossy().to_string()
        } else {
            ".cache/readitnow/thumbnails".to_string()
        };

        Config {
            vault_path,
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
            thumbnail_cache,
        }
    }
}

impl Config {
    pub fn load_or_default() -> Self {
        if let Some(proj_dirs) = ProjectDirs::from("com", "readitnow", "readitnow") {
            let config_dir = proj_dirs.config_dir();
            let config_path = config_dir.join("config.yaml");

            if config_path.exists() {
                if let Ok(config) = Self::load(&config_path) {
                    return config;
                }
            }
        }

        let default_config = Config::default();
        if let Some(proj_dirs) = ProjectDirs::from("com", "readitnow", "readitnow") {
            let config_dir = proj_dirs.config_dir();
            fs::create_dir_all(config_dir).ok();
            let config_path = config_dir.join("config.yaml");
            default_config.save(&config_path).ok();
        }

        default_config
    }

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