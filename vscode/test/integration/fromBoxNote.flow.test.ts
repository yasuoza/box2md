import assert from 'node:assert/strict';

import { createConversionService } from '../../src/conversionService';
import { FakeClipboard, FakeEditor, FakeNotifications } from '../helpers/fakes';

describe('fromBoxNote flow', () => {
  it('inserts markdown converted from clipboard', async () => {
    const notifications = new FakeNotifications();
    const editor = new FakeEditor('before after', {
      start: 7,
      end: 7,
      isEmpty: true
    });

    const service = createConversionService({
      resolveRuntime: async () => ({ source: 'path', executablePath: '/tmp/box2md' }),
      runCommand: async (_path, args) => {
        assert.deepEqual(args, ['to-md', '--from-clipboard']);
        return { stdout: 'middle ', stderr: '' };
      },
      getActiveEditor: () => editor,
      clipboard: new FakeClipboard(),
      notifications,
      showSetupGuide: async () => undefined,
      getConfig: () => ({
        defaultOutput: 'clipboard',
        showVerboseError: false
      })
    });

    const ok = await service.fromBoxNote();

    assert.equal(ok, true);
    assert.equal(editor.content, 'before middle after');
    assert.match(notifications.infos[0], /inserted Markdown/);
  });
});
