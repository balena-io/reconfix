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
 * @module Reconfix.Engine.Formats
 */

const _ = require('lodash');

/**
 * @summary Supported formats
 * @type Object
 * @constant
 * @private
 */
const SUPPORTED_FORMATS = {
  json: require('./json'),
  ini: require('./ini')
};

/**
 * @summary Check that a format type is valid
 * @function
 * @private
 *
 * @param {String} type - type
 * @throws Will throw is type is invalid
 *
 * @example
 * checkType('json');
 */
const checkType = (type) => {
  if (!_.has(SUPPORTED_FORMATS, type)) {
    throw new Error(`Unsupported type: ${type}`);
  }
};

/**
 * @summary Parse text according to a format type
 * @function
 * @public
 *
 * @param {String} type - format type
 * @param {String} text - input text
 * @returns {Object} parsed text
 *
 * @example
 * const object = formats.parse('json', '{"foo":"bar"}');
 * console.log(object.foo);
 * > 'bar'
 */
exports.parse = (type, text) => {
  checkType(type);
  return SUPPORTED_FORMATS[type].parse(text);
};

/**
 * @summary Serialise an object according to a format type
 * @function
 * @public
 *
 * @param {String} type - format type
 * @param {Object} object - input object
 * @returns {String} serialised text
 *
 * @example
 * const text = formats.serialise('ini', { foo: 'bar' });
 * console.log(text):
 * > 'foo=bar'
 */
exports.serialise = (type, object) => {
  checkType(type);
  return SUPPORTED_FORMATS[type].serialise(object);
};
