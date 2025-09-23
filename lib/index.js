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
 * @module Reconfix
 */

const _ = require('lodash');
const filesystem = require('./engine/filesystem');
const configuration = require('./engine/configuration');

/**
 * @summary Read image configuration
 * @function
 * @public
 *
 * @param {Object} schema - schema
 * @param {String} image - path to image
 * @fulfil {Object} - configuration
 * @returns {Promise}
 *
 * @example
 * reconfix.readConfiguration({ ... }, 'path/to/image.img').then((configuration) => {
 *   console.log(configuration);
 * });
 */
exports.readConfiguration = (schema, image) => {
  return filesystem.readImageConfiguration(schema.files, image)
    .then(_.partial(configuration.extract, schema.mapper));
};

/**
 * @summary Write image configuration
 * @function
 * @public
 *
 * @param {Object} schema - schema
 * @param {Object} object - configuration object
 * @param {String} image - path to image
 * @returns {Promise}
 *
 * @example
 * reconfix.writeConfiguration({ ... }, {
 *   setting1: 'value',
 *   setting2: 'value',
 *   setting3: 'value'
 * }, 'path/to/image.img').then(() => {
 *   console.log('Done!');
 * });
 */
exports.writeConfiguration = (schema, object, image) => {
  return filesystem.readImageConfiguration(schema.files, image).then((current) => {
    const data = configuration.generate(schema.mapper, object, {
      defaults: current
    });

    return filesystem.writeImageConfiguration(schema.files, image, data);
  });
};
