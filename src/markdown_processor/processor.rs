use pulldown_cmark::{Parser, Event, Tag, Options};
use super::config::MarkdownProcessorConfig;
use super::regex_patterns::ObsidianRegex;
use super::text_utils::{normalize_whitespace, wrap_lines};

pub struct MarkdownProcessor {
    config: MarkdownProcessorConfig,
    obsidian_regex: ObsidianRegex,
}

impl MarkdownProcessor {
    pub fn new() -> Self {
        Self::with_config(MarkdownProcessorConfig::default())
    }

    pub fn with_config(config: MarkdownProcessorConfig) -> Self {
        Self {
            config,
            obsidian_regex: ObsidianRegex::new(),
        }
    }

    pub fn process(&self, markdown: &str) -> String {
        let preprocessed = self.preprocess_obsidian_syntax(markdown);
        let cleaned = self.extract_clean_text(&preprocessed);
        self.post_process(&cleaned)
    }

    fn preprocess_obsidian_syntax(&self, text: &str) -> String {
        let mut result = text.to_string();

        result = self.remove_yaml_frontmatter(&result);
        result = self.remove_dataview_queries(&result);
        result = self.process_wiki_links(&result);
        result = self.remove_embeds(&result);
        result = self.remove_tags(&result);
        result = self.remove_block_references(&result);

        result
    }

    fn remove_yaml_frontmatter(&self, text: &str) -> String {
        self.obsidian_regex.yaml_frontmatter.replace_all(text, "").to_string()
    }

    fn remove_dataview_queries(&self, text: &str) -> String {
        self.obsidian_regex.dataview_queries.replace_all(text, "").to_string()
    }

    fn process_wiki_links(&self, text: &str) -> String {
        self.obsidian_regex.wiki_links.replace_all(text, |caps: &regex::Captures| {
            if let Some(alias) = caps.get(2) {
                alias.as_str().to_string()
            } else if let Some(link) = caps.get(1) {
                link.as_str().to_string()
            } else {
                String::new()
            }
        }).to_string()
    }

    fn remove_embeds(&self, text: &str) -> String {
        self.obsidian_regex.embeds.replace_all(text, "").to_string()
    }

    fn remove_tags(&self, text: &str) -> String {
        self.obsidian_regex.tags.replace_all(text, "$1").to_string()
    }

    fn remove_block_references(&self, text: &str) -> String {
        self.obsidian_regex.block_references.replace_all(text, "").to_string()
    }

    fn extract_clean_text(&self, markdown: &str) -> String {
        let options = Options::empty();
        let parser = Parser::new_ext(markdown, options);
        let mut result = String::new();
        let mut in_code_block = false;

        for event in parser {
            self.process_event(event, &mut result, &mut in_code_block);
        }

        result
    }

    fn process_event(&self, event: Event, result: &mut String, in_code_block: &mut bool) {
        match event {
            Event::Start(Tag::CodeBlock(_)) => {
                *in_code_block = true;
                if self.config.preserve_formatting {
                    result.push_str("[Code] ");
                }
            }
            Event::End(Tag::CodeBlock(_)) => {
                *in_code_block = false;
                result.push('\n');
            }
            Event::Start(Tag::Heading(level, _, _)) if self.config.convert_headings => {
                let prefix = "#".repeat(level as usize);
                result.push_str(&format!("{} ", prefix));
            }
            Event::End(Tag::Heading(_, _, _)) if self.config.convert_headings => {
                result.push('\n');
            }
            Event::Start(Tag::Image(_, _, _)) => {
                if self.config.preserve_formatting {
                    result.push_str("[Image] ");
                }
            }
            Event::Text(text) => {
                if !*in_code_block {
                    result.push_str(&text);
                }
            }
            Event::Code(code) => {
                if self.config.preserve_formatting {
                    result.push_str(&format!("`{}`", code));
                } else {
                    result.push_str(&code);
                }
            }
            Event::SoftBreak => result.push(' '),
            Event::HardBreak => result.push('\n'),
            Event::Start(Tag::Emphasis) if self.config.preserve_formatting => result.push('*'),
            Event::End(Tag::Emphasis) if self.config.preserve_formatting => result.push('*'),
            Event::Start(Tag::Strong) if self.config.preserve_formatting => result.push_str("**"),
            Event::End(Tag::Strong) if self.config.preserve_formatting => result.push_str("**"),
            Event::Rule => result.push_str("\n---\n"),
            _ => {}
        }
    }

    fn post_process(&self, text: &str) -> String {
        let mut result = normalize_whitespace(text);

        if self.config.max_line_length > 0 {
            result = wrap_lines(&result, self.config.max_line_length);
        }

        result.trim().to_string()
    }
}
