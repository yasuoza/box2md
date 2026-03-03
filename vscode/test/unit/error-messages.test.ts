import assert from 'node:assert/strict';

import { createBox2mdError, toBox2mdError } from '../../src/errors';
import { errorMessage } from '../../src/notifications';

describe('error message mapping', () => {
  it('formats runtime-not-found with cause and next action', () => {
    const error = createBox2mdError('E_RUNTIME_NOT_FOUND');
    const message = errorMessage(error, false);

    assert.match(message, /Cause:/);
    assert.match(message, /Next action:/);
    assert.match(message, /box2md executable/);
  });

  it('maps unknown errors to conversion failed', () => {
    const mapped = toBox2mdError(new Error('boom'));
    assert.equal(mapped.code, 'E_CONVERSION_FAILED');
  });
});
