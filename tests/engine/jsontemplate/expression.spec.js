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
const expression = require('../../../lib/engine/jsontemplate/expression');

_.each([

  // Common malformed cases
  [ '[]', undefined ],
  [ '', undefined ],
  [ 1, undefined ],
  [ [], undefined ],
  [ [ 'foo' ], undefined ],
  [ [ 1 ], undefined ],
  [ {}, undefined ],

  // String expressions
  [ '[name]', 'name' ],
  [ '[NAME]', 'NAME' ],

  // Number expressions
  [ '[1]', '1' ],
  [ '[0]', '0' ],
  [ '[3.14]', '3.14' ],
  [ '[-1]', '-1' ],

  // Expressions containing dots
  [ '[foo.bar]', 'foo.bar' ],
  [ '[foo.bar.baz]', 'foo.bar.baz' ],
  [ '[foo.1]', 'foo.1' ],
  [ '[1.foo]', '1.foo' ],
  [ '[.foo]', '.foo' ],
  [ '[..foo]', '..foo' ],

  // Expressions containing hyphens
  [ '[foo-bar]', 'foo-bar' ],
  [ '[-foo]', '-foo' ],
  [ '[foo-]', 'foo-' ],

  // Expressions containing underscores
  [ '[foo_bar]', 'foo_bar' ],
  [ '[_foo]', '_foo' ],
  [ '[foo_]', 'foo_' ],

  // Expressions containing dollar signs
  [ '[foo$bar]', 'foo$bar' ],
  [ '[$foo]', '$foo' ],
  [ '[foo$]', 'foo$' ],

  // Expressions containing nested brackets
  [ '[[name]]', '[name]' ],
  [ '[[[name]]]', '[[name]]' ],

  // Expressions containing spaces
  [ '[foo bar]', undefined ],
  [ '[ foo]', undefined ],
  [ '[foo ]', undefined ],
  [ '[ foo ]', undefined ],

  // Invalid expressions
  [ 'name', undefined ],
  [ 'NAME', undefined ],
  [ '[name', undefined ],
  [ 'name]', undefined ],
  [ 'foo [name]', undefined ],
  [ '[name] foo', undefined ],
  [ 'foo [name] foo', undefined ]

], (data) => {
  const testCase = _.first(data);
  const expected = _.last(data);

  ava.test(`.getPropertyName() should return ${expected} for ${testCase}`, (test) => {
    test.deepEqual(expression.getPropertyName(testCase), expected);
  });

  if (!_.isUndefined(expected)) {
    ava.test(`.buildExpression() should return ${testCase} for ${expected}`, (test) => {
      test.deepEqual(expression.buildExpression(expected), testCase);
    });
  }

  ava.test(`.isExpression() should return ${Boolean(expected)} for ${testCase}`, (test) => {
    test.is(expression.isExpression(testCase), Boolean(expected));
  });
});

ava.test('.buildExpression() should throw if input is not a string', (test) => {
  test.throws(() => {
    expression.buildExpression([ 'f', 'o', 'o' ]);
  }, 'Invalid input: f,o,o');
});

ava.test('.buildExpression() should throw if string contains spaces', (test) => {
  test.throws(() => {
    expression.buildExpression('foo bar');
  }, 'Invalid input: foo bar');
});
