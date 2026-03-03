import assert from 'node:assert/strict';

import { resolveRuntime } from '../../src/runtime';

describe('runtime platform guard', () => {
  it('fails on unsupported OS', async () => {
    await assert.rejects(
      () =>
        resolveRuntime({
          extensionPath: '/tmp/extension',
          platform: 'linux',
          fileExists: async () => false
        }),
      /supports only macOS and Windows/
    );
  });

  it('resolves configured runtime path first', async () => {
    const runtime = await resolveRuntime({
      extensionPath: '/tmp/extension',
      configuredPath: '/custom/box2md',
      platform: 'darwin',
      fileExists: async (candidate) => candidate === '/custom/box2md'
    });

    assert.equal(runtime.source, 'configured');
    assert.equal(runtime.executablePath, '/custom/box2md');
  });
});
