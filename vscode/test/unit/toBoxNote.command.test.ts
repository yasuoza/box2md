import assert from 'node:assert/strict';

import { extractEditorInput } from '../../src/editorIO';
import { FakeEditor } from '../helpers/fakes';

describe('toBoxNote target resolution', () => {
  it('uses selected text when a selection exists', () => {
    const editor = new FakeEditor('hello markdown world', {
      start: 6,
      end: 14,
      isEmpty: false
    });

    const result = extractEditorInput(editor);

    assert.equal(result.selectionMode, 'selection');
    assert.equal(result.inputText, 'markdown');
  });

  it('uses full document when no selection exists', () => {
    const editor = new FakeEditor('# heading\nbody', {
      start: 0,
      end: 0,
      isEmpty: true
    });

    const result = extractEditorInput(editor);

    assert.equal(result.selectionMode, 'document');
    assert.equal(result.inputText, '# heading\nbody');
  });
});
