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
const interpolation = require('../../../lib/engine/jsontemplate/interpolation');

_.each([

  // -------------------------------------------------------------------
  // Top level string interpolation
  // -------------------------------------------------------------------

  {
    template: '[name]',
    data: {
      name: 'John Doe'
    },
    result: 'John Doe'
  },
  {
    template: 'Hello, [name]',
    data: {
      name: 'John Doe'
    },
    result: 'Hello, John Doe'
  },
  {
    template: 'Hello, [name]!',
    data: {
      name: 'John Doe'
    },
    result: 'Hello, John Doe!'
  },
  {
    template: 'Foo[name]Foo',
    data: {
      name: 'John Doe'
    },
    result: 'FooJohn DoeFoo'
  },

  // -------------------------------------------------------------------
  // Top level number interpolation
  // -------------------------------------------------------------------

  {
    template: '[number]',
    data: {
      number: '0'
    },
    result: '0'
  },
  {
    template: '[age]',
    data: {
      age: '17'
    },
    result: '17'
  },
  {
    template: '[age]',
    data: {
      age: '21.5'
    },
    result: '21.5'
  },
  {
    template: '[number]',
    data: {
      number: '-14'
    },
    result: '-14'
  },
  {
    template: '[number]',
    data: {
      number: '5.0'
    },
    result: '5.0'
  },

  // -------------------------------------------------------------------
  // Special characters in key
  // -------------------------------------------------------------------

  /* eslint-disable camelcase */

  {
    template: '[$name]',
    data: {
      $name: 'John Doe'
    },
    result: 'John Doe'
  },
  {
    template: '[full_name]',
    data: {
      full_name: 'John Doe'
    },
    result: 'John Doe'
  },

  /* eslint-enable camelcase */

  // -------------------------------------------------------------------
  // Nested brackets
  // -------------------------------------------------------------------

  {
    template: '[[name]]',
    data: {
      name: 'John Doe'
    },
    result: '[John Doe]'
  },

  // -------------------------------------------------------------------
  // Nested interpolation
  // -------------------------------------------------------------------

  {
    template: '[foo.bar.baz.name]',
    data: {
      foo: {
        bar: {
          baz: {
            name: 'John Doe'
          }
        }
      }
    },
    result: 'John Doe'
  }

], (testCase) => {

  ava.test(`.interpolateString() should interpolate ${testCase.template}`, (test) => {
    test.deepEqual(interpolation.interpolateString(testCase.template, testCase.data), testCase.result);
  });

});

ava.test('.interpolateString() should reject numeric data values', (test) => {
  test.throws(() => {
    interpolation.interpolateString('[foo]', {
      foo: 1
    });
  }, 'Invalid data value: 1');
});

ava.test('.interpolateString() should reject numeric nested data values', (test) => {
  test.throws(() => {
    interpolation.interpolateString('[foo.bar]', {
      foo: {
        bar: 1
      }
    });
  }, 'Invalid data value: 1');
});
