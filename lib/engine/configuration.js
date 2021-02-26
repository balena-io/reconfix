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
 * @module Reconfix.Engine.Configuration
 */

const _ = require('lodash');
const jsontemplate = require('../jsontemplate');

/**
 * @summary Safely merge an object to a certain path
 * @function
 * @private
 *
 * @param {Object} destination - destination object
 * @param {(String|String[])} path - object path
 * @param {Object} source - source object
 *
 * @example
 * const x = {};
 * const y = {
 *   name: 'John Doe'
 * };
 *
 * mergePath(x, [ 'data', 'foo' ], y);
 *
 * console.log(x);
 * > {
 * >   data: {
 * >     foo: {
 * >       name: 'John Doe'
 * >     }
 * >   }
 * > }
 */
const mergePath = (destination, path, source) => {
  if (!_.has(destination, path)) {
    _.set(destination, path, {});
  }

  _.merge(_.get(destination, path), source);
};

/**
 * @summary Get all unique domain filenames
 * @function
 * @private
 *
 * @param {Array[]} domain - domain
 * @returns {Array[]} unique domain filenames
 *
 * @example
 * const filenames = getDomainFilenames([
 *   [ 'foo', 'bar' ]
 *   [ 'foo', 'baz' ]
 * ]);
 *
 * console.log(filenames);
 * > [ 'foo' ]
 */
const getDomainFilenames = (domain) => {
  return _.uniqBy(_.map(domain, (domainPath) => {
    return _.initial(domainPath);
  }), _.isEqual);
};

/**
 * @summary Generate configuration
 * @function
 * @public
 *
 * @param {Object} schema - configuration schema
 * @param {Object} settings - user settings
 * @param {Object} [options] - options
 * @param {Object} [options.defaults] - default data
 * @returns {Object} configuration
 *
 * @example
 * const result = configuration.generate([
 *   {
 *     template: {
 *       gpu_mem_1024: '{{gpuMem1024}}'
 *     },
 *     domain: [
 *       [ 'config_txt', 'gpu_mem_1024' ]
 *     ]
 *   }
 * ], {
 *   gpuMem1024: 64
 * });
 *
 * console.log(result);
 * > {
 * >   config_txt: {
 * >     gpu_mem_1024: 64
 * >   }
 * > }
 */
exports.generate = (schema, settings, options) => {
  options = options || {};
  _.defaults(options, {
    defaults: {}
  });

  return _.reduce(schema, (accumulator, correspondence) => {
    const domainFilenamesPaths = getDomainFilenames(correspondence.domain);

    _.each(domainFilenamesPaths, (domainFilenamePath) => {
      if (correspondence.choice) {
        correspondence.template = _.find(correspondence.choice, {
          value: _.get(settings, correspondence.property)
        }).template;
      }

      const configuration = jsontemplate.compile(correspondence.template, settings);

      mergePath(accumulator, domainFilenamePath, _.attempt(() => {
        if (correspondence.choice) {
          return configuration;
        }

        const current = _.get(options.defaults, domainFilenamePath, {});
        return _.merge(current, configuration);
      }));
    });

    return accumulator;
  }, {});
};

/**
 * @summary Extract configuration
 * @function
 * @public
 *
 * @param {Object} schema - configuration schema
 * @param {Object} configuration - configuration object
 * @returns {Object} user settings
 *
 * @example
 * const settings = configuration.extract([
 *   {
 *     template: {
 *       gpu_mem_1024: '{{gpuMem1024}}'
 *     },
 *     domain: [
 *       [ 'config_txt', 'gpu_mem_1024' ]
 *     ]
 *   }
 * ], {
 *   config_txt: {
 *     gpu_mem_1024: 64
 *   }
 * });
 *
 * console.log(settings);
 * > {
 * >   gpuMem1024: 64
 * > }
 */
exports.extract = (schema, configuration) => {
  return _.reduce(schema, (accumulator, correspondence) => {
    const domainFilenamesPaths = getDomainFilenames(correspondence.domain);

    _.each(domainFilenamesPaths, (domainFilenamePath) => {
      const domain = _.get(configuration, domainFilenamePath);

      if (correspondence.choice) {
        const matches = _.filter(correspondence.choice, (choice) => {
          return jsontemplate.matches(choice.template, domain);
        });

        if (matches.length !== 1) {
          throw new Error([
            'The current state doesn\'t match the schema.',
            '',
            'Current configuration:',
            '',
            JSON.stringify(domain, null, 2),
            '',
            'Schema choices:',
            '',
            JSON.stringify(correspondence.choice, null, 2)
          ].join('\n'));
        }

        const match = _.first(matches);

        _.set(accumulator, correspondence.property, match.value);
        correspondence.template = match.template;
      }

      _.merge(accumulator, jsontemplate.decompile(correspondence.template, domain));
    });

    return accumulator;
  }, {});
};
