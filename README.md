# box2md

A CLI tool to convert [Box Notes](https://www.box.com/en-gb/notes) to Markdown and Markdown to HTML.

## Features

- **Box Note JSON → Markdown** — Convert `.boxnote` files directly
- **Box Note HTML → Markdown** — Read rich-text HTML from clipboard (as copied from Box Note in browser)
- **Markdown → HTML** — Convert Markdown to GFM-compatible HTML
- **Clipboard integration** — `--from-clipboard` (`-p`) to read from clipboard, `--copy` (`-c`) to write results back
- Handles Box Note-specific quirks: non-standard list nesting, `table_header` nodes, version prefix in JSON files

## Installation

### From source

Requires [Rust](https://www.rust-lang.org/tools/install) (stable).

```sh
cargo install --path .
```

### Pre-built binary

Download from [Releases](https://github.com/yasuoza/box2md/releases).

> **macOS note:** The binary is not signed. macOS Gatekeeper will block it on first run. To allow execution:
>
> ```sh
> xattr -d com.apple.quarantine box2md
> ```

## Usage

### `to-md` — Convert to Markdown

```sh
# From a .boxnote file
box2md to-md -i note.boxnote

# From a .boxnote file to a .md file
box2md to-md -i note.boxnote -o note.md

# From clipboard (HTML copied from Box Note in browser) to clipboard
box2md to-md -p -c

# From clipboard to stdout
box2md to-md --from-clipboard

# From stdin
cat note.boxnote | box2md to-md
```

### `to-html` — Convert Markdown to HTML

```sh
# From a file
box2md to-html -i note.md

# From clipboard to clipboard (as rich HTML)
box2md to-html -p -c

# From stdin
cat note.md | box2md to-html
```

### Flags

| Flag | Description |
|---|---|
| `-i, --input <FILE>` | Read from file (conflicts with `--from-clipboard`) |
| `-o, --output <FILE>` | Write to file (conflicts with `--copy`) |
| `-p, --from-clipboard` | Read from clipboard |
| `-c, --copy` | Write to clipboard |

## How it works

| Input | Command | Pipeline |
|---|---|---|
| `.boxnote` file (JSON) | `to-md -i` | Box Note JSON → Markdown |
| Box Note clipboard (HTML) | `to-md -p` | HTML → Markdown |
| Markdown | `to-html` | Markdown → GFM HTML |

Box Notes use a ProseMirror-based JSON format internally. When copied from the browser, they produce HTML. `box2md` handles both formats transparently via the `to-md` command.

## VSCode Extension

### Installation

Download `box2md-vscode-<version>.vsix` from the [Releases](https://github.com/yasuoza/box2md/releases) page and install:

```sh
code --install-extension box2md-vscode-*.vsix
```

### Commands

- **box2md: Copy as Box Note** — Copy Markdown as rich text HTML to clipboard for pasting into Box Note
- **box2md: Paste as Markdown** — Convert Box Note clipboard content to Markdown and insert into editor
- **box2md: Show Setup Guide** — Open setup and recovery guidance

Runtime resolution order:

1. `box2md.executablePath` (if configured and valid)
2. `box2md` from `PATH`
3. Bundled fallback under `vscode/bin/{darwin,win32}/`

## Vim / Neovim Plugin

A plugin is included under `vim/`. It provides two commands:

| Command | Description |
|---|---|
| `:'<,'>Box2html` | Convert selected Markdown to HTML and copy to clipboard |
| `:Box2md` | Convert Box Note HTML from clipboard to Markdown and insert at cursor |

### Installation

```vim
" vim-plug
Plug 'yasuoza/box2md', { 'rtp': 'vim' }
```

To use a custom binary path:

```vim
let g:box2md_path = '~/.bin/box2md'
```

## Tech Stack

- **Rust** (stable) with [clap](https://crates.io/crates/clap) 4 for CLI
- [comrak](https://crates.io/crates/comrak) 0.50 — Markdown parser/renderer (GFM)
- [htmd](https://crates.io/crates/htmd) 0.5 — HTML to Markdown
- [arboard](https://crates.io/crates/arboard) 3.6 — Cross-platform clipboard (HTML rich-text support)
- [serde](https://crates.io/crates/serde) / [serde_json](https://crates.io/crates/serde_json) — Box Note JSON serialization

## License

MIT
