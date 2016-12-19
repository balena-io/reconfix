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
 * @module Reconfix.Properties
 */

const _ = require('lodash');
const Mapping = require('./mapping');

/**
 * @summary Check if a property is a leaf property
 * @function
 * @public
 *
 * @param {Object} property - property
 * @returns {Boolean} whether the property is a leaf property
 *
 * @example
 * if (Properties.isLeafProperty({
 *   type: [ 'number' ],
 *   mapping: [
 *     [ 'foo' ]
 *   ]
 * })) {
 *   console.log('This is a leaf property');
 * }
 */
exports.isLeafProperty = (property) => {

  // There might be a non-leaf property containing a
  // children called "type", so we check the actual
  // type of "type" to protect ourselves from that ambiguity
  return _.isArray(_.get(property, 'type'));

};

/**
 * @summary List all property paths
 * @function
 * @private
 *
 * @param {Object} properties - properties
 * @returns {Array[]} property paths
 *
 * @example
 * const paths = Properties.listPropertyPaths({
 *   foo: {
 *     type: [ 'number' ]
 *   },
 *   bar: {
 *     type: [ 'number' ]
 *   },
 *   baz: {
 *     type: [ 'number' ]
 *   }
 * });
 *
 * console.log(paths);
 * > [ [ 'foo' ], [ 'bar' ], [ 'baz' ] ]
 */
exports.listPropertyPaths = (properties) => {
  return _.reduce(properties, (accumulator, property, name) => {
    return _.concat(accumulator, _.attempt(() => {
      if (exports.isLeafProperty(property)) {
        return [ [ name ] ];
      }

      return _.map(exports.listPropertyPaths(property), (path) => {
        return _.concat([ name ], path);
      });
    }));
  }, []);
};

/**
 * @summary Get mapping from property
 * @function
 * @private
 *
 * @param {Object} properties - properties
 * @param {Array[]} property - property path
 * @returns {Array[]} mapping
 *
 * @example
 * const mapping = Properties.getPropertyMapping({
 *   foo: {
 *     type: [ 'string' ],
 *     mapping: [
 *       [ 'bar' ]
 *     ]
 *   }
 * }, [
 *   'foo'
 * ]);
 *
 * console.log(mapping);
 * > [ [ 'bar' ] ]
 */
exports.getPropertyMapping = (properties, property) => {
  return _.get(properties, _.concat(property, [ 'mapping' ]));
};

/**
 * @summary Get all property paths
 * @function
 * @public
 *
 * @description
 * This function is wrapper around `Properties.listPropertyPaths()`, which
 * makes sure the returned properties are in an order such that Reconfix
 * can operate on top of them without conflicts.
 *
 * @param {Object} properties - properties
 * @returns {Array[]} property paths
 *
 * @example
 * const paths = Properties.getPropertyPaths({
 *   foo: {
 *     type: [ 'number' ],
 *     mapping: [
 *       [ 'value1' ]
 *     ]
 *   },
 *   bar: {
 *     type: [ 'number' ],
 *     mapping: [
 *       {
 *         value: 1,
 *         template: {
 *           bar: true
 *         }
 *       },
 *       {
 *         value: 0,
 *         template: {
 *           bar: false
 *         }
 *       }
 *     ]
 *   }
 * });
 *
 * console.log(paths);
 * > [ [ 'bar' ], [ 'foo' ] ]
 */
exports.getPropertyPaths = (properties) => {
  return exports.listPropertyPaths(properties).sort((property1, property2) => {
    const mapping1 = exports.getPropertyMapping(properties, property1);
    const mapping2 = exports.getPropertyMapping(properties, property2);

    // Prefer mixed mappings above everything

    if (Mapping.isMixedMapping(mapping1)) {
      return -1;
    }

    if (Mapping.isMixedMapping(mapping2)) {
      return 1;
    }

    // Prefer template mappings above direct mappings

    if (Mapping.isTemplateMapping(mapping1)) {
      return -1;
    }

    if (Mapping.isTemplateMapping(mapping2)) {
      return 1;
    }

    return 0;
  });
};
