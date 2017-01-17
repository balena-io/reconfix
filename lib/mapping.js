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
 * @module Reconfix.Mapping
 */

const _ = require('lodash');
const Domain = require('./domain');
const Template = require('./template');

/**
 * @summary Get relationships for value
 * @function
 * @private
 *
 * @param {Array} mapping - mapping
 * @param {*} value - value
 * @returns {Object[]} relationships that apply to value
 *
 * @example
 * const relationships = Mapping.getRelationshipsForValue([
 *   {
 *     value: 'foo',
 *     template: {
 *       foo: 1
 *     }
 *   },
 *   {
 *     value: 'bar',
 *     template: {
 *       bar: 1
 *     }
 *   }
 * ], 'foo');
 *
 * console.log(relationships);
 * > { value: 'foo', template: { foo: 1 } }
 */
exports.getRelationshipsForValue = (mapping, value) => {
  return _.filter(mapping, (relationship) => {
    return _.isEqual(relationship.value, value);
  });
};

/**
 * @summary Get the value associated with a template
 * @function
 * @private
 *
 * @param {Array} mapping - mapping
 * @param {Object} template - template
 * @returns {*} value
 *
 * @example
 * const value = Mapping.getTemplateValue([
 *   {
 *     value: 'foo',
 *     template: {
 *       foo: 1
 *     }
 *   },
 *   {
 *     value: 'bar',
 *     template: {
 *       bar: 1
 *     }
 *   }
 * ], {
 *   foo: 1
 * });
 *
 * console.log(value);
 * > 'foo'
 */
exports.getTemplateValue = (mapping, template) => {
  const values = _.chain(mapping)
    .filter((relationship) => {
      return _.isEqual(template, relationship.template);
    })
    .map('value')
    .uniqWith(_.isEqual)
    .value();

  if (_.size(values) > 1) {
    throw new Error(`Ambiguous duplicated template: ${JSON.stringify(template)}`);
  }

  return _.first(values);
};

/**
 * @summary Map a value to an object
 * @function
 * @public
 *
 * @param {Array[]} mapping - mapping
 * @param {*} value - value
 * @returns {Object} mapped object
 *
 * @throws Will throw if value applies to more than one relationship
 *
 * @example
 * const object = Mapping.map([
 *   [ 'foo' ]
 * ], 'bar');
 *
 * console.log(object);
 * > { foo: 'bar' }
 *
 * @example
 * const object = Mapping.map([
 *   [ 'foo' ],
 *   [ 'bar' ],
 *   [ 'baz' ]
 * ], '3');
 *
 * console.log(object);
 * > { foo: 3, bar: 3, baz: 3 }
 */
exports.map = (mapping, value) => {
  const relationships = exports.getRelationshipsForValue(mapping, value);

  if (_.size(relationships) > 1) {
    throw new Error(`Ambiguous mapping for value: ${value}`);
  }

  return _.reduce(mapping, (accumulator, relationship) => {
    if (_.isPlainObject(relationship)) {
      if (_.isEqual(relationship.value, value)) {
        return _.assign(accumulator, relationship.template);
      }

      return accumulator;
    }

    if (!_.isNull(value)) {
      return _.set(accumulator, relationship, value);
    }

    return accumulator;
  }, {});
};

/**
 * @summary Unmap a value from an object
 * @function
 * @public
 *
 * @param {Array[]} mapping - mapping
 * @param {Object} object - object
 * @returns {*} value
 *
 * @throws Will throw if there is an unmapping ambiguity
 * @throws Will throw if no match was found
 *
 * @example
 * const value = Mapping.unmap([
 *   [ 'foo' ]
 * ], {
 *   foo: 'bar'
 * });
 *
 * console.log(value);
 * > 'bar'
 *
 * @example
 * const value = Mapping.map([
 *   [ 'foo' ],
 *   [ 'bar' ],
 *   [ 'baz' ]
 * ], {
 *   foo: 3,
 *   bar: 3,
 *   baz: 3
 * });
 *
 * console.log(value);
 * > 3
 */
exports.unmap = (mapping, object) => {
  const matchingTemplates = Template.getHighestDegreeMatchingTemplates(
    Domain.mask(object, Domain.getFromMapping(mapping)),
    _.map(_.filter(mapping, _.isPlainObject), 'template')
  );

  const templateValues = _.map(matchingTemplates, _.partial(exports.getTemplateValue, mapping));

  const directValues = _.map(_.reject(mapping, _.isPlainObject), (relationship) => {

    // We return `null` instead of `undefined` since
    // `undefined` is not a valid JSON keyword
    return _.get(object, relationship, null);

  });

  const values = _.uniqWith(_.concat(templateValues, directValues), _.isEqual);

  if (_.size(values) > 1) {
    throw new Error(`Ambiguous values: ${values}`);
  }

  if (_.isEmpty(values)) {
    throw new Error('No match found');
  }

  return _.first(values);
};

/**
 * @summary Check if a mapping is a template mapping
 * @function
 * @public
 *
 * @description
 * This function will return false is mapping is mixed.
 *
 * @param {(Array[]|Object[])} mapping - mapping
 * @returns {Boolean} whether the mapping is a direct mapping
 *
 * @example
 * if (Mapping.isTemplateMapping([
 *   {
 *     value: 1,
 *     template: {
 *       foo: true
 *     }
 *   },
 *   {
 *     value: 0,
 *     template: {
 *       foo: false
 *     }
 *   }
 * ])) {
 *   console.log('This is a template mapping');
 * }
 */
exports.isTemplateMapping = (mapping) => {
  return _.every(_.map(mapping, _.isPlainObject));
};

/**
 * @summary Check if a mapping is a direct mapping
 * @function
 * @public
 *
 * @description
 * This function will return false is mapping is mixed.
 *
 * @param {(Array[]|Object[])} mapping - mapping
 * @returns {Boolean} whether the mapping is a direct mapping
 *
 * @example
 * if (Mapping.isDirectMapping([
 *   [ 'foo' ]
 * ])) {
 *   console.log('This is a direct mapping');
 * }
 */
exports.isDirectMapping = (mapping) => {
  return _.every(_.map(mapping, _.isArray));
};

/**
 * @summary Check if a mapping is a mixed mapping
 * @function
 * @public
 *
 * @param {(Array[]|Object[])} mapping - mapping
 * @returns {Boolean} whether the mapping is a mixed mapping
 *
 * @example
 * if (Mapping.isMixedMapping([
 *   [ 'foo' ],
 *   {
 *     value: 1,
 *     template: {
 *       foo: true
 *     }
 *   },
 *   {
 *     value: 0,
 *     template: {
 *       foo: false
 *     }
 *   }
 * ])) {
 *   console.log('This is a mixed mapping');
 * }
 */
exports.isMixedMapping = (mapping) => {
  return _.every([
    _.some(_.map(mapping, _.isArray)),
    _.some(_.map(mapping, _.isPlainObject))
  ]);
};
