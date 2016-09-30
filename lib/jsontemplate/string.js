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
 * @summary Transform value to another type
 * @function
 * @private
 *
 * @param {String} type - new type
 * @param {*} value - value to cast
 * @returns {*} casted value
 *
 * @example
 * console.log(transformValue('number', '21'));
 * > 21
 */
const transformValue = (type, value) => {
  const castFunctions = {
    number: parseFloat,
    string: String
  };

  const result = _.get(castFunctions, type, _.identity)(value);

  if (_.isNaN(result)) {
    throw new Error(`Can't convert ${value} to ${type}`);
  }

  return result;
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
    const interpolation = regexes.execute(regexes.BOUNDED_INTERPOLATION, template);
    const value = _.get(data, interpolation.property);

    if (_.isUndefined(value) || _.isNull(value)) {
      throw new Error(`Missing variable ${interpolation.property}`);
    }

    return transformValue(interpolation.type, value);
  }

  try {
    return _.template(template, {
      interpolate: regexes.TEMPLATE_INTERPOLATION
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
 * @summary Create a single property object
 * @function
 * @private
 *
 * @param {String} key - object key
 * @param {*} value - object value
 * @returns {Object} single property object
 *
 * @example
 * console.log(createSinglePropertyObject('foo', 'bar'));
 * > { foo: 'bar' }
 *
 * console.log(createSinglePropertyObject('foo.baz', 'bar'));
 * > { foo: { bar: 'baz' } }
 */
const createSinglePropertyObject = (key, value) => {
  const object = {};

  // `_.set` ensures that if `key` is a path
  // (e.g: `foo.bar.baz`), it will be expanded correctly.
  _.set(object, key, value);

  return object;
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
    const interpolation = regexes.execute(regexes.BOUNDED_INTERPOLATION, template);
    return createSinglePropertyObject(
      interpolation.property,
      transformValue(interpolation.type, string)
    );
  }

  const templateRegexString = template.replace(regexes.UNBOUNDED_INTERPOLATION, '(.+)');
  const templateRegex = new RegExp(templateRegexString);
  const allExpressions = template.match(regexes.UNBOUNDED_INTERPOLATION);
  const allValues = _.tail(templateRegex.exec(string));

  return _.reduce(_.zip(allExpressions, allValues), (data, pair) => {
    const interpolation = regexes.execute(regexes.UNBOUNDED_INTERPOLATION, _.first(pair));
    const value = _.last(pair);

    if (_.isUndefined(value)) {
      throw new Error(`No match for '${interpolation.property}'`);
    }

    _.set(data, interpolation.property, transformValue(interpolation.type, value));
    return data;
  }, {});
};
