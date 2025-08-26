# Markdown Processor Implementation

## Overview

This document describes the implementation of the Obsidian Markdown cleaning functionality added to readItNow. The solution uses the `pulldown-cmark` library to properly parse and clean Obsidian-specific syntax from markdown notes.

## Architecture

### Files Added/Modified

1. **`docs/pulldown-cmark-api.md`** - Complete API documentation for the pulldown-cmark library
2. **`src/markdown_processor.rs`** - New module containing the markdown processing logic
3. **`src/models.rs`** - Modified to use the markdown processor for excerpts
4. **`src/lib.rs`** - Added markdown_processor module declaration
5. **`Cargo.toml`** - Added pulldown-cmark dependency

### Core Components

#### MarkdownProcessor
The main processor class with configurable options:

```rust
pub struct MarkdownProcessor {
    config: MarkdownProcessorConfig,
    obsidian_regex: ObsidianRegex,
}
```

#### MarkdownProcessorConfig
Configuration options for processing behavior:

```rust
pub struct MarkdownProcessorConfig {
    pub preserve_formatting: bool,    // Keep **bold** and *italic*
    pub convert_headings: bool,       // Convert # headings to plain text
    pub include_link_text: bool,      // Include link text without URLs
    pub max_line_length: usize,       // Text wrapping (0 = disabled)
}
```

## Processing Pipeline

The processor uses a three-stage pipeline:

### 1. Preprocessing (Obsidian-specific)
Removes/transforms Obsidian syntax using regex:

- **Wiki Links**: `[[Page]]` → `Page`, `[[Page|Alias]]` → `Alias`
- **Embeds**: `![[image.png]]` → *(removed)*
- **Tags**: `#tag` → *(removed)*
- **YAML Frontmatter**: `---\ntitle: Test\n---` → *(removed)*
- **Dataview Queries**: ````dataview ... ``` → *(removed)*
- **Block References**: `^block-id` → *(removed)*

### 2. Markdown Parsing (pulldown-cmark)
Uses pulldown-cmark's event-based parser to:

- Extract text content
- Handle standard markdown elements (headings, links, code, etc.)
- Preserve or remove formatting based on config
- Convert structure to plain text

### 3. Post-processing
Final cleanup:

- Normalize whitespace (collapse multiple spaces/newlines)
- Apply line wrapping if configured
- Trim excessive whitespace

## Integration Points

### Note Creation
In `models.rs`, the processor is integrated into `Note::from_markdown()`:

```rust
// Configure processor for excerpts
let processor_config = MarkdownProcessorConfig {
    preserve_formatting: false,
    convert_headings: false,
    include_link_text: true,
    max_line_length: 0,
};

let processor = MarkdownProcessor::with_config(processor_config);
note.excerpt = processor.process(&raw_excerpt);
```

## Usage Examples

### Basic Cleaning
```rust
use crate::markdown_processor::clean_obsidian_markdown;

let obsidian_text = "# Hello\n\nCheck [[Some Page]] with #tag";
let clean_text = clean_obsidian_markdown(obsidian_text);
// Result: "# Hello\n\nCheck Some Page with"
```

### With Formatting Preservation
```rust
use crate::markdown_processor::clean_obsidian_markdown_with_formatting;

let obsidian_text = "**Bold** text with [[link]]";
let clean_text = clean_obsidian_markdown_with_formatting(obsidian_text);
// Result: "**Bold** text with link"
```

### Custom Configuration
```rust
use crate::markdown_processor::{MarkdownProcessor, MarkdownProcessorConfig};

let config = MarkdownProcessorConfig {
    preserve_formatting: true,
    convert_headings: true,
    include_link_text: true,
    max_line_length: 80,
};

let processor = MarkdownProcessor::with_config(config);
let result = processor.process(markdown_text);
```

## Before/After Examples

### Input (Obsidian Markdown)
```markdown
---
title: My Note
tags: [productivity, tools]
---

# Important Article

Read this [[Amazing Article|great piece]] about #productivity.

![[screenshot.png]]

The author discusses:
- Point 1 with [[reference]]
- Point 2 #important
- Code: `pulldown-cmark`

> Quote with [[internal link]]

Check out: https://example.com ^abc123
```

### Output (Cleaned Text)
```
# Important Article

Read this great piece about productivity.

The author discusses:
- Point 1 with reference
- Point 2
- Code: pulldown-cmark

Quote with internal link

Check out: https://example.com
```

## Performance Considerations

- **Regex Compilation**: Compiled once per processor instance
- **Memory Efficiency**: Event-based parsing doesn't load entire AST
- **Processing Cost**: O(n) where n is content length
- **Caching**: Consider caching processor instances for repeated use

## Testing

The module includes comprehensive tests covering:

- Basic markdown cleaning
- Wiki link transformation
- Embed removal
- Tag removal
- YAML frontmatter removal
- Edge cases and malformed syntax

Run tests with:
```bash
cargo test markdown_processor
```

## Future Enhancements

Potential improvements for future versions:

1. **Caching**: Cache cleaned excerpts to avoid reprocessing
2. **Custom Regex**: Allow user-defined cleaning patterns
3. **Preservation Options**: More granular control over what to preserve
4. **Performance**: Streaming processing for very large files
5. **Format Support**: Support for other note formats (Notion, Roam, etc.)

## Dependencies

- **pulldown-cmark v0.9.1**: Markdown parsing
- **regex v1.10.4**: Obsidian syntax pattern matching (already in project)

## Maintenance Notes

- The regex patterns in `ObsidianRegex` may need updates as Obsidian syntax evolves
- Configuration options can be extended without breaking existing code
- The three-stage pipeline allows for easy modification of individual steps
- All public APIs are documented and tested
