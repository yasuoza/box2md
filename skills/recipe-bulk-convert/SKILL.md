---
name: recipe-bulk-convert
version: 1.0.0
description: "Bulk convert .boxnote files to Markdown in a directory. Use this recipe when the user has multiple Box Note files to convert at once, or wants to migrate a folder of .boxnote files to Markdown."
metadata:
  openclaw:
    category: "recipe"
    domain: "productivity"
    requires:
      bins: ["box2md"]
      skills: ["box2md-to-md"]
---

# Bulk Convert Box Notes

> **PREREQUISITE:** Load the following skills to execute this recipe: `box2md-to-md`

Bulk convert all `.boxnote` files in a directory to Markdown. Each `.boxnote` file produces a `.md` file with the same base name.

## Steps

1. Find all `.boxnote` files in the target directory:
   ```bash
   find /path/to/notes -name '*.boxnote'
   ```

2. Convert each file, placing the `.md` output alongside the original:
   ```bash
   for f in /path/to/notes/*.boxnote; do
     box2md to-md -i "$f" -o "${f%.boxnote}.md"
   done
   ```

3. For nested directories, use `find` with `-exec`:
   ```bash
   find /path/to/notes -name '*.boxnote' -exec sh -c \
     'box2md to-md -i "$1" -o "${1%.boxnote}.md"' _ {} \;
   ```

4. Verify the results:
   ```bash
   find /path/to/notes -name '*.md' -newer /path/to/notes
   ```

## Tips

- If a `.boxnote` file fails to convert, `box2md` exits with a non-zero status. Add `|| echo "FAILED: $f"` to the loop to log failures without stopping the batch.
- To output all Markdown files to a separate directory, adjust the `-o` path:
  ```bash
  mkdir -p output
  for f in *.boxnote; do
    box2md to-md -i "$f" -o "output/$(basename "${f%.boxnote}.md")"
  done
  ```
