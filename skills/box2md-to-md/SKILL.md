---
name: box2md-to-md
version: 1.0.0
description: "box2md: Convert Box Note JSON files or clipboard HTML to Markdown. Use this skill when the user wants to convert .boxnote files, extract content from Box Notes, or paste Box Note clipboard content as Markdown."
metadata:
  openclaw:
    category: "productivity"
    requires:
      bins: ["box2md"]
      skills: ["box2md-shared"]
    cliHelp: "box2md to-md --help"
---

# to-md

> **PREREQUISITE:** Read `../box2md-shared/SKILL.md` for installation, common flags, and I/O modes.

Convert Box Note to Markdown. Accepts two input formats:

- `.boxnote` JSON files (direct export or from Box API)
- HTML from clipboard (as copied from a Box Note in browser)

## Usage

```bash
box2md to-md -i <FILE>
box2md to-md --from-clipboard
```

## Flags

| Flag | Short | Required | Description |
|------|-------|----------|-------------|
| `--input <FILE>` | `-i` | one of `-i` or `-p` | Box Note JSON file (.boxnote) to convert |
| `--output <FILE>` | `-o` | — | Output Markdown file (default: stdout) |
| `--from-clipboard` | `-p` | one of `-i` or `-p` | Read HTML from clipboard |
| `--copy` | `-c` | — | Write Markdown result to clipboard |
| *(stdin)* | — | — | Reads Box Note JSON from stdin when neither `-i` nor `-p` is given |

## Examples

```bash
# Convert a .boxnote file and print to stdout
box2md to-md -i meeting-notes.boxnote

# Convert and save to .md file
box2md to-md -i meeting-notes.boxnote -o meeting-notes.md

# Convert clipboard HTML (copied from Box Note in browser) to clipboard
box2md to-md -p -c

# Convert clipboard HTML to stdout
box2md to-md -p

# Convert from stdin
cat notes.boxnote | box2md to-md
```

## Tips

- When converting from clipboard (`-p`), first select and copy text from the Box Note in your browser. The clipboard will contain rich HTML that box2md parses.
- Pipe from stdin for scripting: `cat *.boxnote | box2md to-md > all-notes.md`
- Output includes GFM features: tables, checklists, strikethrough, fenced code blocks.

## See Also

- [box2md-shared](../box2md-shared/SKILL.md) — Installation and common flags
- [box2md-to-html](../box2md-to-html/SKILL.md) — Markdown to HTML conversion
