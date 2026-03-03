import assert from 'node:assert/strict';

import { writeFromBoxNoteResult } from '../../src/editorIO';
import { FakeEditor } from '../helpers/fakes';

describe('fromBoxNote replace vs insert', () => {
  it('replaces selected range when selection exists', async () => {
    const editor = new FakeEditor('before target after', {
      start: 7,
      end: 13,
      isEmpty: false
    });

    const mode = await writeFromBoxNoteResult(editor, 'markdown');

    assert.equal(mode, 'replaced');
    assert.equal(editor.content, 'before markdown after');
  });

  it('inserts at cursor when no selection exists', async () => {
    const editor = new FakeEditor('hello world', {
      start: 5,
      end: 5,
      isEmpty: true
    });

    const mode = await writeFromBoxNoteResult(editor, ' markdown');

    assert.equal(mode, 'inserted');
    assert.equal(editor.content, 'hello markdown world');
  });
});
