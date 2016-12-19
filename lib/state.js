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
 * @module Reconfix.State
 */

const _ = require('lodash');
const Mapping = require('./mapping');
const Properties = require('./properties');
const Type = require('./type');

/**
 * @summary Check property value type
 * @function
 * @private
 *
 * @param {String[]} path - property path
 * @param {String[]} type - property type
 * @param {*} value - property type
 *
 * @throws Will throw if there is a type mismatch
 *
 * @example
 * checkPropertyValueType([ 'my', 'property' ], [ 'number' ], 3);
 */
const checkPropertyValueType = (path, type, value) => {

  // Every value is optional by default, therefore we don't
  // try to match "undefined" or "null" with the property types
  if (!_.isNull(value) && !_.isUndefined(value) && !Type.matchesSomeType(type, value)) {
    throw new Error(`Type mismatch for "${path}": expected ${type}, but got "${value}"`);
  }

};

/**
 * @summary Utility to reduce properties to an object
 * @function
 * @private
 *
 * @param {Object} properties - properties
 * @param {Function} callback - callback (accumulator, property, path)
 * @returns {Object} reduced object
 *
 * @example
 * const object = reducePropertiesToObject({
 *   foo: {
 *     type: [ 'number' ],
 *     mapping: [
 *       [ 'bar' ]
 *     ]
 *   }
 * }, (accumulator, property, path) => {
 *
 *   _.defaults(accumulator, {
 *     numberOfProperties: 0
 *   });
 *
 *
 *   accumulator.numberOfProperties += 1;
 *   return accumulator;
 * });
 *
 * console.log(object);
 * > { numberOfProperties: 1 }
 */
const reducePropertiesToObject = (properties, callback) => {
  const propertyPaths = Properties.getPropertyPaths(properties);
  return _.reduce(propertyPaths, (accumulator, propertyPath) => {
    const property = _.get(properties, propertyPath);
    return callback(accumulator, property, propertyPath);
  }, {});
};

/**
 * @summary Compile state with a set of properties
 * @function
 * @public
 *
 * @param {Object} properties - properties
 * @param {Object} state - state
 * @returns {Object} compiled state
 *
 * @throws Will throw if there is a type mismatch
 *
 * @example
 * const object = State.compile({
 *   property1: {
 *     type: [ 'string' ],
 *     mapping: [
 *       [ 'foo' ]
 *     ]
 *   },
 *   property2: {
 *     type: [ 'string' ],
 *     mapping: [
 *       [ 'baz' ]
 *     ]
 *   }
 * }, {
 *   property1: 'hello',
 *   property2: 'world'
 * });
 *
 * console.log(object);
 * > { foo: 'hello', bar: 'world' }
 */
exports.compile = (properties, state) => {
  return reducePropertiesToObject(properties, (accumulator, property, path) => {
    const value = _.get(state, path);
    checkPropertyValueType(path, property.type, value);
    return _.merge(accumulator, Mapping.map(property.mapping, value));
  });
};

/**
 * @summary Deompile state from a set of properties
 * @function
 * @public
 *
 * @param {Object} properties - properties
 * @param {Object} source - source
 * @returns {Object} state
 *
 * @throws Will throw if there is a type mismatch
 *
 * @example
 * const state = State.decompile({
 *   property1: {
 *     type: [ 'string' ],
 *     mapping: [
 *       [ 'foo' ]
 *     ]
 *   },
 *   property2: {
 *     type: [ 'string' ],
 *     mapping: [
 *       [ 'baz' ]
 *     ]
 *   }
 * }, {
 *   foo: 'hello',
 *   bar: 'world'
 * });
 *
 * console.log(state);
 * > { property1: 'hello', property2: 'world' }
 */
exports.decompile = (properties, source) => {
  return reducePropertiesToObject(properties, (accumulator, property, path) => {
    const value = Mapping.unmap(property.mapping, source);
    checkPropertyValueType(path, property.type, value);
    return _.set(accumulator, path, value);
  });
};
