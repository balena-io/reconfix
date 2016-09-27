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

const parseIniLines = (lines) => {
  return _.join(_.concat(lines, ''), '\n');
};

const testParseFor = (title, lines, object) => {
  ava.test(`formats: should parse ${title}`, (test) => {
    test.deepEqual(ini.parse(parseIniLines(lines)), object);
  });
};

const testSerializeFor = (title, lines, object) => {
  ava.test(`formats: should serialise ${title}`, (test) => {
    test.deepEqual(ini.serialise(object), parseIniLines(lines));
  });
};

const testBidirectionalFor = (title, lines, object) => {
  testParseFor(title, lines, object);
  testSerializeFor(title, lines, object);
};

testParseFor('a property without a space before the equal sign', [
  'foo= bar '
], {
  foo: 'bar'
});

testParseFor('a property without a space after the equal sign', [
  'foo =bar '
], {
  foo: 'bar'
});

testParseFor('a property with spaces around the equal sign', [
  'foo = bar '
], {
  foo: 'bar'
});

testBidirectionalFor('an empty string property', [
  'foo='
], {
  foo: ''
});

testBidirectionalFor('a single character string property', [
  'foo=a'
], {
  foo: 'a'
});

testBidirectionalFor('a multiple character string property', [
  'foo=bar'
], {
  foo: 'bar'
});

testParseFor('a single quote surrounded string property', [
  'foo=\'bar baz\''
], {
  foo: 'bar baz'
});

testParseFor('a double quote surrounded string property', [
  'foo="bar baz"'
], {
  foo: 'bar baz'
});

testBidirectionalFor('a string property prefixed with a number', [
  'foo=1bar'
], {
  foo: '1bar'
});

testBidirectionalFor('a string property suffixed with a number', [
  'foo=bar1'
], {
  foo: 'bar1'
});

testBidirectionalFor('a string property infixed with a number', [
  'foo=he23llo'
], {
  foo: 'he23llo'
});

testBidirectionalFor('a string property surrounded by numbers', [
  'foo=34bar34'
], {
  foo: '34bar34'
});

testBidirectionalFor('a zero property', [
  'foo=0'
], {
  foo: 0
});

testParseFor('a single quote surrounded number property', [
  'foo=\'35\''
], {
  foo: 35
});

testParseFor('a double quote surrounded number property', [
  'foo="35"'
], {
  foo: 35
});

testBidirectionalFor('a positive integer property', [
  'foo=1'
], {
  foo: 1
});

testBidirectionalFor('a negative integer property', [
  'foo=-5'
], {
  foo: -5
});

testParseFor('a positive float property with a zero decimal', [
  'foo=3.0'
], {
  foo: 3
});

testBidirectionalFor('a positive float property', [
  'foo=3.45'
], {
  foo: 3.45
});

testBidirectionalFor('a negative float property', [
  'foo=-10.5'
], {
  foo: -10.5
});

testBidirectionalFor('a truthy boolean property', [
  'foo=true'
], {
  foo: true
});

testBidirectionalFor('a falsy boolean property', [
  'foo=false'
], {
  foo: false
});

testBidirectionalFor('a capitalized truthy boolean property as a string', [
  'foo=True'
], {
  foo: 'True'
});

testBidirectionalFor('a capitalized falsy boolean property as a string', [
  'foo=False'
], {
  foo: 'False'
});

testBidirectionalFor('a string sentence containing commas', [
  'foo=The Obelisco, a national historic monument, is the one over there.'
], {
  foo: 'The Obelisco, a national historic monument, is the one over there.'
});

testBidirectionalFor('a list of strings using comma notation', [
  'hello=foo,bar,baz'
], {
  hello: 'foo,bar,baz'
});

testBidirectionalFor('an array of strings', [
  'hello[]=foo',
  'hello[]=bar',
  'hello[]=baz'
], {
  hello: [
    'foo',
    'bar',
    'baz'
  ]
});

testBidirectionalFor('an array containing a single string', [
  'hello[]=foo'
], {
  hello: [ 'foo' ]
});

testBidirectionalFor('a list of integers using comma notation', [
  'hello=1,2,3,4'
], {
  hello: '1,2,3,4'
});

testBidirectionalFor('an array of integers', [
  'hello[]=1',
  'hello[]=2',
  'hello[]=3',
  'hello[]=4'
], {
  hello: [ 1, 2, 3, 4 ]
});

testBidirectionalFor('an ip address property', [
  'foo=192.168.1.5'
], {
  foo: '192.168.1.5'
});

testParseFor('an empty section', [
  '[mysection]'
], {
  mysection: {}
});

testParseFor('a section title containing spaces', [
  '[my spaced section]'
], {
  'my spaced section': {}
});

testBidirectionalFor('a string property inside a section', [
  '[mysection]',
  'foo=bar'
], {
  mysection: {
    foo: 'bar'
  }
});

testBidirectionalFor('a number property inside a section', [
  '[mysection]',
  'foo=1'
], {
  mysection: {
    foo: 1
  }
});

testBidirectionalFor('multiple properties inside a section', [
  '[mysection]',
  'foo=bar',
  'bar=baz'
], {
  mysection: {
    foo: 'bar',
    bar: 'baz'
  }
});
