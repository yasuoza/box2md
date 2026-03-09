import * as vscode from 'vscode';

import { runCommand } from './commandRunner';
import { createConversionService } from './conversionService';
import type { EditorLike } from './editorIO';
import { resolveRuntime } from './runtime';
import { createSetupGuideCommand } from './setupGuide';

function toEditorLike(editor: vscode.TextEditor): EditorLike {
  return {
    get selection() {
      const selection = editor.selection;
      return {
        start: editor.document.offsetAt(selection.start),
        end: editor.document.offsetAt(selection.end),
        isEmpty: selection.isEmpty
      };
    },
    getText(range) {
      if (!range) {
        return editor.document.getText();
      }
      const start = editor.document.positionAt(range.start);
      const end = editor.document.positionAt(range.end);
      return editor.document.getText(new vscode.Range(start, end));
    },
    async replaceSelection(text) {
      await editor.edit((editBuilder) => {
        editBuilder.replace(editor.selection, text);
      });
    },
    async insertAtCursor(text) {
      await editor.edit((editBuilder) => {
        editBuilder.insert(editor.selection.active, text);
      });
    }
  };
}

export function activate(context: vscode.ExtensionContext): void {
  const guideCommand = createSetupGuideCommand({
    openTextDocument: (options) => vscode.workspace.openTextDocument(options),
    showTextDocument: (document) => vscode.window.showTextDocument(document as vscode.TextDocument)
  });

  const service = createConversionService({
    resolveRuntime: async () => {
      const config = vscode.workspace.getConfiguration('box2md');
      return resolveRuntime({
        extensionPath: context.extensionPath,
        configuredPath: config.get<string>('executablePath')
      });
    },
    runCommand,
    getActiveEditor: () => {
      const editor = vscode.window.activeTextEditor;
      if (!editor) {
        return undefined;
      }
      return toEditorLike(editor);
    },
    clipboard: {
      writeText: (text) => vscode.env.clipboard.writeText(text)
    },
    notifications: {
      showInformationMessage: (message, ...items) =>
        vscode.window.showInformationMessage(message, ...items),
      showErrorMessage: (message, ...items) => vscode.window.showErrorMessage(message, ...items)
    },
    showSetupGuide: guideCommand,
    getConfig: () => {
      const config = vscode.workspace.getConfiguration('box2md');
      return {
        defaultOutput: config.get('toBoxNote.defaultOutput'),
        showVerboseError: Boolean(config.get('showVerboseError'))
      };
    }
  });

  context.subscriptions.push(
    vscode.commands.registerCommand('box2md.toBoxNote', async () => {
      await service.toBoxNote();
    }),
    vscode.commands.registerCommand('box2md.fromBoxNote', async () => {
      await service.fromBoxNote();
    }),
    vscode.commands.registerCommand('box2md.showSetupGuide', guideCommand)
  );
}

export function deactivate(): void {
  // no-op
}
