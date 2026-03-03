import { Box2mdError, ERROR_SETUP_GUIDE_ACTION } from './errors';
import type { OutputMode } from './types';

export interface NotificationApi {
  showInformationMessage(message: string, ...items: string[]): Thenable<string | undefined>;
  showErrorMessage(message: string, ...items: string[]): Thenable<string | undefined>;
}

export function successMessage(
  command: 'toBoxNote' | 'fromBoxNote',
  outputMode: OutputMode = 'editor'
): string {
  if (command === 'toBoxNote') {
    return outputMode === 'clipboard'
      ? 'Converted for Box Note and copied to clipboard.'
      : 'Converted for Box Note and inserted in the editor.';
  }

  return 'Converted Box Note content and inserted Markdown in the editor.';
}

export function errorMessage(error: Box2mdError, verbose: boolean): string {
  const lines = [
    `Cause: ${error.causeText}`,
    `Next action: ${error.nextAction}`
  ];

  if (verbose && error.details) {
    lines.push(`Details: ${error.details}`);
  }

  return lines.join('\n');
}

export async function notifySuccess(
  api: NotificationApi,
  command: 'toBoxNote' | 'fromBoxNote',
  outputMode?: OutputMode
): Promise<void> {
  await api.showInformationMessage(successMessage(command, outputMode));
}

export async function notifyError(
  api: NotificationApi,
  error: Box2mdError,
  options: {
    verbose: boolean;
    showSetupGuide: () => PromiseLike<void>;
  }
): Promise<void> {
  const actions = error.showSetupGuide ? [ERROR_SETUP_GUIDE_ACTION] : [];
  const picked = await api.showErrorMessage(errorMessage(error, options.verbose), ...actions);
  if (picked === ERROR_SETUP_GUIDE_ACTION) {
    await options.showSetupGuide();
  }
}
