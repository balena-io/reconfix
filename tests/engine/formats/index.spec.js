/*
 * Copyright 2016 Resin.io
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *    http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

'use strict';

const ava = require('ava');
const _ = require('lodash');
const formats = require('../../../lib/engine/formats');

ava('.parse() should throw if type is not supported', (test) => {
  test.throws(() => {
    formats.parse('foo', 'hello=world');
  }, 'Unsupported type: foo');
});

ava('.serialise() should throw if type is not supported', (test) => {
  test.throws(() => {
    formats.serialise('foo', {
      hello: 'world'
    });
  }, 'Unsupported type: foo');
});

_.each([
  {
    type: 'json',
    string: [
      '{',
      '  "data": {',
      '    "name": "John Doe",',
      '    "job": "Software Engineer"',
      '  }',
      '}'
    ].join('\n')
  },
  {
    type: 'ini',
    string: [
      '[data]',
      'name=John Doe',
      'job=Software Engineer'
    ].join('\n')
  }
], (testCase) => {

  ava(`should handle ${testCase.type}`, (test) => {
    const parsed = formats.parse(testCase.type, testCase.string);
    const serialised = formats.serialise(testCase.type, parsed);
    test.deepEqual(serialised, testCase.string);
  });

});
