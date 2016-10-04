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
 * @module Reconfix.Engine.Filesystem
 */

const _ = require('lodash');
const Bluebird = require('bluebird');
const imagefs = require('resin-image-fs');
const formats = require('./formats');

/**
 * @summary Check if a file declaration represents a virtual file
 * @function
 * @private
 *
 * @param {Object} fileDeclaration - file declaration
 * @returns {Boolean} whether the file declaration represents a virtual file
 *
 * @example
 * if (filesystem.isSchemaFileVirtual({
 *   network_config: {
 *     type: 'ini',
 *     location: {
 *       parent: 'config_json',
 *       property: [ 'files', 'network/network.config' ]
 *     }
 *   },
 * })) {
 *   console.log('This file is virtual!');
 * }
 */
exports.isSchemaFileVirtual = (fileDeclaration) => {
  return _.has(fileDeclaration, 'location.parent');
};

/**
 * @summary Generate a files manifest
 * @function
 * @private
 *
 * @param {Object} schema - schema
 * @param {Object} data - file data
 * @returns {Object} manifest
 *
 * @example
 * const manifest = filesystem.generateFilesManifest({
 *   file1: { ... },
 *   file2: { ... },
 *   file3: { ... }
 * }, {
 *   file1: { ... },
 *   file2: { ... },
 *   file3: { ... }
 * });
 */
exports.generateFilesManifest = (schema, data) => {
  const rootFiles = _.chain(_.cloneDeep(data))
    .pick(_.keys(_.omitBy(schema, exports.isSchemaFileVirtual)))
    .mapValues((fileData, fileId) => {
      return _.set(_.get(schema, fileId), 'data', fileData);
    })
    .value();

  return _.chain(schema)
    .pickBy(exports.isSchemaFileVirtual)
    .reduce((accumulator, fileDeclaration, fileId) => {
      const fileContents = _.get(data, fileId);
      const finalPath = _.concat([
        fileDeclaration.location.parent,
        'data'
      ], fileDeclaration.location.property);
      return _.set(accumulator, finalPath, formats.serialise(fileDeclaration.type, fileContents));
    }, rootFiles)
    .mapValues((value) => {
      return {
        data: formats.serialise(value.type, value.data),
        location: value.location
      };
    })
    .value();
};

/**
 * @summary Remove empty object properties from object
 * @function
 * @private
 *
 * @description
 * From http://stackoverflow.com/a/38278831/1641422.
 *
 * @param {Object} object - object
 * @returns {Object} object without properties containing empty objects
 *
 * @example
 * const result = removeEmptyObjects({
 *   foo: {}
 * });
 *
 * console.log(result.foo);
 * > undefined
 */
const removeEmptyObjects = (object) => {
  return _.chain(object)
    .pickBy(_.isPlainObject)
    .mapValues(removeEmptyObjects)
    .omitBy(_.isEmpty)
    .assign(_.omitBy(object, _.isPlainObject))
    .value();
};

/**
 * @summary Parse files manifest
 * @function
 * @private
 *
 * @param {Object} schema - schema
 * @param {Object} manifest - file manifest
 * @returns {Object} file data
 *
 * @example
 * const data = filesystem.parseFilesManifest({
 *   file1: { ... },
 *   file2: { ... },
 *   file3: { ... }
 * }, {
 *   file1: { ... },
 *   file2: { ... },
 *   file3: { ... }
 * });
 */
exports.parseFilesManifest = (schema, manifest) => {
  const data = _.chain(schema)
    .omitBy(exports.isSchemaFileVirtual)
    .mapValues((fileDeclaration, fileId) => {
      const fileData = _.get(manifest, [ fileId, 'data' ]);
      return formats.parse(fileDeclaration.type, fileData);
    })
    .value();

  _.chain(schema)
    .pickBy(exports.isSchemaFileVirtual)
    .each((fileDeclaration, fileId) => {
      const propertyPath = _.concat([
        fileDeclaration.location.parent
      ], fileDeclaration.location.property);

      const fileData = _.get(data, propertyPath);
      _.set(data, fileId, formats.parse(fileDeclaration.type, fileData));
      _.unset(data, propertyPath);
    })
    .value();

  return removeEmptyObjects(data);
};

/**
 * @summary Read image data
 * @function
 * @private
 *
 * @param {Object} schema - schema
 * @param {String} image - path to image
 * @fulfil {Object} - image data
 * @returns {Promise}
 *
 * @example
 * filesystem.readImageData({
 *   file1: { ... },
 *   file2: { ... },
 *   file3: { ... }
 * }, 'path/to/image.img').then((data) => {
 *   console.log(data);
 * });
 */
exports.readImageData = (schema, image) => {
  const realFiles = _.omitBy(schema, exports.isSchemaFileVirtual);

  return Bluebird.map(_.toPairs(realFiles), (filePair) => {
    const fileId = _.first(filePair);
    const fileDeclaration = _.last(filePair);

    return imagefs.readFile({
      image: image,
      partition: fileDeclaration.location.partition,
      path: fileDeclaration.location.path
    })
    .then((data) => {
      return [
        fileId,
        {
          location: fileDeclaration.location,
          data: data
        }
      ];
    });
  }).then(_.fromPairs);
};

/**
 * @summary Write image data
 * @function
 * @private
 *
 * @param {Object} manifest - file manifest
 * @param {String} image - path to image
 * @returns {Promise}
 *
 * @example
 * filesystem.writeImageData({
 *   file1: { ... },
 *   file2: { ... },
 *   file3: { ... }
 * }, 'path/to/image.img').then(() => {
 *   console.log('Done!');
 * });
 */
exports.writeImageData = (manifest, image) => {
  return Bluebird.each(_.values(manifest), (fileDeclaration) => {
    return imagefs.writeFile({
      image: image,
      partition: fileDeclaration.location.partition,
      path: fileDeclaration.location.path
    }, fileDeclaration.data);
  });
};

/**
 * @summary Read image configuration
 * @function
 * @public
 *
 * @param {Object} schema - schema
 * @param {String} image - path to image
 * @fulfil {Object} - image configuration
 * @returns {Promise}
 *
 * @example
 * filesystem.readImageConfiguration({
 *   config_txt: {
 *     type: 'ini',
 *     location: {
 *       path: 'config.txt',
 *       partition: {
 *         primary: 1
 *       }
 *     }
 *   }
 * }, 'path/to/image.img').then((configuration) => {
 *   console.log(configuration.config_txt);
 * });
 */
exports.readImageConfiguration = (schema, image) => {
  return exports.readImageData(schema, image).then((imageFileDeclarations) => {
    return exports.parseFilesManifest(schema, imageFileDeclarations);
  });
};

/**
 * @summary Write image configuration
 * @function
 * @public
 *
 * @param {Object} schema - schema
 * @param {String} image - path to image
 * @param {Object} settings - settings
 * @returns {Promise}
 *
 * @example
 * filesystem.writeImageConfiguration({
 *   config_txt: {
 *     type: 'ini',
 *     location: {
 *       path: 'config.txt',
 *       partition: {
 *         primary: 1
 *       }
 *     }
 *   }
 * }, 'path/to/image.img', {
 *   config_txt: {
 *     foo: 'bar'
 *   }
 * }).then(() => {
 *   console.log('Done!');
 * });
 */
exports.writeImageConfiguration = (schema, image, settings) => {
  return Bluebird.try(() => {
    return exports.readImageConfiguration(schema, image).catch((error) => {
      if (error.code === 'NOENT') {
        return {};
      }

      throw error;
    });
  }).then((configuration) => {
    _.merge(configuration, settings);
    const manifest = exports.generateFilesManifest(schema, configuration);
    return exports.writeImageData(manifest, image);
  });
};
