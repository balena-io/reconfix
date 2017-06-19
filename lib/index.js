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
const filedisk = require('file-disk');
const Promise = require('bluebird');

const filesystem = require('./engine/filesystem');
const configuration = require('./engine/configuration');

const checkStringOrDisk = (image) => {
  if (!_.isString(image) && !(image instanceof filedisk.Disk)) {
    throw new Error('image must be a String or a filedisk.Disk instance');
  }
};

const readConfiguration = (schema, image) => {
  return filesystem.readImageConfiguration(schema.files, image)
  .then(_.partial(configuration.extract, schema.mapper));
};

/**
 * @summary Read image configuration
 * @function
 * @public
 *
 * @param {Object} schema - schema
 * @param {String|filedisk.Disk} image - path to image or filedisk.Disk instance
 * @fulfil {Object} - configuration
 * @returns {Promise}
 *
 * @example
 * reconfix.readConfiguration({ ... }, 'path/to/image.img').then((configuration) => {
 *   console.log(configuration);
 * });
 */
exports.readConfiguration = (schema, image) => {
  checkStringOrDisk(image);
  if (_.isString(image)) {
    return Promise.using(filedisk.openFile(image, 'r'), (fd) => {
      const disk = new filedisk.FileDisk(fd);
      return readConfiguration(schema, disk);
    });
  }
  return readConfiguration(schema, image);
};

const writeConfiguration = (schema, object, image) => {
  return filesystem.readImageConfiguration(schema.files, image)
  .then((current) => {
    const data = configuration.generate(schema.mapper, object, {
      defaults: current
    });
    return filesystem.writeImageConfiguration(schema.files, image, data);
  });
};

/**
 * @summary Write image configuration
 * @function
 * @public
 *
 * @param {Object} schema - schema
 * @param {Object} object - configuration object
 * @param {String|filedisk.Disk} image - path to image or filedisk.Disk instance
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
  checkStringOrDisk(image);
  if (_.isString(image)) {
    return Promise.using(filedisk.openFile(image, 'r+'), (fd) => {
      const disk = new filedisk.FileDisk(fd);
      return writeConfiguration(schema, object, disk);
    });
  }
  return writeConfiguration(schema, object, image);
};
