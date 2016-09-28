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
 * @summary Interpolation expression RegExp
 * @type String
 * @constant
 * @private
 */
const EXPRESSION_REGEX = /{{([\w$_\.\[\]]+)}}/g;

/**
 * @summary Ensure template data is valid
 * @function
 * @private
 *
 * @param {Object} data - template data
 * @throws Will throw if data is invalid
 *
 * @example
 * ensureTemplateDataIsValid({ ... });
 * console.log('At this point, we know everything is valid');
 */
const ensureTemplateDataIsValid = (data) => {
  _.each(data, (value) => {
    if (_.isPlainObject(value)) {
      return ensureTemplateDataIsValid(value);
    }

    if (!_.isString(value)) {
      throw new Error(`Invalid data value: ${value}`);
    }
  });
};

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
 * @returns {String} interpolated string
 *
 * @example
 * console.log(interpolation.interpolateString('Hello, {{name}}!', {
 *   name: 'John Doe'
 * }));
 * > 'Hello, John Doe!'
 */
exports.interpolateString = (template, data) => {
  ensureTemplateDataIsValid(data);

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
 * @param {String} string - interpolated string
 * @returns {Object} template data
 *
 * @example
 * console.log(interpolation.deinterpolateString('Hello, {{name}}!', 'Hello, John Doe!');
 * > {
 * >   name: 'John Doe'
 * > }
 */
exports.deinterpolateString = (template, string) => {
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
