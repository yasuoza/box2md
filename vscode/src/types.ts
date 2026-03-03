export type ConversionDirection = 'md_to_boxnote' | 'boxnote_to_md';
export type SelectionMode = 'selection' | 'document';
export type OutputMode = 'clipboard' | 'editor';
export type RuntimeSource = 'configured' | 'path' | 'bundled';

export type Box2mdErrorCode =
  | 'E_NO_ACTIVE_EDITOR'
  | 'E_EMPTY_INPUT'
  | 'E_RUNTIME_NOT_FOUND'
  | 'E_CONVERSION_FAILED'
  | 'E_UNSUPPORTED_OS';

export interface RuntimeResolution {
  source: RuntimeSource;
  executablePath: string;
  version?: string;
}

export interface ConversionRequest {
  direction: ConversionDirection;
  inputText: string;
  selectionMode: SelectionMode;
  outputMode: OutputMode;
}

export interface ConversionResult {
  status: 'success' | 'error';
  outputText?: string;
  errorCode?: Box2mdErrorCode;
  userMessage: string;
}
