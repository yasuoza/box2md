import { createBox2mdError, toBox2mdError } from './errors';
import type { CommandRunner } from './commandRunner';
import { extractEditorInput, writeFromBoxNoteResult } from './editorIO';
import type { EditorLike } from './editorIO';
import { dispatchToBoxNoteOutput, resolveToBoxNoteOutputMode } from './output';
import type { ClipboardLike } from './output';
import type { NotificationApi } from './notifications';
import { notifyError, notifySuccess } from './notifications';
import type { RuntimeResolution } from './types';

export interface ConversionServiceDeps {
  resolveRuntime: () => Promise<RuntimeResolution>;
  runCommand: CommandRunner;
  getActiveEditor: () => EditorLike | undefined;
  clipboard: ClipboardLike;
  notifications: NotificationApi;
  showSetupGuide: () => PromiseLike<void>;
  getConfig: () => {
    defaultOutput: unknown;
    showVerboseError: boolean;
  };
}

export interface ConversionService {
  toBoxNote: () => Promise<boolean>;
  fromBoxNote: () => Promise<boolean>;
}

async function handleFailure(error: unknown, deps: ConversionServiceDeps): Promise<false> {
  const mapped = toBox2mdError(error);
  await notifyError(deps.notifications, mapped, {
    verbose: deps.getConfig().showVerboseError,
    showSetupGuide: deps.showSetupGuide
  });
  return false;
}

export function createConversionService(deps: ConversionServiceDeps): ConversionService {
  return {
    toBoxNote: async () => {
      try {
        const editor = deps.getActiveEditor();
        if (!editor) {
          throw createBox2mdError('E_NO_ACTIVE_EDITOR');
        }

        const input = extractEditorInput(editor);
        if (!input.inputText.trim()) {
          throw createBox2mdError('E_EMPTY_INPUT');
        }

        const runtime = await deps.resolveRuntime();
        const outputMode = resolveToBoxNoteOutputMode(deps.getConfig().defaultOutput);

        const args = outputMode === 'clipboard' ? ['to-html', '-c'] : ['to-html'];
        const commandResult = await deps.runCommand(runtime.executablePath, args, input.inputText);

        if (outputMode === 'editor') {
          await dispatchToBoxNoteOutput({
            outputMode,
            outputText: commandResult.stdout,
            clipboard: deps.clipboard,
            editor
          });
        }

        await notifySuccess(deps.notifications, 'toBoxNote', outputMode);
        return true;
      } catch (error) {
        return handleFailure(error, deps);
      }
    },

    fromBoxNote: async () => {
      try {
        const editor = deps.getActiveEditor();
        if (!editor) {
          throw createBox2mdError('E_NO_ACTIVE_EDITOR');
        }

        const runtime = await deps.resolveRuntime();
        const commandResult = await deps.runCommand(runtime.executablePath, ['to-md', '--from-clipboard']);
        await writeFromBoxNoteResult(editor, commandResult.stdout);

        await notifySuccess(deps.notifications, 'fromBoxNote', 'editor');
        return true;
      } catch (error) {
        return handleFailure(error, deps);
      }
    }
  };
}
