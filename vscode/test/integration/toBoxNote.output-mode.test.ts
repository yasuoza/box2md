import assert from 'node:assert/strict';

import { createConversionService } from '../../src/conversionService';
import { FakeClipboard, FakeEditor, FakeNotifications } from '../helpers/fakes';

describe('toBoxNote output mode', () => {
  it('writes to editor when configured output mode is editor', async () => {
    const clipboard = new FakeClipboard();
    const notifications = new FakeNotifications();
    const editor = new FakeEditor('text', {
      start: 4,
      end: 4,
      isEmpty: true
    });

    const service = createConversionService({
      resolveRuntime: async () => ({ source: 'path', executablePath: '/tmp/box2md' }),
      runCommand: async () => ({ stdout: '<p>converted</p>', stderr: '' }),
      getActiveEditor: () => editor,
      clipboard,
      notifications,
      showSetupGuide: async () => undefined,
      getConfig: () => ({
        defaultOutput: 'editor',
        showVerboseError: false
      })
    });

    const ok = await service.toBoxNote();

    assert.equal(ok, true);
    assert.equal(clipboard.text, '');
    assert.equal(editor.content, 'text<p>converted</p>');
    assert.match(notifications.infos[0], /inserted/);
  });
});
