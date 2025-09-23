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

const json = require('../../../../lib/engine/formats/json');
const utils = require('../_utils');

utils.testBidirectional(json, 'a single character string property', [
  '{',
  '  "foo": "x"',
  '}'
], {
  foo: 'x'
});

utils.testBidirectional(json, 'a multiple character string property', [
  '{',
  '  "foo": "bar"',
  '}'
], {
  foo: 'bar'
});

utils.testBidirectional(json, 'a zero property', [
  '{',
  '  "foo": 0',
  '}'
], {
  foo: 0
});

utils.testBidirectional(json, 'a positive integer property', [
  '{',
  '  "foo": 5',
  '}'
], {
  foo: 5
});

utils.testBidirectional(json, 'a negative integer property', [
  '{',
  '  "foo": -1',
  '}'
], {
  foo: -1
});

utils.testBidirectional(json, 'a positive float property', [
  '{',
  '  "foo": 5.34',
  '}'
], {
  foo: 5.34
});

utils.testBidirectional(json, 'a negative float property', [
  '{',
  '  "foo": -10.51',
  '}'
], {
  foo: -10.51
});

utils.testBidirectional(json, 'a truthy boolean property', [
  '{',
  '  "foo": true',
  '}'
], {
  foo: true
});

utils.testBidirectional(json, 'a falsy boolean property', [
  '{',
  '  "foo": false',
  '}'
], {
  foo: false
});

utils.testBidirectional(json, 'a nested object', [
  '{',
  '  "foo": {',
  '    "bar": 1,',
  '    "baz": {',
  '      "name": "John Doe"',
  '    }',
  '  }',
  '}'
], {
  foo: {
    bar: 1,
    baz: {
      name: 'John Doe'
    }
  }
});

utils.testBidirectional(json, 'an empty object', [ '{}' ], {});

utils.testParse(json, 'a compressed object', [
  '{"foo":"bar"}'
], {
  foo: 'bar'
});
