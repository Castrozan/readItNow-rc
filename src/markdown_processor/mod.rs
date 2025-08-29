mod config;
mod regex_patterns;
mod text_utils;
mod processor;

pub use config::MarkdownProcessorConfig;
pub use processor::MarkdownProcessor;

pub fn clean_obsidian_markdown(markdown: &str) -> String {
    let processor = MarkdownProcessor::new();
    processor.process(markdown)
}

pub fn clean_obsidian_markdown_with_formatting(markdown: &str) -> String {
    let config = MarkdownProcessorConfig {
        preserve_formatting: true,
        ..MarkdownProcessorConfig::default()
    };
    let processor = MarkdownProcessor::with_config(config);
    processor.process(markdown)
}
