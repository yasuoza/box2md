# box2md

CLI tool for converting Box Note to Markdown.

## Tech Stack

- Rust (stable) / clap 4 (derive)
- comrak 0.50 (Markdown parser/renderer, GFM support)
- htmd 0.5 (HTML → Markdown conversion)
- arboard 3.6 (clipboard HTML rich text read/write)
- serde / serde_json 1 (Box Note JSON serialization)
- anyhow 1 (error handling)

## Development Guidelines

- Add tests when implementing changes
- Commit messages follow Conventional Commits (in English)
- Run `cargo fmt` after code changes before committing
