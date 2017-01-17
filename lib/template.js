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
 * @module Reconfix.Template
 */

const _ = require('lodash');
const Type = require('./type');

/**
 * @summary "Type wildcard" regular expression
 * @type String
 * @constant
 * @private
 */
const REGEX_TYPE_WILDCARD = /^\[\[([a-z|]+)\]\]$/;

/**
 * @summary Check if a string represents a "type wildcard"
 * @function
 * @private
 *
 * @param {String} string - string
 * @returns {Boolean} whether the string represents a "type wildcard"
 *
 * @example
 * if (Template.isTypeWildcard('[[string]]')) {
 *   console.log('This is a type wildcard');
 * }
 */
exports.isTypeWildcard = (string) => {
  return REGEX_TYPE_WILDCARD.test(string);
};

/**
 * @summary Get "type wildcard" type
 * @function
 * @private
 *
 * @param {String} string - string
 * @returns {(String[]|Undefined)} wildcard type, if any
 *
 * @example
 * const type = Template.getWildcardType('[[string|number]]');
 * console.log(type);
 * > [ 'string', 'number' ]
 */
exports.getWildcardType = (string) => {
  const type = _.nth(REGEX_TYPE_WILDCARD.exec(string), 1);

  if (_.isUndefined(type)) {
    return;
  }

  return _.split(type, '|');
};

/**
 * @summary Match a template with an object
 * @function
 * @public
 *
 * @param {Object} object - object
 * @param {Object} template - template
 * @returns {Boolean} whether the template matches the object
 *
 * @example
 * Template.matches({
 *   foo: {
 *      bar: 'baz'
 *   }
 * }, {
 *   foo: {
 *     bar: '[[string]]'
 *   }
 * });
 * > true
 */
exports.matches = (object, template) => {
  return _.every(template, (value, key) => {
    const objectValue = _.get(object, key);

    if (exports.isTypeWildcard(value)) {
      const wildcardType = exports.getWildcardType(value);
      return Type.matchesSomeType(wildcardType, objectValue);
    }

    if (_.isArray(value)) {
      return _.isEmpty(_.differenceWith(objectValue, value, _.isEqual));
    }

    if (_.isPlainObject(value)) {
      return exports.matches(objectValue, value);
    }

    return _.isEqual(objectValue, value);
  });
};

/**
 * @summary Get template degree
 * @function
 * @public
 *
 * @description
 * The degree of a template is calculated based on
 * its number of keys.
 *
 * @param {Object} template - mplate
 * @returns {Number} template degree
 *
 * @example
 * const degree = Template.getTemplateDegree({
 *   foo: {
 *     bar: {
 *       baz: 'qux'
 *     }
 *   }
 * });
 *
 * console.log(degree);
 * > 3
 */
exports.getTemplateDegree = (template) => {
  return _.reduce(template, (accumulator, value) => {
    const nestedKeys = _.isPlainObject(value)
      ? exports.getTemplateDegree(value) : 0;

    return accumulator + nestedKeys + 1;
  }, 0);
};

/**
 * @summary Get highest degree matching templates for object
 * @function
 * @public
 *
 * @param {Object} object - object
 * @param {Object[]} templates - templates
 * @returns {Object[]} highest degree matching templates
 *
 * @example
 * const result = Template.getHighestDegreeMatchingTemplates({
 *   foo: 1,
 *   bar: 2,
 *   baz: 3
 * }, [
 *   {
 *     foo: 1
 *   },
 *   {
 *     foo: 1,
 *     bar: 2
 *   }
 * ]);
 *
 * console.log(result);
 * > { foo: 1, bar: 2 }
 */
exports.getHighestDegreeMatchingTemplates = (object, templates) => {
  const matchingTemplates = _.filter(templates, _.partial(exports.matches, object));

  const highestMatchingDegree = _.reduce(matchingTemplates, (accumulator, template) => {
    return _.max([ exports.getTemplateDegree(template), accumulator ]);
  }, 0);

  return _.filter(matchingTemplates, (template) => {
    return exports.getTemplateDegree(template) === highestMatchingDegree;
  });
};
