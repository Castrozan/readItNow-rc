# ReadItNow

ReadItNow is a terminal UI application built with Rust and Ratatui, designed to help you manage and read notes from your Obsidian vault's ReadItLater folder.

## Features

- **Two-column infinite scroll UI**: Easily browse your notes.
- **Note Card Display**: Each note is displayed as a card with:
    - Title (from filename)
    - Excerpt (first few lines of content)
    - Tags (extracted from `[[tag-here]]` wiki-links)
    - Read State (visually differentiated if tagged `[[readitnow/read]]`)
- **Keyboard Navigation**: Navigate through notes using arrow keys, PageUp/PageDown.
- **Actions**: Open note URLs in your default browser, open note files in your default editor, and toggle read/unread status.
- **Persistence**: Read/unread state is managed directly within your Markdown files using wiki-link tags, ensuring portability and version control.

## Installation

### Prerequisites

- Rust and Cargo: If you don't have Rust installed, you can install it using `rustup`:
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
- `build-essential` (for Linux): Required for compiling some dependencies.
  ```bash
  sudo apt-get update
  sudo apt-get install -y build-essential libssl-dev
  ```

### Build from Source

1. Clone the repository:
   ```bash
   git clone <repository_url>
   cd readitnow
   ```
2. Build the application:
   ```bash
   cargo build --release
   ```

## Usage

1. **Configuration**: Before running, create a `config.yaml` file in `~/.config/readitnow/` (or modify the default behavior in `src/models.rs`). A sample `config.yaml` looks like this:

   ```yaml
   vault_path: "/home/you/vault/ReadItLater Inbox"
   max_notes: 20
   excerpt_lines: 5
   keybindings:
     open_link: "enter"
     open_file: "shift+enter"
     up: "up"
     down: "down"
     left: "left"
     right: "right"
     page_up: "pageup"
     page_down: "pagedown"
     quit: "q"
   thumbnail_cache: "~/.cache/readitnow/thumbnails"
   ```

2. **Run the application**:
   ```bash
   cargo run --release
   ```

## Keybindings

- **↑ / ↓ / ← / →**: Navigate between notes.
- **PageUp / PageDown**: Scroll through notes.
- **Enter**: Open the note's URL in your default browser.
- **Shift+Enter**: Open the `.md` note file in your default editor.
- **r**: Toggle the read/unread status of the selected note.
- **q**: Quit the application.

## Development

### Running Tests

```bash
cargo test
```

### Project Structure

- `src/main.rs`: Main application entry point, handles terminal setup, event loop, and UI rendering.
- `src/models.rs`: Defines data structures for `Note` and `Config`, and includes parsing logic.
- `src/ui.rs`: Contains Ratatui widgets and rendering logic for note cards and the note list.
- `src/app.rs`: Manages the application state and business logic.
- `src/vault.rs`: Handles interaction with the Obsidian vault, including scanning for notes and toggling read status.
- `src/kitty.rs`: (Planned) Integration with Kitty terminal graphics protocol for thumbnail display.

## Future Enhancements

- Full implementation of Kitty graphics protocol for proper thumbnail display.
- Asynchronous note loading and thumbnail fetching to prevent UI blocking.
- More robust error handling and user feedback.
- Customizable UI themes and styling.
- Search and filtering capabilities.


