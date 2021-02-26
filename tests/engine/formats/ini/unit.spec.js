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

const ini = require('../../../../lib/engine/formats/ini');
const utils = require('../_utils');

// TODO: Test ideas
// - Test a section name that starts with a period.
// - Test the informal `key[nestedkey]: value` syntax.

utils.testParse(ini, 'a property without a space before the equal sign', [
  'foo= bar '
], {
  foo: 'bar'
});

utils.testParse(ini, 'a property without a space after the equal sign', [
  'foo =bar '
], {
  foo: 'bar'
});

utils.testParse(ini, 'a property with spaces around the equal sign', [
  'foo = bar '
], {
  foo: 'bar'
});

utils.testBidirectional(ini, 'an empty string property', [
  'foo='
], {
  foo: ''
});

utils.testBidirectional(ini, 'a single character string property', [
  'foo=a'
], {
  foo: 'a'
});

utils.testBidirectional(ini, 'a multiple character string property', [
  'foo=bar'
], {
  foo: 'bar'
});

utils.testParse(ini, 'a single quote surrounded string property', [
  'foo=\'bar baz\''
], {
  foo: 'bar baz'
});

utils.testParse(ini, 'a double quote surrounded string property', [
  'foo="bar baz"'
], {
  foo: 'bar baz'
});

utils.testBidirectional(ini, 'a string property prefixed with a number', [
  'foo=1bar'
], {
  foo: '1bar'
});

utils.testBidirectional(ini, 'a string property suffixed with a number', [
  'foo=bar1'
], {
  foo: 'bar1'
});

utils.testBidirectional(ini, 'a string property infixed with a number', [
  'foo=he23llo'
], {
  foo: 'he23llo'
});

utils.testBidirectional(ini, 'a string property surrounded by numbers', [
  'foo=34bar34'
], {
  foo: '34bar34'
});

utils.testBidirectional(ini, 'a string property with a trailing hash', [
  'foo=bar#'
], {
  foo: 'bar#'
});

utils.testBidirectional(ini, 'a string property with a leading hash', [
  'foo=#bar'
], {
  foo: '#bar'
});

utils.testBidirectional(ini, 'a zero property', [
  'foo=0'
], {
  foo: 0
});

utils.testParse(ini, 'a single quote surrounded number property', [
  'foo=\'35\''
], {
  foo: 35
});

utils.testParse(ini, 'a double quote surrounded number property', [
  'foo="35"'
], {
  foo: 35
});

utils.testBidirectional(ini, 'a positive integer property', [
  'foo=1'
], {
  foo: 1
});

utils.testBidirectional(ini, 'a negative integer property', [
  'foo=-5'
], {
  foo: -5
});

utils.testParse(ini, 'a positive float property with a zero decimal', [
  'foo=3.0'
], {
  foo: 3
});

utils.testBidirectional(ini, 'a positive float property', [
  'foo=3.45'
], {
  foo: 3.45
});

utils.testBidirectional(ini, 'a negative float property', [
  'foo=-10.5'
], {
  foo: -10.5
});

utils.testBidirectional(ini, 'a truthy boolean property', [
  'foo=true'
], {
  foo: true
});

utils.testBidirectional(ini, 'a falsy boolean property', [
  'foo=false'
], {
  foo: false
});

utils.testBidirectional(ini, 'a capitalized truthy boolean property as a string', [
  'foo=True'
], {
  foo: 'True'
});

utils.testBidirectional(ini, 'a capitalized falsy boolean property as a string', [
  'foo=False'
], {
  foo: 'False'
});

utils.testBidirectional(ini, 'a string sentence containing commas', [
  'foo=The Obelisco, a national historic monument, is the one over there.'
], {
  foo: 'The Obelisco, a national historic monument, is the one over there.'
});

utils.testBidirectional(ini, 'a list of strings using comma notation', [
  'hello=foo,bar,baz'
], {
  hello: 'foo,bar,baz'
});

utils.testBidirectional(ini, 'an array of strings', [
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

utils.testBidirectional(ini, 'an array containing a single string', [
  'hello[]=foo'
], {
  hello: [ 'foo' ]
});

utils.testBidirectional(ini, 'a list of integers using comma notation', [
  'hello=1,2,3,4'
], {
  hello: '1,2,3,4'
});

utils.testBidirectional(ini, 'an array of integers', [
  'hello[]=1',
  'hello[]=2',
  'hello[]=3',
  'hello[]=4'
], {
  hello: [ 1, 2, 3, 4 ]
});

utils.testBidirectional(ini, 'an ip address property', [
  'foo=192.168.1.5'
], {
  foo: '192.168.1.5'
});

utils.testParse(ini, 'an empty section', [
  '[mysection]'
], {
  mysection: {}
});

utils.testParse(ini, 'a section title containing spaces', [
  '[my spaced section]'
], {
  'my spaced section': {}
});

utils.testBidirectional(ini, 'a string property inside a section', [
  '[mysection]',
  'foo=bar'
], {
  mysection: {
    foo: 'bar'
  }
});

utils.testBidirectional(ini, 'a number property inside a section', [
  '[mysection]',
  'foo=1'
], {
  mysection: {
    foo: 1
  }
});

utils.testBidirectional(ini, 'multiple properties inside a section', [
  '[mysection]',
  'foo=bar',
  'bar=baz'
], {
  mysection: {
    foo: 'bar',
    bar: 'baz'
  }
});
