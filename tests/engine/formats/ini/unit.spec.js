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
const ini = require('../../../../lib/engine/formats/ini');

// TODO: Test ideas
// - Test a section name that starts with a period.
// - Test the informal `key[nestedkey]: value` syntax.

const shouldParse = (title, lines, object) => {
  ava.test(`formats: should parse ${title}`, (test) => {
    test.deepEqual(ini.parse(_.join(lines, '\n')), object);
  });
};

shouldParse('a property without a space before the equal sign', [
  'foo= bar '
], {
  foo: 'bar'
});

shouldParse('a property without a space after the equal sign', [
  'foo =bar '
], {
  foo: 'bar'
});

shouldParse('a property without spaces around the equal sign', [
  'foo=bar '
], {
  foo: 'bar'
});

shouldParse('an empty string property', [
  'foo = '
], {
  foo: ''
});

shouldParse('a single character string property', [
  'foo = a'
], {
  foo: 'a'
});

shouldParse('a multiple character string property', [
  'foo = bar'
], {
  foo: 'bar'
});

shouldParse('a single quote surrounded string property', [
  'foo = \'bar baz\''
], {
  foo: 'bar baz'
});

shouldParse('a double quote surrounded string property', [
  'foo = "bar baz"'
], {
  foo: 'bar baz'
});

shouldParse('a string property prefixed with a number', [
  'foo = 1bar'
], {
  foo: '1bar'
});

shouldParse('a string property suffixed with a number', [
  'foo = bar1'
], {
  foo: 'bar1'
});

shouldParse('a string property infixed with a number', [
  'foo = he23llo'
], {
  foo: 'he23llo'
});

shouldParse('a string property surrounded by numbers', [
  'foo = 34bar34'
], {
  foo: '34bar34'
});

shouldParse('a zero property', [
  'foo = 0'
], {
  foo: 0
});

shouldParse('a single quote surrounded number property', [
  'foo = \'35\''
], {
  foo: 35
});

shouldParse('a double quote surrounded number property', [
  'foo = "35"'
], {
  foo: 35
});

shouldParse('a positive integer property', [
  'foo = 1'
], {
  foo: 1
});

shouldParse('a negative integer property', [
  'foo = -5'
], {
  foo: -5
});

shouldParse('a positive float property with a zero decimal', [
  'foo = 3.0'
], {
  foo: 3
});

shouldParse('a positive float property', [
  'foo = 3.45'
], {
  foo: 3.45
});

shouldParse('a negative float property', [
  'foo = -10.5'
], {
  foo: -10.5
});

shouldParse('a truthy boolean property', [
  'foo = true'
], {
  foo: true
});

shouldParse('a falsy boolean property', [
  'foo = false'
], {
  foo: false
});

shouldParse('a capitalized truthy boolean property as a string', [
  'foo = True'
], {
  foo: 'True'
});

shouldParse('a capitalized falsy boolean property as a string', [
  'foo = False'
], {
  foo: 'False'
});

shouldParse('a string sentence containing commas', [
  'foo = The Obelisco, a national historic monument, is the one over there.'
], {
  foo: 'The Obelisco, a national historic monument, is the one over there.'
});

shouldParse('an array of strings', [
  'hello = foo,bar,baz'
], {
  hello: [
    'foo',
    'bar',
    'baz'
  ]
});

shouldParse('an array containing a single string using the square bracketed notation', [
  'hello[] = foo'
], {
  hello: [ 'foo' ]
});

shouldParse('an array of strings using the square bracketed notation', [
  'hello[] = foo',
  'hello[] = bar',
  'hello[] = baz'
], {
  hello: [
    'foo',
    'bar',
    'baz'
  ]
});

shouldParse('an array of integers', [
  'hello = 1,2,3,4'
], {
  hello: [ 1, 2, 3, 4 ]
});

shouldParse('an array of integers using the square bracketed notation', [
  'hello[] = 1',
  'hello[] = 2',
  'hello[] = 3',
  'hello[] = 4'
], {
  hello: [ 1, 2, 3, 4 ]
});

shouldParse('an ip address property', [
  'foo = 192.168.1.5'
], {
  foo: '192.168.1.5'
});

shouldParse('an empty section', [
  '[mysection]'
], {
  mysection: {}
});

shouldParse('a section title containing spaces', [
  '[my spaced section]'
], {
  'my spaced section': {}
});

shouldParse('a string property inside a section', [
  '[mysection]',
  'foo = bar'
], {
  mysection: {
    foo: 'bar'
  }
});

shouldParse('a number property inside a section', [
  '[mysection]',
  'foo = 1'
], {
  mysection: {
    foo: 1
  }
});

shouldParse('multiple properties inside a section', [
  '[mysection]',
  'foo = bar',
  'bar = baz'
], {
  mysection: {
    foo: 'bar',
    bar: 'baz'
  }
});
