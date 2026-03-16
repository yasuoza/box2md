---
name: box2md-to-html
version: 1.0.0
description: "box2md: Convert Markdown to GFM-compatible HTML for pasting into Box Note. Use this skill when the user wants to copy Markdown content into a Box Note with formatting preserved, or convert Markdown files to HTML."
metadata:
  openclaw:
    category: "productivity"
    requires:
      bins: ["box2md"]
      skills: ["box2md-shared"]
    cliHelp: "box2md to-html --help"
---

# to-html

> **PREREQUISITE:** Read `../box2md-shared/SKILL.md` for installation, common flags, and I/O modes.

Convert Markdown to GFM-compatible HTML. When used with `--copy` (`-c`), the HTML is written to the clipboard as rich text, ready to paste into a Box Note.

## Usage

```bash
box2md to-html -i <FILE>
box2md to-html --from-clipboard
```

## Flags

| Flag | Short | Required | Description |
|------|-------|----------|-------------|
| `--input <FILE>` | `-i` | one of `-i` or `-p` | Markdown file to convert |
| `--output <FILE>` | `-o` | — | Output HTML file (default: stdout) |
| `--from-clipboard` | `-p` | one of `-i` or `-p` | Read Markdown from clipboard |
| `--copy` | `-c` | — | Write HTML result to clipboard as rich text |
| *(stdin)* | — | — | Reads Markdown from stdin when neither `-i` nor `-p` is given |

## Examples

```bash
# Convert a Markdown file and print HTML to stdout
box2md to-html -i notes.md

# Convert and copy as rich text to clipboard (for pasting into Box Note)
box2md to-html -i notes.md -c

# Round-trip: clipboard Markdown → clipboard rich HTML
box2md to-html -p -c

# Convert from stdin
cat notes.md | box2md to-html
```

## Tips

- Use `-c` to copy as **rich text HTML** to the clipboard — this pastes directly into Box Note with formatting preserved.
- The output uses GFM extensions: tables, strikethrough, task lists.
- Box Note checklists use a custom `<li class="check-list-item">` format; `to-html` handles this automatically.

## See Also

- [box2md-shared](../box2md-shared/SKILL.md) — Installation and common flags
- [box2md-to-md](../box2md-to-md/SKILL.md) — Box Note to Markdown conversion
