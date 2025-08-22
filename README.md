# ReadItNow

A terminal-based application for managing and browsing your ["Read It Later"](https://github.com/DominikPieper/obsidian-ReadItLater) notes with a clean, keyboard-driven interface.

## Features

- **Two-column pagination UI**: Easily browse your notes.
- **Note Card Display**: Each note is displayed as a card with:
    - Title
    - First few lines of content
    - Tags (extracted from `[[tag-here]]` wiki-links)
    - Read State (visually differentiated if tagged `[[readitnow/read]]`)
- **Keyboard Navigation**: Navigate through note pages using arrow keys, PageUp/PageDown.
- **Actions**: Open note URLs in your browser, open note files in your editor, and toggle read/unread status.
- **Persistence**: Read/unread state is managed directly within your Markdown files using wiki-link tags, ensuring portability and version control.

## Installation

It's available as a nix package. Add the following to your `flake.nix`:
```nix
inputs.readitnow.url = "github:castrozan/readitnow";
```

## Keybindings

- **↑ / ↓ / ← / →**: Navigate between notes.
- **PageUp / PageDown**: Scroll through pages.
- **Enter**: Open the note's URL in your default browser.
- **Shift+Enter**: Open the `.md` note file in your default editor.
- **r**: Toggle the read/unread status of the selected note.
- **q**: Quit the application.

## Development

```bash
devenv shell
```
