use readitnow::markdown_processor::{clean_obsidian_markdown, MarkdownProcessor, MarkdownProcessorConfig};

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

#[test]
fn test_with_formatting_preserved() {
    let config = MarkdownProcessorConfig {
        preserve_formatting: true,
        ..MarkdownProcessorConfig::default()
    };
    let processor = MarkdownProcessor::with_config(config);
    let input = "This is **bold** and *italic* text.";
    let result = processor.process(input);
    assert_eq!(result, "This is **bold** and *italic* text.");
}

#[test]
fn test_yaml_frontmatter_removal() {
    let input = "---\ntitle: Test\ntags: [test, markdown]\n---\n\n# Content here";
    let result = clean_obsidian_markdown(input);
    assert_eq!(result, "# Content here");
}
