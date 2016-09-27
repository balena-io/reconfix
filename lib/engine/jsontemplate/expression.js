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

/**
 * @module Reconfix.Engine.JSONTemplate.Expression
 */

const _ = require('lodash');

/**
 * @summary Get the property name declared by an expression
 * @function
 * @public
 *
 * @description
 * In this context, an expression is a string surrounded by
 * curly brackets. The string contained inside the curly
 * brackets represents the property name.
 *
 * @param {String} expression - expression
 * @returns {(String|Undefined)} property name
 *
 * @example
 * console.log(expression.getPropertyName('[foo]'));
 * > 'foo'
 *
 * @example
 * console.log(expression.getPropertyName('invalid'));
 * > undefined
 */
exports.getPropertyName = (expression) => {
  return _.nth(/^\[([\w\.$-\[\]]+)\]$/.exec(expression), 1);
};

/**
 * @summary Check if a string represents an expression
 * @function
 * @public
 *
 * @param {String} string - string
 * @returns {Boolean} whether the expression is a valid expression
 *
 * @example
 * if (expression.isExpression('[foo]')) {
 *   console.log('This is an expression!');
 * }
 */
exports.isExpression = (string) => {
  return Boolean(exports.getPropertyName(string));
};

/**
 * @summary Build an expression out of a string
 * @function
 * @public
 *
 * @description
 * This is the opposite of `.getPropertyName()`.
 *
 * @param {String} string - string
 * @returns {String} expression
 *
 * @example
 * console.log(expression.buildExpression('foo'));
 * > '[foo]'
 */
exports.buildExpression = (string) => {
  const expression = `[${string}]`;

  // The best way to test that the resulting expression is valid
  // without duplicating validation is to check if it can be parsed
  // back and the resulting property name matches the original string.
  if (exports.getPropertyName(expression) !== string) {
    throw new Error(`Invalid input: ${string}`);
  }

  return expression;
};
