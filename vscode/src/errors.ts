import type { Box2mdErrorCode } from './types';

export const ERROR_SETUP_GUIDE_ACTION = 'Open Setup Guide';

type ErrorShape = {
  cause: string;
  nextAction: string;
  showSetupGuide?: boolean;
};

const ERROR_MAP: Record<Box2mdErrorCode, ErrorShape> = {
  E_NO_ACTIVE_EDITOR: {
    cause: 'No active Markdown editor is available.',
    nextAction: 'Open a Markdown file and run the command again.'
  },
  E_EMPTY_INPUT: {
    cause: 'The conversion input is empty.',
    nextAction: 'Select text or add content before running the command again.'
  },
  E_RUNTIME_NOT_FOUND: {
    cause: 'Could not find a box2md executable.',
    nextAction: 'Install box2md or set box2md.executablePath, then retry.',
    showSetupGuide: true
  },
  E_CONVERSION_FAILED: {
    cause: 'box2md failed to convert the content.',
    nextAction: 'Check the input and retry. If it persists, inspect the error details.'
  },
  E_UNSUPPORTED_OS: {
    cause: 'This extension currently supports only macOS and Windows.',
    nextAction: 'Run this command on a supported OS.'
  }
};

export class Box2mdError extends Error {
  public readonly code: Box2mdErrorCode;
  public readonly causeText: string;
  public readonly nextAction: string;
  public readonly showSetupGuide: boolean;
  public readonly details?: string;

  constructor(code: Box2mdErrorCode, details?: string) {
    const shape = ERROR_MAP[code];
    super(shape.cause);
    this.name = 'Box2mdError';
    this.code = code;
    this.causeText = shape.cause;
    this.nextAction = shape.nextAction;
    this.showSetupGuide = Boolean(shape.showSetupGuide);
    this.details = details;
  }
}

export function createBox2mdError(code: Box2mdErrorCode, details?: string): Box2mdError {
  return new Box2mdError(code, details);
}

export function toBox2mdError(
  error: unknown,
  fallbackCode: Box2mdErrorCode = 'E_CONVERSION_FAILED'
): Box2mdError {
  if (error instanceof Box2mdError) {
    return error;
  }

  if (error instanceof Error) {
    return new Box2mdError(fallbackCode, error.message);
  }

  return new Box2mdError(fallbackCode, String(error));
}
