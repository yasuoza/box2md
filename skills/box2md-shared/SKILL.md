---
name: box2md-shared
version: 1.0.0
description: "box2md CLI: Shared patterns for installation, common flags, and input/output. Use this skill whenever working with Box Notes, .boxnote files, or converting content between Box Note and Markdown formats."
metadata:
  openclaw:
    category: "productivity"
    requires:
      bins: ["box2md"]
---

# box2md — Shared Reference

## Installation

### From source

Requires [Rust](https://www.rust-lang.org/tools/install) (stable).

```bash
cargo install --path .
```

### Pre-built binary

Download from [Releases](https://github.com/yasuoza/box2md/releases). macOS, Linux, and Windows binaries are available.

> **macOS note:** The binary is not signed. Remove the quarantine attribute on first run:
>
> ```bash
> xattr -d com.apple.quarantine box2md
> ```

## CLI Syntax

```bash
box2md <command> [flags]
```

### Commands

| Command | Description |
|---------|-------------|
| `to-md` | Convert Box Note (JSON file or clipboard HTML) to Markdown |
| `to-html` | Convert Markdown to GFM-compatible HTML |

### Common Flags

| Flag | Short | Description |
|------|-------|-------------|
| `--input <FILE>` | `-i` | Read from file (conflicts with `--from-clipboard`) |
| `--output <FILE>` | `-o` | Write to file (conflicts with `--copy`) |
| `--from-clipboard` | `-p` | Read from clipboard |
| `--copy` | `-c` | Write to clipboard |
| `--version` | `-V` | Print version |

### I/O Modes

| Input source | Output destination | Flags |
|---|---|---|
| File | stdout | `-i file` |
| File | File | `-i file -o file` |
| Clipboard | stdout | `-p` |
| Clipboard | Clipboard | `-p -c` |
| stdin | stdout | *(no flags)* |

## How Box Notes Work

Box Notes use a ProseMirror-based JSON format (`.boxnote` files). When copied from the browser, they produce HTML. `box2md` handles both formats transparently via the `to-md` command.

## Editor Integrations

- **VSCode extension** — Download `.vsix` from Releases. Commands: "Copy as Box Note", "Paste as Markdown".
- **Vim/Neovim plugin** — `Plug 'yasuoza/box2md', { 'rtp': 'vim' }`. Commands: `:Box2html`, `:Box2md`.

## Community & Feedback

- For bugs or feature requests, open issues at: `https://github.com/yasuoza/box2md/issues`
- Before creating a new issue, search existing issues first
