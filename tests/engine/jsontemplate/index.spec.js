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

const testBidirectionalCompilation = (title, template, data, result) => {
  ava.test(`.compile() should compile ${title}`, (test) => {
    test.deepEqual(jsontemplate.compile(template, data), result);
  });

  ava.test(`.decompile() should decompile ${title}`, (test) => {
    test.deepEqual(jsontemplate.decompile(template, result), data);
  });
};

testBidirectionalCompilation('a single top-level independent string property', {
  person: '{{name}}'
}, {
  name: 'John Doe'
}, {
  person: 'John Doe'
});

testBidirectionalCompilation('a single top-level dependent string property', {
  greeting: 'Hello, {{name}}'
}, {
  name: 'John Doe'
}, {
  greeting: 'Hello, John Doe'
});

testBidirectionalCompilation('a single nested independent string property', {
  data: {
    person: '{{name}}'
  }
}, {
  name: 'John Doe'
}, {
  data: {
    person: 'John Doe'
  }
});

testBidirectionalCompilation('a single top-level independent number property', {
  magicNumber: '{{age}}'
}, {
  age: '17'
}, {
  magicNumber: '17'
});

testBidirectionalCompilation('a single top-level dependent number property', {
  age: 'My age is {{age}}'
}, {
  age: '21'
}, {
  age: 'My age is 21'
});

testBidirectionalCompilation('multiple independent properties', {
  profile: {
    fullName: '{{name}}',
    age: '{{age}}',
    jobTitle: '{{job}}'
  }
}, {
  name: 'John Doe',
  age: '42',
  job: 'Software Engineer'
}, {
  profile: {
    fullName: 'John Doe',
    age: '42',
    jobTitle: 'Software Engineer'
  }
});

testBidirectionalCompilation('multiple nested independent properties', {
  profile: {
    fullName: '{{person.name}}',
    age: '{{person.age}}',
    jobTitle: '{{person.job}}'
  }
}, {
  person: {
    name: 'John Doe',
    age: '42',
    job: 'Software Engineer'
  }
}, {
  profile: {
    fullName: 'John Doe',
    age: '42',
    jobTitle: 'Software Engineer'
  }
});
