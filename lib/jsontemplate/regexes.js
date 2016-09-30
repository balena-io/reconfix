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
 * @module Reconfix.JSONTemplate.Regexes
 */

const _ = require('lodash');

/**
 * @summary RegExp string portions
 * @type {Object}
 * @constant
 * @public
 */
const REGEX = {
  capturingType: '((\\w+):)?',
  nonCapturingType: '(?:\\w+:)?',
  property: '([\\w$_\\.\\[\\]]+)',
  openDelimiters: '{{',
  closeDelimiters: '}}'
};

/**
 * @summary Unbounded interpolation RegExp
 * @type RegExp
 * @constant
 * @public
 */
exports.UNBOUNDED_INTERPOLATION = new RegExp([
  REGEX.openDelimiters,
  REGEX.capturingType,
  REGEX.property,
  REGEX.closeDelimiters
].join(''), 'g');

/**
 * @summary Lodash template interpolation RegExp
 * @type RegExp
 * @constant
 * @public
 *
 * We need to make a special regular expression without a capturing
 * group on the type section, since `_.template` will get confused
 * if there is more than one capturing group.
 */
exports.TEMPLATE_INTERPOLATION = new RegExp([
  REGEX.openDelimiters,
  REGEX.nonCapturingType,
  REGEX.property,
  REGEX.closeDelimiters
].join(''), 'g');

/**
 * @summary Bounded interpolation RegExp
 * @type RegExp
 * @constant
 * @pblic
 */
exports.BOUNDED_INTERPOLATION = new RegExp([
  '^',
  REGEX.openDelimiters,
  REGEX.capturingType,
  REGEX.property,
  REGEX.closeDelimiters,
  '$'
].join(''));

/**
 * @summary Execute interpolation regex
 * @function
 * @public
 *
 * @param {RegExp} regex - interpolation regex
 * @param {String} template - template string
 * @returns {Object} interpolation details
 *
 * @example
 * const interpolation = regexes.execute(regexes.BOUNDED_INTERPOLATION, '{{string:name}}');
 *
 * console.log(interpolation.type);
 * > 'string'
 *
 * console.log(interpolation.property);
 * > 'name'
 */
exports.execute = (regex, template) => {

  // Reset global RegExp index
  // See: http://stackoverflow.com/a/11477448/1641422
  regex.lastIndex = 0;

  const matches = regex.exec(template);
  return {
    type: _.nth(matches, 2),
    property: _.nth(matches, 3)
  };
};
