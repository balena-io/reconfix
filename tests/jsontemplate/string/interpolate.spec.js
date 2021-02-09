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

ava('.interpolate() should cast positive integer to string if interpolation has context', (test) => {
  test.deepEqual(string.interpolate('My age is {{age}}', {
    age: 21
  }), 'My age is 21');
});

ava('.interpolate() should cast negative integer to string if interpolation has context', (test) => {
  test.deepEqual(string.interpolate('The temperature is {{temperature}}', {
    temperature: -5
  }), 'The temperature is -5');
});

ava('.interpolate() should cast positive float to string if interpolation has context', (test) => {
  test.deepEqual(string.interpolate('Foo {{bar}} baz', {
    bar: 5.1
  }), 'Foo 5.1 baz');
});

ava('.interpolate() should cast negative float to string if interpolation has context', (test) => {
  test.deepEqual(string.interpolate('Foo {{bar}} baz', {
    bar: -3.3
  }), 'Foo -3.3 baz');
});

ava('.interpolate() should cast true to string if interpolation has context', (test) => {
  test.deepEqual(string.interpolate('Foo {{bool}} baz', {
    bool: true
  }), 'Foo true baz');
});

ava('.interpolate() should cast false to string if interpolation has context', (test) => {
  test.deepEqual(string.interpolate('Foo {{bool}} baz', {
    bool: false
  }), 'Foo false baz');
});

ava('.interpolate() should throw if a referenced variable does not exist', (test) => {
  test.throws(() => {
    string.interpolate('{{foo}}', {});
  }, 'Missing variable foo');
});

ava('.interpolate() should throw if a referenced variable is null', (test) => {
  test.throws(() => {
    string.interpolate('{{foo}}', {
      foo: null
    });
  }, 'Missing variable foo');
});

ava('.interpolate() should throw if a referenced nested variable does not exist', (test) => {
  test.throws(() => {
    string.interpolate('{{foo.bar.baz}}', {});
  }, 'Missing variable foo.bar.baz');
});

ava('.interpolate() should ignore unused data variables', (test) => {
  const result = string.interpolate('{{foo}} {{bar}}', {
    foo: 'FOO',
    bar: 'BAR',
    baz: 'BAZ',
    data: {
      hello: 'world'
    }
  });

  test.deepEqual(result, 'FOO BAR');
});

ava('.interpolate() should be able to force a string type on a dependent string', (test) => {
  test.deepEqual(string.interpolate('{{string:age}}', {
    age: 43
  }), '43');
});
