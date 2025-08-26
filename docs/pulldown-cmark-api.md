# pulldown-cmark API Documentation

## Overview
`pulldown-cmark` is a Rust library for parsing CommonMark-compliant Markdown. It provides an event-based streaming parser that converts Markdown into a sequence of events representing the document structure.

## Dependencies
```toml
[dependencies]
pulldown-cmark = "0.9.1"
```

## Core API

### Parser
The main struct for parsing Markdown:

```rust
use pulldown_cmark::{Parser, Options};

let parser = Parser::new(markdown_text);
// or with options:
let parser = Parser::new_ext(markdown_text, Options::all());
```

### Event Enum
The parser generates a stream of events representing the Markdown structure:

```rust
pub enum Event<'a> {
    Start(Tag<'a>),
    End(Tag<'a>),
    Text(CowStr<'a>),
    Code(CowStr<'a>),
    Html(CowStr<'a>),
    FootnoteReference(CowStr<'a>),
    SoftBreak,
    HardBreak,
    Rule,
    TaskListMarker(bool),
}
```

### Tag Enum
Tags represent the start and end of block and inline elements:

```rust
pub enum Tag<'a> {
    Paragraph,
    Heading(HeadingLevel, Option<&'a str>, Vec<CowStr<'a>>),
    BlockQuote,
    CodeBlock(CodeBlockKind<'a>),
    List(Option<u64>),
    Item,
    FootnoteDefinition(CowStr<'a>),
    Table(Vec<Alignment>),
    TableHead,
    TableRow,
    TableCell,
    Emphasis,
    Strong,
    Strikethrough,
    Link(LinkType, CowStr<'a>, CowStr<'a>),
    Image(LinkType, CowStr<'a>, CowStr<'a>),
}
```

### LinkType Enum
```rust
pub enum LinkType {
    Inline,
    Reference,
    ReferenceUnknown,
    Collapsed,
    CollapsedUnknown,
    Shortcut,
    ShortcutUnknown,
    Autolink,
    Email,
}
```

## Usage Patterns

### Basic Text Extraction
```rust
use pulldown_cmark::{Parser, Event};

fn extract_text(markdown: &str) -> String {
    let parser = Parser::new(markdown);
    let mut text = String::new();
    
    for event in parser {
        match event {
            Event::Text(t) => text.push_str(&t),
            Event::Code(c) => text.push_str(&c),
            Event::SoftBreak | Event::HardBreak => text.push(' '),
            _ => {}
        }
    }
    
    text
}
```

### Filtering Events
```rust
fn filter_markdown(markdown: &str) -> Vec<Event> {
    let parser = Parser::new(markdown);
    
    parser.filter(|event| {
        match event {
            Event::Start(Tag::Link(_, url, _)) => {
                !url.starts_with("obsidian://")
            }
            _ => true
        }
    }).collect()
}
```

### Custom Processing
```rust
fn process_markdown(markdown: &str) -> String {
    let parser = Parser::new(markdown);
    let mut output = String::new();
    
    for event in parser {
        match event {
            Event::Start(Tag::Heading(level, _, _)) => {
                output.push_str(&"#".repeat(level as usize));
                output.push(' ');
            }
            Event::Text(text) => {
                output.push_str(&text);
            }
            Event::SoftBreak => output.push(' '),
            Event::HardBreak => output.push('\n'),
            _ => {}
        }
    }
    
    output
}
```

## Obsidian-Specific Considerations

### Wiki Links
Obsidian `[[wiki links]]` are typically parsed as regular text and need preprocessing:

```rust
fn preprocess_obsidian(text: &str) -> String {
    // Remove wiki links
    let wiki_link_re = regex::Regex::new(r"\[\[([^\]]+)\]\]").unwrap();
    let text = wiki_link_re.replace_all(text, "$1");
    
    // Remove embeds
    let embed_re = regex::Regex::new(r"!\[\[([^\]]+)\]\]").unwrap();
    let text = embed_re.replace_all(&text, "");
    
    text.to_string()
}
```

### Tags
Obsidian tags `#tag` can be handled during text processing:

```rust
fn clean_tags(text: &str) -> String {
    let tag_re = regex::Regex::new(r"#\w+").unwrap();
    tag_re.replace_all(text, "").to_string()
}
```

## Options
Available parsing options:

```rust
pub struct Options: u32 {
    const ENABLE_TABLES = 1 << 1;
    const ENABLE_FOOTNOTES = 1 << 2;
    const ENABLE_STRIKETHROUGH = 1 << 3;
    const ENABLE_TASKLISTS = 1 << 4;
    const ENABLE_SMART_PUNCTUATION = 1 << 5;
    const ENABLE_HEADING_ATTRIBUTES = 1 << 6;
}
```

## Error Handling
pulldown-cmark is designed to be forgiving and doesn't typically return parsing errors. It attempts to parse any input as valid Markdown.

## Performance Notes
- Event-based parsing is memory efficient for large documents
- The parser processes content incrementally
- Use `Parser::new` for basic parsing, `Parser::new_ext` for extended features
