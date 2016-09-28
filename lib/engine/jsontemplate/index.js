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
 * @module Reconfix.Engine.JSONTemplate
 */

const _ = require('lodash');
const interpolation = require('./interpolation');

/**
 * @summary Compile a JSON template
 * @function
 * @public
 *
 * @param {Object} template - json template
 * @param {Object} data - template data
 * @returns {Object} compilation result
 *
 * @example
 * const result = jsontemplate.compile({
 *   greeting: 'Hello, {{name}}!'
 * }, {
 *   name: 'John Doe'
 * });
 *
 * console.log(result);
 * > {
 * >   greeting: 'Hello, John Doe!'
 * > }
 */
exports.compile = (template, data) => {
  return _.mapValues(template, (value) => {
    if (_.isPlainObject(value)) {
      return exports.compile(value, data);
    }

    if (_.isString(value)) {
      return interpolation.interpolateString(value, data);
    }

    return value;
  });
};

/**
 * @summary Decompile a JSON template
 * @function
 * @public
 *
 * @param {Object} template - json template
 * @param {Object} result - compilation result
 * @returns {Object} template data
 *
 * @example
 * const data = jsontemplate.decompile({
 *   greeting: 'Hello, {{name}}!'
 * }, {
 *   greeting: 'Hello, John Doe!'
 * });
 *
 * console.log(data);
 * > {
 * >   name: 'John Doe'
 * > }
 */
exports.decompile = (template, result) => {
  return _.reduce(template, (data, value, key) => {
    const string = _.get(result, key);

    if (_.isPlainObject(value)) {
      _.merge(data, exports.decompile(value, string));
    }

    if (_.isString(value)) {
      _.merge(data, interpolation.deinterpolateString(value, string));
    }

    return data;
  }, {});
};
