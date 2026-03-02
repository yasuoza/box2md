# box2md

A CLI tool to convert [Box Notes](https://www.box.com/en-gb/notes) to Markdown and Markdown to HTML.

## Features

- **Box Note JSON → Markdown** — Convert `.boxnote` files directly
- **Box Note HTML → Markdown** — Read rich-text HTML from clipboard (as copied from Box Note in browser)
- **Markdown → HTML** — Convert Markdown to GFM-compatible HTML
- **Clipboard integration** — `--paste` to read from clipboard, `--copy` to write results back
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
box2md to-md --paste --copy

# From clipboard to stdout
box2md to-md --paste

# From stdin
cat note.boxnote | box2md to-md
```

### `to-html` — Convert Markdown to HTML

```sh
# From a file
box2md to-html -i note.md

# From clipboard to clipboard (as rich HTML)
box2md to-html --paste --copy

# From stdin
cat note.md | box2md to-html
```

### Flags

| Flag | Description |
|---|---|
| `-i, --input <FILE>` | Read from file (conflicts with `--paste`) |
| `-o, --output <FILE>` | Write to file (conflicts with `--copy`) |
| `--paste` | Read from clipboard |
| `--copy` | Write to clipboard |

## How it works

| Input | Command | Pipeline |
|---|---|---|
| `.boxnote` file (JSON) | `to-md -i` | Box Note JSON → Markdown |
| Box Note clipboard (HTML) | `to-md --paste` | HTML → Markdown |
| Markdown | `to-html` | Markdown → GFM HTML |

Box Notes use a ProseMirror-based JSON format internally. When copied from the browser, they produce HTML. `box2md` handles both formats transparently via the `to-md` command.

## Tech Stack

- **Rust** (stable) with [clap](https://crates.io/crates/clap) 4 for CLI
- [comrak](https://crates.io/crates/comrak) 0.50 — Markdown parser/renderer (GFM)
- [htmd](https://crates.io/crates/htmd) 0.5 — HTML to Markdown
- [arboard](https://crates.io/crates/arboard) 3.6 — Cross-platform clipboard (HTML rich-text support)
- [serde](https://crates.io/crates/serde) / [serde_json](https://crates.io/crates/serde_json) — Box Note JSON serialization

## License

MIT
