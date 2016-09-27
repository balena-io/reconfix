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
 * @module Reconfix.Engine.JSONTemplate.Interpolation
 */

const _ = require('lodash');

/**
 * @summary Get the property name declared by an interpolation expression
 * @function
 * @public
 *
 * @description
 * In this context, an interpolation expression is a string surrounded
 * by curly brackets. The string contained inside the curly brackets
 * represents the property name.
 *
 * @param {String} expression - expression
 * @returns {(String|Undefined)} property name
 *
 * @example
 * console.log(interpolation.getInterpolationPropertyName('[foo]'));
 * > 'foo'
 *
 * @example
 * console.log(interpolation.getInterpolationPropertyName('invalid'));
 * > undefined
 */
exports.getInterpolationPropertyName = (expression) => {
  return _.nth(/^\[([\w\.$-\[\]]+)\]$/.exec(expression), 1);
};

/**
 * @summary Check if an expression represents an interpolation expression
 * @function
 * @public
 *
 * @param {String} expression - expression
 * @returns {Boolean} whether the expression is a valid interpolation expression
 *
 * @example
 * if (interpolation.isInterpolationExpression('[foo]')) {
 *   console.log('This is an interpolation expression!');
 * }
 */
exports.isInterpolationExpression = (expression) => {
  return Boolean(exports.getInterpolationPropertyName(expression));
};

/**
 * @summary Build an interpolation expression out of a string
 * @function
 * @public
 *
 * @description
 * This is the opposite of `.getInterpolationPropertyName()`.
 *
 * @param {String} string - string
 * @returns {String} interpolation expression
 *
 * @example
 * console.log(interpolation.buildInterpolationExpression('foo'));
 * > '[foo]'
 */
exports.buildInterpolationExpression = (string) => {
  const expression = `[${string}]`;

  // The best way to test that the resulting expression is valid
  // without duplicating validation is to check if it can be parsed
  // back and the resulting property name matches the original string.
  if (exports.getInterpolationPropertyName(expression) !== string) {
    throw new Error(`Invalid input: ${string}`);
  }

  return expression;
};
