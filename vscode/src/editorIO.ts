import { createBox2mdError } from './errors';
import type { SelectionMode } from './types';

export interface SelectionLike {
  start: number;
  end: number;
  isEmpty: boolean;
}

export interface EditorLike {
  selection: SelectionLike;
  getText(range?: SelectionLike): string;
  replaceSelection(text: string): Promise<void>;
  insertAtCursor(text: string): Promise<void>;
}

export interface EditorInput {
  inputText: string;
  selectionMode: SelectionMode;
}

export function extractEditorInput(editor: EditorLike): EditorInput {
  const { selection } = editor;
  if (selection.isEmpty) {
    return {
      inputText: editor.getText(),
      selectionMode: 'document'
    };
  }

  return {
    inputText: editor.getText(selection),
    selectionMode: 'selection'
  };
}

export async function writeFromBoxNoteResult(
  editor: EditorLike,
  markdownText: string
): Promise<'replaced' | 'inserted'> {
  if (!markdownText.trim()) {
    throw createBox2mdError('E_EMPTY_INPUT');
  }

  if (editor.selection.isEmpty) {
    await editor.insertAtCursor(markdownText);
    return 'inserted';
  }

  await editor.replaceSelection(markdownText);
  return 'replaced';
}

export async function insertToEditor(editor: EditorLike, text: string): Promise<void> {
  if (!text.trim()) {
    throw createBox2mdError('E_EMPTY_INPUT');
  }
  await editor.insertAtCursor(text);
}
