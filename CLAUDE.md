# box2md

CLI tool + editor plugins for converting between Box Note and Markdown.

## Tech Stack

### CLI (Rust)
- Rust (stable) / clap 4 (derive)
- comrak 0.50 (Markdown parser/renderer, GFM support)
- htmd 0.5 (HTML → Markdown conversion)
- arboard 3.6 (clipboard HTML rich text read/write)
- serde / serde_json 1 (Box Note JSON serialization)
- anyhow 1 (error handling)

### VSCode Extension (TypeScript)
- TypeScript 5.x / Node.js 20 LTS
- VSCode Extension API (^1.95.0)
- mocha (testing) / eslint (linting)
- Invokes box2md binary via child_process

### Vim/Neovim Plugin
- VimScript (`vim/plugin/box2md.vim`)

## Development Guidelines

- Commit messages follow Conventional Commits (in English)
- Versions are updated in bulk via `scripts/set-version.sh <version>` (Cargo.toml / vscode/package.json / git tag)
