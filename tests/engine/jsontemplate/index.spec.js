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
const jsontemplate = require('../../../lib/engine/jsontemplate');

const testCompile = (title, template, data, expected) => {
  ava.test(`should compile ${title}`, (test) => {
    test.deepEqual(jsontemplate.compile(template, data), expected);
  });
};

testCompile('a single top-level independent string property', {
  person: '[name]'
}, {
  name: 'John Doe'
}, {
  person: 'John Doe'
});

testCompile('a single top-level dependent string property', {
  greeting: 'Hello, [name]'
}, {
  name: 'John Doe'
}, {
  greeting: 'Hello, John Doe'
});

testCompile('a single nested independent string property', {
  data: {
    person: '[name]'
  }
}, {
  name: 'John Doe'
}, {
  data: {
    person: 'John Doe'
  }
});

testCompile('a single top-level independent number property', {
  magicNumber: '[age]'
}, {
  age: '17'
}, {
  magicNumber: '17'
});
