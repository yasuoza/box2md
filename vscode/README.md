# box2md VSCode Extension

Use the existing `box2md` CLI from VSCode commands.

## Commands

- `box2md.toBoxNote`: Convert selected Markdown (or whole document) into Box Note-ready content.
- `box2md.fromBoxNote`: Convert clipboard Box Note content into Markdown and insert into editor.
- `box2md.showSetupGuide`: Open setup and recovery guidance.

## Settings

- `box2md.executablePath`: Explicit path to `box2md` executable.
- `box2md.toBoxNote.defaultOutput`: `clipboard` (default) or `editor`.
- `box2md.preferBundledBinary`: Reserved for future behavior.
- `box2md.showVerboseError`: Include extra details in error notifications.

## Runtime resolution

Resolution order is fixed for MVP:

1. `box2md.executablePath` when valid
2. `box2md` found on `PATH`
3. Bundled binary fallback (`bin/darwin/box2md` or `bin/win32/box2md.exe`)

Supported OS for MVP: macOS, Windows.

## Development

```bash
cd vscode
npm install
npm run lint
npm test
npm run test:vscode
```

## Troubleshooting

- **Cause:** Could not find a `box2md` executable.
  **Next action:** Install `box2md` or configure `box2md.executablePath`.
- **Cause:** Input is empty.
  **Next action:** Select text or add Markdown content and retry.
- **Cause:** Unsupported OS.
  **Next action:** Run on macOS or Windows.
