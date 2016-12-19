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
 * @module Reconfix.Domain
 */

const _ = require('lodash');

/**
 * @summary Mask an object with a domain
 * @function
 * @public
 *
 * @param {Object} object - object
 * @param {Array[]} domain - domain
 * @returns {Object} resulting object
 *
 * @example
 * const object = Domain.mask({
 *   foo: 1,
 *   bar: 2,
 *   baz: 3
 * }, [
 *   [ 'foo' ],
 *   [ 'baz' ]
 * ]);
 *
 * console.log(object);
 * > { foo: 1, baz: 3 }
 */
exports.mask = (object, domain) => {
  return _.reduce(domain, (accumulator, path) => {
    const value = _.get(object, path);

    if (_.isUndefined(value)) {
      return accumulator;
    }

    return _.set(accumulator, path, value);
  }, {});
};

/**
 * @summary Get the domain from mapping
 * @function
 * @public
 *
 * @param {Object[]} mapping - mapping
 * @returns {Array[]} domain
 *
 * @example
 * const domain = Domain.getFromMapping([
 *   {
 *     value: true,
 *     template: {
 *       foo: 1
 *     }
 *   },
 *   {
 *     value: false,
 *     template: {
 *       bar: 1
 *     }
 *   }
 * ]);
 *
 * console.log(domain);
 * > [ [ 'foo' ], [ 'bar' ] ]
 */
exports.getFromMapping = (mapping) => {
  return _.uniqWith(_.reduce(mapping, (accumulator, choice) => {
    return _.concat(accumulator, _.map(_.keys(choice.template), (key) => {
      return [ key ];
    }));
  }, []), _.isEqual);
};
