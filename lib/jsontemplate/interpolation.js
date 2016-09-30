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
 * @module Reconfix.JSONTemplate.Interpolation
 */

const _ = require('lodash');

/**
 * @summary Interpolation RegExp string
 * @type String
 * @constant
 * @private
 */
const INTERPOLATION_REGEX_STRING = '{{([\\w$_\\.\\[\\]]+)}}';

/**
 * @summary Interpolation expression RegExp
 * @type RegExp
 * @constant
 * @private
 */
const EXPRESSION_REGEX = new RegExp(INTERPOLATION_REGEX_STRING, 'g');

/**
 * @summary Absolute interpolation expression RegExp
 * @type RegExp
 * @constant
 * @private
 */
const ABSOLUTE_EXPRESSION_REGEX = new RegExp(`^${INTERPOLATION_REGEX_STRING}$`);

/**
 * @summary Interpolate a string
 * @function
 * @public
 *
 * @description
 * The gist of this function is: `(template, data) => string`
 *
 * @param {String} template - template
 * @param {Object} data - data
 * @returns {*} interpolated result
 *
 * @example
 * console.log(interpolation.interpolateString('Hello, {{name}}!', {
 *   name: 'John Doe'
 * }));
 * > 'Hello, John Doe!'
 */
exports.interpolateString = (template, data) => {
  if (ABSOLUTE_EXPRESSION_REGEX.test(template)) {
    const property = _.nth(ABSOLUTE_EXPRESSION_REGEX.exec(template), 1);
    const value = _.get(data, property);

    if (_.isUndefined(value) || _.isNull(value)) {
      throw new Error(`Missing variable ${property}`);
    }

    return value;
  }

  try {
    return _.template(template, {
      interpolate: EXPRESSION_REGEX
    })(data);

  // This is a terrible way to intercept an undefined
  // variable error to give it a better message, but
  // sadly its the best we can to still be able to re-use
  // the `_.template` functionality.
  } catch (error) {
    const undefinedExpression = _.nth(/(.*) is not defined/.exec(error.message), 1);

    if (undefinedExpression) {
      error.message = `Missing variable ${undefinedExpression}`;
    }

    throw error;
  }
};

/**
 * @summary Deinterpolate a string
 * @function
 * @public
 *
 * @description
 * The gist of this function is: `(template, string) => data`
 *
 * @param {String} template - template
 * @param {*} string - interpolated string
 * @returns {Object} template data
 *
 * @example
 * console.log(interpolation.deinterpolateString('Hello, {{name}}!', 'Hello, John Doe!');
 * > {
 * >   name: 'John Doe'
 * > }
 */
exports.deinterpolateString = (template, string) => {
  if (ABSOLUTE_EXPRESSION_REGEX.test(template)) {
    const property = _.nth(ABSOLUTE_EXPRESSION_REGEX.exec(template), 1);
    const result = {};
    _.set(result, property, string);
    return result;
  }

  const templateRegexString = template.replace(EXPRESSION_REGEX, '(.+)');
  const templateRegex = new RegExp(templateRegexString);
  const allExpressions = template.match(EXPRESSION_REGEX);
  const allValues = _.tail(templateRegex.exec(string));

  return _.reduce(_.zip(allExpressions, allValues), (data, pair) => {

    // Reset global RegExp index
    // See: http://stackoverflow.com/a/11477448/1641422
    EXPRESSION_REGEX.lastIndex = 0;

    const key = _.nth(EXPRESSION_REGEX.exec(_.first(pair)), 1);
    const value = _.last(pair);

    if (_.isUndefined(value)) {
      throw new Error(`No match for '${key}'`);
    }

    _.set(data, key, value);
    return data;
  }, {});
};
