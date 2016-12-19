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
 * @module Reconfix.Configuration
 */

const _ = require('lodash');
const State = require('./state');

/**
 * @summary Generate configuration
 * @function
 * @public
 *
 * @param {Object} schema - schema
 * @param {Object} state - state
 * @returns {Object} configuration
 *
 * @example
 * const configuration = Configuration.generate({
 *   config_json: {
 *     connector: {
 *       type: 'json'
 *       path: [ 'config.json' ],
 *       partition: {
 *         primary: 4,
 *         logical: 1
 *       }
 *     },
 *     properties: {
 *       network: {
 *         ssid: {
 *           type: [ 'string' ],
 *           mapping: [
 *             [ 'network', 'default', 'ssid' ]
 *           ]
 *         }
 *       }
 *     }
 *   }
 * }, {
 *   network: {
 *     ssid: 'mynetwork'
 *   }
 * });
 *
 * console.log(configuration);
 * > {
 * >   config_json: {
 * >     network: {
 * >       default: {
 * >         ssid: "mynetwork"
 * >       }
 * >     }
 * >   }
 * > }
 */
exports.generate = (schema, state) => {
  return _.mapValues(schema, (entity) => {
    return State.compile(entity.properties, state);
  });
};

/**
 * @summary Extract configuration
 * @function
 * @public
 *
 * @param {Object} schema - schema
 * @param {Object} configuration - configuration
 * @returns {Object} state
 *
 * @example
 * const state = Configuration.extract({
 *   config_json: {
 *     connector: {
 *       type: 'json'
 *       path: [ 'config.json' ],
 *       partition: {
 *         primary: 4,
 *         logical: 1
 *       }
 *     },
 *     properties: {
 *       network: {
 *         ssid: {
 *           type: [ 'string' ],
 *           mapping: [
 *             [ 'network', 'default', 'ssid' ]
 *           ]
 *         }
 *       }
 *     }
 *   }
 * }, {
 *   config_json: {
 *     network: {
 *       default: {
 *         ssid: "mynetwork"
 *       }
 *     }
 *   }
 * });
 *
 * console.log(state);
 * > {
 * >   tainted: [],
 * >   result: {
 * >     network: {
 * >       ssid: 'mynetwork'
 * >     }
 * >   }
 * > }
 */
exports.extract = (schema, configuration) => {
  return _.reduce(schema, (accumulator, entity, name) => {
    try {
      return _.merge(accumulator, {
        result: State.decompile(entity.properties, _.get(configuration, name))
      });
    } catch (error) {
      accumulator.tainted.push(name);
      return accumulator;
    }
  }, {
    tainted: [],
    result: {}
  });
};
