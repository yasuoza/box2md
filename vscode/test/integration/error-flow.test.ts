import assert from 'node:assert/strict';

import { createConversionService } from '../../src/conversionService';
import { createBox2mdError } from '../../src/errors';
import { FakeClipboard, FakeEditor, FakeNotifications } from '../helpers/fakes';

describe('error flows', () => {
  it('shows runtime-not-found error with recovery action', async () => {
    const notifications = new FakeNotifications();
    const service = createConversionService({
      resolveRuntime: async () => {
        throw createBox2mdError('E_RUNTIME_NOT_FOUND');
      },
      runCommand: async () => ({ stdout: '', stderr: '' }),
      getActiveEditor: () => new FakeEditor('# title'),
      clipboard: new FakeClipboard(),
      notifications,
      showSetupGuide: async () => undefined,
      getConfig: () => ({
        defaultOutput: 'clipboard',
        showVerboseError: false
      })
    });

    const ok = await service.toBoxNote();

    assert.equal(ok, false);
    assert.match(notifications.errors[0], /Cause:/);
    assert.match(notifications.errors[0], /Next action:/);
  });

  it('shows empty-input error for blank editor content', async () => {
    const notifications = new FakeNotifications();
    const service = createConversionService({
      resolveRuntime: async () => ({ source: 'path', executablePath: '/tmp/box2md' }),
      runCommand: async () => ({ stdout: '', stderr: '' }),
      getActiveEditor: () => new FakeEditor('   '),
      clipboard: new FakeClipboard(),
      notifications,
      showSetupGuide: async () => undefined,
      getConfig: () => ({
        defaultOutput: 'clipboard',
        showVerboseError: false
      })
    });

    const ok = await service.toBoxNote();

    assert.equal(ok, false);
    assert.match(notifications.errors[0], /conversion input is empty/i);
  });
});
