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
 * @module Reconfix.Type
 */

const _ = require('lodash');

/**
 * @summary Type check functions by type name
 * @type Object
 * @constant
 * @private
 */
const TYPE_CHECKS = {
  number: _.isNumber,
  string: _.isString,
  boolean: _.isBoolean,
  object: _.isPlainObject,
  array: _.isArray
};

/**
 * @summary Supported types
 * @type String[]
 * @constant
 * @private
 */
const VALID_TYPES = _.keys(TYPE_CHECKS);

/**
 * @summary Check if a type is a valid type
 * @function
 * @public
 *
 * @param {String} type - type
 * @returns {Boolean} whether the type is valid
 *
 * @example
 * if (Type.isValidType('string')) {
 *   console.log('"string" is a valid type');
 * }
 */
exports.isValidType = _.partial(_.includes, VALID_TYPES);

/**
 * @summary Check if a value matches a type
 * @function
 * @public
 *
 * @param {String} type - type
 * @param {String} value - value
 * @returns {Boolean} whether `value` matches the type
 *
 * @throws Will throw if `type` is invalid
 *
 * @example
 * if (Type.matchesType('number', 3)) {
 *   console.log('3 is a number');
 * }
 */
exports.matchesType = (type, value) => {
  if (!exports.isValidType(type)) {
    throw new Error(`Invalid type: ${type}`);
  }

  return _.invoke(TYPE_CHECKS, type, value);
};

/**
 * @summary Check if a value matches any type inside a list
 * @function
 * @public
 *
 * @param {String[]} types - types
 * @param {String} value - value
 * @returns {Boolean} whether `value` matches any type
 *
 * @example
 * if (Type.matchesSomeType([ 'number', 'string' ], 3)) {
 *   console.log('3 matches some type');
 * }
 */
exports.matchesSomeType = (types, value) => {
  return _.some(types, (type) => {
    return exports.matchesType(type, value);
  });
};
