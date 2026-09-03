# miv

`miv` is a small terminal-based text editor written in Rust. As a Neovim
user, I wanted to understand how my text editor worked under the hood, so I
started building a minimal editor from scratch.

The project focuses on the foundations of a terminal editor: reading input
one byte at a time, switching the terminal into raw mode, maintaining a text
buffer and cursor, and drawing the editor interface with ANSI escape
sequences.

## Running the editor

The project requires Rust and a Unix-like terminal. Launch an empty editor
with:

```bash
cargo run
```

Open an existing file by passing its path:

```bash
cargo run -- path/to/file.txt
```

The editor currently writes changes back to the supplied file with `w` in
Normal mode. If no file is supplied, the buffer is not saved.

## Current features and roadmap

- Alternate-screen terminal interface.
- Raw terminal input using `termios`.
- In-memory text buffer represented as a collection of lines.
- Loading an existing file at startup.
- Saving the current buffer with `w`.
- Normal and Insert modes.
- Basic cursor movement and insertion editing.
- Line numbers and a mode/position information bar.
- Horizontal scrolling when the cursor moves beyond the visible width.

- [ ] More complete Normal-mode command handling
- [ ] Reliable quitting and terminal-state restoration on errors
- [ ] Vertical scrolling
- [ ] Deleting and replacing text
- [ ] Search and navigation commands
- [ ] Better Unicode and wide-character support
- [ ] Editor configuration

## Key bindings

### Normal mode

| Key | Action |
| --- | --- |
| `h` | Move left |
| `j` | Move down |
| `k` | Move up |
| `l` | Move right |
| `0` | Move to the start of the line |
| `$` | Move to the end of the line |
| `gg` | Move to the first line |
| `G` | Move to the last line |
| `i` | Enter Insert mode |
| `w` | Save the file |
| `q` | Quit |

### Insert mode

| Key | Action |
| --- | --- |
| `Esc` | Return to Normal mode |
| `Backspace` | Delete the previous character or join lines |
| `Enter` | Insert a new line |
| Other characters | Insert at the cursor |

## Project structure

```text
miv/
├── src/
│   ├── main.rs           Application entrypoint and event-loop setup
│   ├── terminal.rs       Raw mode, alternate screen, and terminal size
│   ├── key_bindings.rs   Input handling and editor state transitions
│   ├── display.rs        Buffer, cursor, and status-bar rendering
│   ├── utils.rs          Editor positions, modes, CLI, and file I/O
│   └── logger.rs         File-based debug logging
├── Cargo.toml            Package metadata and dependencies
└── Cargo.lock            Locked dependency versions
```

## Technical choices

### Raw terminal input

The editor uses `libc` to read and update the terminal's `termios` settings.
In raw mode, key presses are delivered immediately instead of waiting for a
newline, which lets the event loop react to individual keys.

### ANSI rendering

The display is drawn directly to standard output with ANSI escape sequences.
The editor uses the terminal's alternate screen, clears and redraws the
buffer when needed, and positions the cursor explicitly.

### Small, explicit state model

The editor state consists of the current mode, cursor position, text buffer,
optional file path, and renderer. Keeping these pieces explicit makes the
relationship between input handling, editing, and rendering easier to study.

## Building

Build a release binary with:

```bash
cargo build --release
```

The resulting executable is located at `target/release/miv`.
