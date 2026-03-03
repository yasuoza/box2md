export interface SetupGuideHost {
  openTextDocument(options: { language: string; content: string }): PromiseLike<unknown>;
  showTextDocument(document: unknown): PromiseLike<unknown>;
}

export function setupGuideContent(): string {
  return [
    '# box2md Setup Guide',
    '',
    '## Prerequisites',
    '- Use macOS or Windows for MVP.',
    '- Install `box2md` and ensure it is available in PATH, or set `box2md.executablePath`.',
    '',
    '## Commands',
    '- `box2md.toBoxNote`: Convert Markdown to Box Note-ready HTML.',
    '- `box2md.fromBoxNote`: Convert clipboard Box Note content into Markdown and insert into editor.',
    '',
    '## Recovery',
    '- If runtime is not found: install box2md or set executablePath.',
    '- If input is empty: select text or add content before running commands.'
  ].join('\n');
}

export async function showSetupGuide(host: SetupGuideHost): Promise<void> {
  const document = await host.openTextDocument({
    language: 'markdown',
    content: setupGuideContent()
  });
  await host.showTextDocument(document);
}

export function createSetupGuideCommand(host: SetupGuideHost): () => Promise<void> {
  return async () => {
    await showSetupGuide(host);
  };
}
