import { constants } from 'node:fs';
import { access } from 'node:fs/promises';
import path from 'node:path';

import { createBox2mdError } from './errors';
import type { RuntimeResolution } from './types';

export interface ResolveRuntimeOptions {
  extensionPath: string;
  configuredPath?: string;
  platform?: NodeJS.Platform;
  pathEnv?: string;
  fileExists?: (candidate: string) => Promise<boolean>;
}

const SUPPORTED_PLATFORMS: NodeJS.Platform[] = ['darwin', 'win32'];

function binaryNameFor(platform: NodeJS.Platform): string {
  return platform === 'win32' ? 'box2md.exe' : 'box2md';
}

async function defaultFileExists(candidate: string): Promise<boolean> {
  const mode = process.platform === 'win32' ? constants.F_OK : constants.X_OK;
  try {
    await access(candidate, mode);
    return true;
  } catch {
    return false;
  }
}

function splitPath(pathEnv: string | undefined): string[] {
  if (!pathEnv) {
    return [];
  }
  return pathEnv.split(path.delimiter).filter(Boolean);
}

async function findOnPath(
  platform: NodeJS.Platform,
  pathEnv: string | undefined,
  fileExists: (candidate: string) => Promise<boolean>
): Promise<string | undefined> {
  const binaryName = binaryNameFor(platform);
  const pathEntries = splitPath(pathEnv);
  for (const entry of pathEntries) {
    const candidate = path.join(entry, binaryName);
    if (await fileExists(candidate)) {
      return candidate;
    }
  }
  return undefined;
}

function bundledBinaryPath(extensionPath: string, platform: NodeJS.Platform): string {
  return path.join(
    extensionPath,
    'bin',
    platform === 'win32' ? 'win32' : 'darwin',
    binaryNameFor(platform)
  );
}

export function assertSupportedPlatform(platform: NodeJS.Platform = process.platform): void {
  if (!SUPPORTED_PLATFORMS.includes(platform)) {
    throw createBox2mdError('E_UNSUPPORTED_OS', `Detected platform: ${platform}`);
  }
}

export async function resolveRuntime(options: ResolveRuntimeOptions): Promise<RuntimeResolution> {
  const {
    extensionPath,
    configuredPath,
    platform = process.platform,
    pathEnv = process.env.PATH,
    fileExists = defaultFileExists
  } = options;

  assertSupportedPlatform(platform);

  const trimmedConfiguredPath = configuredPath?.trim();
  if (trimmedConfiguredPath && (await fileExists(trimmedConfiguredPath))) {
    return {
      source: 'configured',
      executablePath: trimmedConfiguredPath
    };
  }

  const pathResolved = await findOnPath(platform, pathEnv, fileExists);
  if (pathResolved) {
    return {
      source: 'path',
      executablePath: pathResolved
    };
  }

  const bundledCandidate = bundledBinaryPath(extensionPath, platform);
  if (await fileExists(bundledCandidate)) {
    return {
      source: 'bundled',
      executablePath: bundledCandidate
    };
  }

  throw createBox2mdError(
    'E_RUNTIME_NOT_FOUND',
    `configured=${trimmedConfiguredPath ?? '<empty>'}, bundled=${bundledCandidate}`
  );
}
