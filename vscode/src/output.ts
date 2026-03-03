import { createBox2mdError } from './errors';
import type { EditorLike } from './editorIO';
import { insertToEditor } from './editorIO';
import type { OutputMode } from './types';

export interface ClipboardLike {
  writeText(text: string): PromiseLike<void>;
}

export interface ToBoxNoteOutputOptions {
  outputMode: OutputMode;
  outputText: string;
  clipboard: ClipboardLike;
  editor?: EditorLike;
}

export function resolveToBoxNoteOutputMode(configuredValue: unknown): OutputMode {
  return configuredValue === 'editor' ? 'editor' : 'clipboard';
}

export async function dispatchToBoxNoteOutput(options: ToBoxNoteOutputOptions): Promise<void> {
  const { outputMode, outputText, clipboard, editor } = options;

  if (outputMode === 'clipboard') {
    await clipboard.writeText(outputText);
    return;
  }

  if (!editor) {
    throw createBox2mdError('E_NO_ACTIVE_EDITOR');
  }

  await insertToEditor(editor, outputText);
}
