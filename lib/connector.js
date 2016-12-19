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
 * @module Reconfix.Connector
 */

const _ = require('lodash');

/**
 * @summary Built-in connectors
 * @type Object
 * @constant
 * @public
 */
exports.BUILTIN_CONNECTORS = {
  json: require('./connectors/json')
};

/**
 * @summary Get the type of a connector
 * @function
 * @private
 *
 * @param {Object} connector - connector
 * @returns {String} type - type
 *
 * @example
 * const type = Connector.getType({
 *   type: 'json',
 *   path: [ 'config.txt' ],
 *   partition: {
 *     primary: 1
 *   }
 * });
 */
exports.getType = (connector) => {
  return _.get(connector, 'type');
};

/**
 * @summary Get the options of a connector
 * @function
 * @private
 *
 * @param {Object} connector - connector
 * @returns {Object} options - options
 *
 * @example
 * const options = Connector.getOptions({
 *   type: 'json',
 *   path: [ 'config.txt' ],
 *   partition: {
 *     primary: 1
 *   }
 * });
 *
 * console.log(options);
 * > {
 * >   path: [ 'config.txt' ],
 * >   partition: {
 * >     primary: 1
 * >   }
 * > }
 */
exports.getOptions = (connector) => {
  return _.omit(connector, [ 'type' ]);
};

/**
 * @summary Set data using a connector
 * @function
 * @public
 *
 * @param {Object} connector - connector
 * @param {Object} data - data
 * @param {Object} options - options
 * @param {Object} options.connectors - available connectors
 * @returns {Promise}
 *
 * @example
 * Connector.set({
 *   type: 'json',
 *   path: [ 'config.json' ],
 *   partition: {
 *     primary: 1
 *   }
 * }, {
 *   foo: 'bar'
 * }, {
 *   connectors: Connector.BUILTIN_CONNECTORS
 * }).then(() => {
 *   console.log('Done!');
 * });
 */
exports.set = (connector, data, options) => {
  const type = exports.getType(connector);
  const executor = _.get(options.connectors, [ type, 'set' ]);

  if (!_.has(options.connectors, type)) {
    throw new Error(`Unknown connector type: "${type}"`);
  }

  if (!_.isFunction(executor)) {
    throw new Error(`Invalid connector type: "${type}", "set" is not a function`);
  }

  const connectorOptions = exports.getOptions(connector);
  return executor(connectorOptions, data);
};
