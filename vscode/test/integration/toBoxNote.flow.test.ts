import assert from 'node:assert/strict';

import { createConversionService } from '../../src/conversionService';
import { FakeClipboard, FakeEditor, FakeNotifications } from '../helpers/fakes';

describe('toBoxNote flow', () => {
  it('uses clipboard output by default', async () => {
    const clipboard = new FakeClipboard();
    const notifications = new FakeNotifications();
    const editor = new FakeEditor('# title');

    const service = createConversionService({
      resolveRuntime: async () => ({ source: 'path', executablePath: '/tmp/box2md' }),
      runCommand: async (_path, args, stdin) => {
        // clipboard mode delegates rich-text writing to box2md via -c flag
        assert.deepEqual(args, ['to-html', '-c']);
        assert.equal(stdin, '# title');
        return { stdout: '<h1>title</h1>', stderr: '' };
      },
      getActiveEditor: () => editor,
      clipboard,
      notifications,
      showSetupGuide: async () => undefined,
      getConfig: () => ({
        defaultOutput: undefined,
        showVerboseError: false
      })
    });

    const ok = await service.toBoxNote();

    assert.equal(ok, true);
    // clipboard.writeText is NOT called; box2md -c writes rich text directly
    assert.equal(clipboard.text, '');
    assert.equal(editor.content, '# title');
    assert.match(notifications.infos[0], /clipboard/);
  });
});
