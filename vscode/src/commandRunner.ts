import { spawn } from 'node:child_process';

import { createBox2mdError } from './errors';

export interface CommandRunResult {
  stdout: string;
  stderr: string;
}

export type CommandRunner = (
  executablePath: string,
  args: string[],
  stdin?: string
) => Promise<CommandRunResult>;

export const runCommand: CommandRunner = (executablePath, args, stdin) =>
  new Promise((resolve, reject) => {
    const child = spawn(executablePath, args, {
      stdio: 'pipe'
    });

    let stdout = '';
    let stderr = '';

    child.stdout.setEncoding('utf8');
    child.stderr.setEncoding('utf8');

    child.stdout.on('data', (chunk: string) => {
      stdout += chunk;
    });

    child.stderr.on('data', (chunk: string) => {
      stderr += chunk;
    });

    child.on('error', (error) => {
      reject(createBox2mdError('E_CONVERSION_FAILED', error.message));
    });

    child.on('close', (code) => {
      if (code === 0) {
        resolve({ stdout, stderr });
        return;
      }

      const detail = stderr.trim() || `box2md exited with code ${String(code)}`;
      reject(createBox2mdError('E_CONVERSION_FAILED', detail));
    });

    if (stdin !== undefined) {
      child.stdin.write(stdin);
    }
    child.stdin.end();
  });
