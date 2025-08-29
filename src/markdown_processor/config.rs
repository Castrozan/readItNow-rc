#[derive(Debug, Clone)]
pub struct MarkdownProcessorConfig {
    pub preserve_formatting: bool,
    pub convert_headings: bool,
    pub include_link_text: bool,
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
