---
name: recipe-clipboard-roundtrip
version: 1.0.0
description: "Round-trip between Box Note and Markdown via clipboard. Use this recipe when the user wants to edit Box Note content in a text editor and paste it back, or needs a quick clipboard-based workflow without saving intermediate files."
metadata:
  openclaw:
    category: "recipe"
    domain: "productivity"
    requires:
      bins: ["box2md"]
      skills: ["box2md-to-md", "box2md-to-html"]
---

# Clipboard Round-Trip

> **PREREQUISITE:** Load the following skills to execute this recipe: `box2md-to-md`, `box2md-to-html`

Convert content between Box Note and Markdown using the clipboard, without intermediate files. This workflow is ideal for quick edits — grab content from a Box Note, edit in your favorite text editor, and paste it back with formatting intact.

## Steps

1. **Box Note → Markdown:** In your browser, select and copy the content from the Box Note. Then run:
   ```bash
   box2md to-md -p -c
   ```
   The clipboard now contains Markdown text.

2. **Edit** the Markdown in your text editor (paste from clipboard, make changes, copy the result).

3. **Markdown → Box Note:** With the edited Markdown in your clipboard, run:
   ```bash
   box2md to-html -p -c
   ```
   The clipboard now contains rich HTML formatted for Box Note.

4. **Paste** into the Box Note in your browser — formatting (headings, lists, tables, code blocks) is preserved.

## Tips

- If the clipboard appears empty or the command produces no output, verify that you copied from the Box Note in a browser (not from the desktop app, which may not expose HTML to the clipboard).
- For a one-shot conversion without editing, chain both directions: copy from Box Note, run `box2md to-md -p -c`, then immediately `box2md to-html -p -c` to round-trip test.
- The VSCode extension wraps this same workflow into "Copy as Box Note" and "Paste as Markdown" commands.
