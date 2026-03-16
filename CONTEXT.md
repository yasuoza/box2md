# box2md Context

A CLI tool for bidirectional conversion between [Box Notes](https://www.box.com/en-gb/notes) and Markdown.

## Rules of Engagement for Agents

* **Input Detection:** Box Note files (`.boxnote`) are JSON. Clipboard content from Box Notes is HTML. The `to-md` command handles both transparently — use `-i` for files and `-p` for clipboard.
* **Rich Clipboard:** When writing to clipboard with `-c`, `to-html` writes **rich text HTML** (not plain text). This ensures formatting is preserved when pasting into Box Note.
* **No Auth Required:** box2md is a local conversion tool with no API calls. No authentication or credentials are needed.

## Core Syntax

```bash
box2md <command> [flags]
```

Use `--help` to get help on available commands.

```bash
box2md --help
box2md to-md --help
box2md to-html --help
```

### Commands

| Command | Input | Output |
|---------|-------|--------|
| `to-md` | `.boxnote` file (JSON) or clipboard HTML | Markdown |
| `to-html` | Markdown file or clipboard text | GFM HTML |

### Common Flags

| Flag | Short | Description |
|------|-------|-------------|
| `--input <FILE>` | `-i` | Read from file |
| `--output <FILE>` | `-o` | Write to file |
| `--from-clipboard` | `-p` | Read from clipboard |
| `--copy` | `-c` | Write to clipboard |

## Usage Patterns

### 1. Converting Box Notes to Markdown

```bash
# Single file
box2md to-md -i notes.boxnote -o notes.md

# From clipboard (copied from Box Note in browser)
box2md to-md -p -c
```

### 2. Converting Markdown to Box Note

```bash
# Copy as rich HTML for pasting into Box Note
box2md to-html -i notes.md -c

# Clipboard round-trip
box2md to-html -p -c
```

### 3. Batch Processing

```bash
for f in *.boxnote; do box2md to-md -i "$f" -o "${f%.boxnote}.md"; done
```
