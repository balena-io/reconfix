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
 * @module Reconfix.JSONTemplate.String
 */

const _ = require('lodash');
const regexes = require('./regexes');

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
 * console.log(string.interpolate('Hello, {{name}}!', {
 *   name: 'John Doe'
 * }));
 * > 'Hello, John Doe!'
 */
exports.interpolate = (template, data) => {
  if (regexes.BOUNDED_INTERPOLATION.test(template)) {
    const property = _.nth(regexes.BOUNDED_INTERPOLATION.exec(template), 1);
    const value = _.get(data, property);

    if (_.isUndefined(value) || _.isNull(value)) {
      throw new Error(`Missing variable ${property}`);
    }

    return value;
  }

  try {
    return _.template(template, {
      interpolate: regexes.UNBOUNDED_INTERPOLATION
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
 * console.log(string.deinterpolate('Hello, {{name}}!', 'Hello, John Doe!');
 * > {
 * >   name: 'John Doe'
 * > }
 */
exports.deinterpolate = (template, string) => {
  if (regexes.BOUNDED_INTERPOLATION.test(template)) {
    const property = _.nth(regexes.BOUNDED_INTERPOLATION.exec(template), 1);
    const result = {};
    _.set(result, property, string);
    return result;
  }

  const templateRegexString = template.replace(regexes.UNBOUNDED_INTERPOLATION, '(.+)');
  const templateRegex = new RegExp(templateRegexString);
  const allExpressions = template.match(regexes.UNBOUNDED_INTERPOLATION);
  const allValues = _.tail(templateRegex.exec(string));

  return _.reduce(_.zip(allExpressions, allValues), (data, pair) => {

    // Reset global RegExp index
    // See: http://stackoverflow.com/a/11477448/1641422
    regexes.UNBOUNDED_INTERPOLATION.lastIndex = 0;

    const key = _.nth(regexes.UNBOUNDED_INTERPOLATION.exec(_.first(pair)), 1);
    const value = _.last(pair);

    if (_.isUndefined(value)) {
      throw new Error(`No match for '${key}'`);
    }

    _.set(data, key, value);
    return data;
  }, {});
};
