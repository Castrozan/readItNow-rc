//! Markdown processing module for cleaning Obsidian-specific syntax
//! 
//! This module provides functionality to convert Obsidian-flavored Markdown
//! into clean, readable text by removing or transforming Obsidian-specific
//! syntax elements while preserving essential content structure.

use pulldown_cmark::{Parser, Event, Tag, Options, CowStr};
use regex::Regex;

/// Configuration for markdown processing
#[derive(Debug, Clone)]
pub struct MarkdownProcessorConfig {
    /// Whether to preserve basic formatting (bold, italic) in output
    pub preserve_formatting: bool,
    /// Whether to convert headings to plain text with level indicators
    pub convert_headings: bool,
    /// Whether to include link text without URLs
    pub include_link_text: bool,
    /// Maximum line length for text wrapping (0 = no wrapping)
    pub max_line_length: usize,
}

impl Default for MarkdownProcessorConfig {
    fn default() -> Self {
        Self {
            preserve_formatting: false,
            convert_headings: true,
            include_link_text: true,
            max_line_length: 0,
        }
    }
}

/// Main markdown processor for cleaning Obsidian syntax
pub struct MarkdownProcessor {
    config: MarkdownProcessorConfig,
    obsidian_regex: ObsidianRegex,
}

/// Compiled regex patterns for Obsidian syntax
struct ObsidianRegex {
    wiki_links: Regex,
    embeds: Regex,
    tags: Regex,
    yaml_frontmatter: Regex,
    dataview_queries: Regex,
    block_references: Regex,
}

impl ObsidianRegex {
    fn new() -> Self {
        Self {
            // [[wiki link]] or [[wiki link|alias]]
            wiki_links: Regex::new(r"\[\[([^\]|]+)(?:\|([^\]]+))?\]\]").unwrap(),
            // ![[embed]]
            embeds: Regex::new(r"!\[\[([^\]]+)\]\]").unwrap(),
            // #tags (but not in code or links)
            tags: Regex::new(r"(?:^|[^\w/#])#([a-zA-Z][\w/-]*)").unwrap(),
            // YAML frontmatter (--- ... ---)
            yaml_frontmatter: Regex::new(r"^---\s*\n.*?\n---\s*\n").unwrap(),
            // Dataview queries
            dataview_queries: Regex::new(r"```dataview\s.*?\s```").unwrap(),
            // Block references ^block-id
            block_references: Regex::new(r"\s*\^[\w-]+\s*$").unwrap(),
        }
    }
}

impl MarkdownProcessor {
    /// Create a new markdown processor with default configuration
    pub fn new() -> Self {
        Self::with_config(MarkdownProcessorConfig::default())
    }

    /// Create a new markdown processor with custom configuration
    pub fn with_config(config: MarkdownProcessorConfig) -> Self {
        Self {
            config,
            obsidian_regex: ObsidianRegex::new(),
        }
    }

    /// Process markdown content and return cleaned text
    pub fn process(&self, markdown: &str) -> String {
        // Step 1: Preprocess to remove Obsidian-specific syntax
        let preprocessed = self.preprocess_obsidian_syntax(markdown);
        
        // Step 2: Parse with pulldown-cmark and extract clean text
        let cleaned = self.extract_clean_text(&preprocessed);
        
        // Step 3: Post-process for final cleanup
        self.post_process(&cleaned)
    }

    /// Preprocess text to remove Obsidian-specific syntax before markdown parsing
    fn preprocess_obsidian_syntax(&self, text: &str) -> String {
        let mut result = text.to_string();

        // Remove YAML frontmatter
        result = self.obsidian_regex.yaml_frontmatter.replace_all(&result, "").to_string();

        // Remove dataview queries
        result = self.obsidian_regex.dataview_queries.replace_all(&result, "").to_string();

        // Process wiki links - keep the text content
        result = self.obsidian_regex.wiki_links.replace_all(&result, |caps: &regex::Captures| {
            if let Some(alias) = caps.get(2) {
                // Use alias if available: [[link|alias]] -> alias
                alias.as_str().to_string()
            } else if let Some(link) = caps.get(1) {
                // Use link text: [[link]] -> link
                link.as_str().to_string()
            } else {
                String::new()
            }
        }).to_string();

        // Remove embeds entirely
        result = self.obsidian_regex.embeds.replace_all(&result, "").to_string();

        // Remove tags
        result = self.obsidian_regex.tags.replace_all(&result, "$1").to_string();

        // Remove block references
        result = self.obsidian_regex.block_references.replace_all(&result, "").to_string();

        result
    }

    /// Extract clean text from markdown using pulldown-cmark
    fn extract_clean_text(&self, markdown: &str) -> String {
        let options = Options::empty(); // Use basic parsing, no extensions
        let parser = Parser::new_ext(markdown, options);
        let mut result = String::new();
        let mut in_code_block = false;

        for event in parser {
            match event {
                Event::Start(Tag::CodeBlock(_)) => {
                    in_code_block = true;
                    if self.config.preserve_formatting {
                        result.push_str("[Code] ");
                    }
                }
                Event::End(Tag::CodeBlock(_)) => {
                    in_code_block = false;
                    result.push('\n');
                }
                Event::Start(Tag::Heading(level, _, _)) if self.config.convert_headings => {
                    let prefix = "#".repeat(level as usize);
                    result.push_str(&format!("{} ", prefix));
                }
                Event::End(Tag::Heading(_, _, _)) if self.config.convert_headings => {
                    result.push('\n');
                }
                Event::Start(Tag::Link(_, url, _)) => {
                    if !self.config.include_link_text {
                        // Skip link content if not including link text
                        continue;
                    }
                    // Otherwise, process link text normally
                }
                Event::Start(Tag::Image(_, _, _)) => {
                    if self.config.preserve_formatting {
                        result.push_str("[Image] ");
                    }
                    // Skip image content
                }
                Event::Text(text) => {
                    if !in_code_block {
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
                Event::SoftBreak => {
                    result.push(' ');
                }
                Event::HardBreak => {
                    result.push('\n');
                }
                Event::Start(Tag::Emphasis) if self.config.preserve_formatting => {
                    result.push('*');
                }
                Event::End(Tag::Emphasis) if self.config.preserve_formatting => {
                    result.push('*');
                }
                Event::Start(Tag::Strong) if self.config.preserve_formatting => {
                    result.push_str("**");
                }
                Event::End(Tag::Strong) if self.config.preserve_formatting => {
                    result.push_str("**");
                }
                Event::Rule => {
                    result.push_str("\n---\n");
                }
                _ => {}
            }
        }

        result
    }

    /// Post-process the extracted text for final cleanup
    fn post_process(&self, text: &str) -> String {
        let mut result = text.to_string();

        // Clean up excessive whitespace
        result = self.normalize_whitespace(&result);

        // Apply line length limits if configured
        if self.config.max_line_length > 0 {
            result = self.wrap_lines(&result, self.config.max_line_length);
        }

        // Trim and ensure we don't have trailing newlines
        result.trim().to_string()
    }

    /// Normalize whitespace - collapse multiple spaces and clean up line breaks
    fn normalize_whitespace(&self, text: &str) -> String {
        // Replace multiple spaces with single space
        let space_regex = Regex::new(r" +").unwrap();
        let result = space_regex.replace_all(text, " ");

        // Replace multiple newlines with at most two newlines
        let newline_regex = Regex::new(r"\n\s*\n\s*\n+").unwrap();
        let result = newline_regex.replace_all(&result, "\n\n");

        // Clean up space before newlines
        let trailing_space_regex = Regex::new(r" +\n").unwrap();
        trailing_space_regex.replace_all(&result, "\n").to_string()
    }

    /// Wrap lines to specified length (simple word wrapping)
    fn wrap_lines(&self, text: &str, max_length: usize) -> String {
        text.lines()
            .map(|line| {
                if line.len() <= max_length {
                    line.to_string()
                } else {
                    self.wrap_single_line(line, max_length)
                }
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Wrap a single line to specified length
    fn wrap_single_line(&self, line: &str, max_length: usize) -> String {
        let words: Vec<&str> = line.split_whitespace().collect();
        let mut result = Vec::new();
        let mut current_line = String::new();

        for word in words {
            if current_line.is_empty() {
                current_line = word.to_string();
            } else if current_line.len() + 1 + word.len() <= max_length {
                current_line.push(' ');
                current_line.push_str(word);
            } else {
                result.push(current_line);
                current_line = word.to_string();
            }
        }

        if !current_line.is_empty() {
            result.push(current_line);
        }

        result.join("\n")
    }
}

/// Convenience function for basic markdown cleaning with default settings
pub fn clean_obsidian_markdown(markdown: &str) -> String {
    let processor = MarkdownProcessor::new();
    processor.process(markdown)
}

/// Convenience function for cleaning markdown while preserving some formatting
pub fn clean_obsidian_markdown_with_formatting(markdown: &str) -> String {
    let config = MarkdownProcessorConfig {
        preserve_formatting: true,
        ..MarkdownProcessorConfig::default()
    };
    let processor = MarkdownProcessor::with_config(config);
    processor.process(markdown)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_markdown_cleaning() {
        let input = "# Hello World\nThis is **bold** text.";
        let result = clean_obsidian_markdown(input);
        assert_eq!(result, "# Hello World\nThis is bold text.");
    }

    #[test]
    fn test_wiki_link_cleaning() {
        let input = "Check out [[Some Page]] and [[Another Page|with alias]].";
        let result = clean_obsidian_markdown(input);
        assert_eq!(result, "Check out Some Page and with alias.");
    }

    #[test]
    fn test_embed_removal() {
        let input = "Here's an embed: ![[image.png]] and some text.";
        let result = clean_obsidian_markdown(input);
        assert_eq!(result, "Here's an embed:  and some text.");
    }

    #[test]
    fn test_tag_removal() {
        let input = "This has #tag1 and #tag2/nested tags.";
        let result = clean_obsidian_markdown(input);
        assert_eq!(result, "This has  and  tags.");
    }
}
