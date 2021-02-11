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
const string = require('../../../lib/jsontemplate/string');

ava('.deinterpolate() should throw if strings do not match', (test) => {
  test.throws(() => {
    string.deinterpolate('Hello {{name}}!', 'Hi John Doe!');
  }, {
    message: 'No match for \'name\''
  });
});

ava('.deinterpolate() should throw if interpolation result is missing', (test) => {
  test.throws(() => {
    string.deinterpolate('Hello {{name}}!', 'Hi !');
  }, {
    message: 'No match for \'name\''
  });
});

ava('.deinterpolate() should accept a number type on an independent string', (test) => {
  test.deepEqual(string.deinterpolate('{{number:age}}', '21'), {
    age: 21
  });
});

ava('.deinterpolate() should accept a number type on a dependent string', (test) => {
  test.deepEqual(string.deinterpolate('I am {{number:age}} years old', 'I am 21 years old'), {
    age: 21
  });
});

ava('.deinterpolate() should parse a float from an independent string', (test) => {
  test.deepEqual(string.deinterpolate('{{number:foo}}', '21.123'), {
    foo: 21.123
  });
});

ava('.deinterpolate() should parse a float from an dependent string', (test) => {
  test.deepEqual(string.deinterpolate('Foo {{number:foo}} Foo', 'Foo 21.123 Foo'), {
    foo: 21.123
  });
});

ava('.deinterpolate() should be able to cast a zero', (test) => {
  test.deepEqual(string.deinterpolate('{{number:foo}}', '0'), {
    foo: 0
  });
});

ava('.deinterpolate() should be able to cast a negative number', (test) => {
  test.deepEqual(string.deinterpolate('{{number:foo}}', '-5'), {
    foo: -5
  });
});

ava('.deinterpolate() should throw if independent string casted to number becomes NaN', (test) => {
  test.throws(() => {
    string.deinterpolate('{{number:age}}', 'foo');
  }, {
    message: 'Can\'t convert foo to number'
  });
});

ava('.deinterpolate() should throw if dependent string casted to number becomes NaN', (test) => {
  test.throws(() => {
    string.deinterpolate('I am {{number:age}} years old', 'I am foo years old');
  }, {
    message: 'Can\'t convert foo to number'
  });
});

ava('.deinterpolate() should accept a string type', (test) => {
  test.deepEqual(string.deinterpolate('{{string:age}}', 21), {
    age: '21'
  });
});
