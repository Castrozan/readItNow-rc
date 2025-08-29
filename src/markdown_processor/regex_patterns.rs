use regex::Regex;

pub struct ObsidianRegex {
    pub wiki_links: Regex,
    pub embeds: Regex,
    pub tags: Regex,
    pub yaml_frontmatter: Regex,
    pub dataview_queries: Regex,
    pub block_references: Regex,
}

impl ObsidianRegex {
    pub fn new() -> Self {
        Self {
            wiki_links: Regex::new(r"\[\[([^\]|]+)(?:\|([^\]]+))?\]\]").unwrap(),
            embeds: Regex::new(r"!\[\[([^\]]+)\]\]").unwrap(),
            tags: Regex::new(r"(?:^|[^\w/#])#([a-zA-Z][\w/-]*)").unwrap(),
            yaml_frontmatter: Regex::new(r"^---\s*\n.*?\n---\s*\n").unwrap(),
            dataview_queries: Regex::new(r"```dataview\s.*?\s```").unwrap(),
            block_references: Regex::new(r"\s*\^[\w-]+\s*$").unwrap(),
        }
    }
}
