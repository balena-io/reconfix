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
const formats = require('./formats');

exports.isSchemaFileVirtual = (fileDeclaration) => {
  return _.has(fileDeclaration, 'location.parent');
};

exports.generateFilesManifest = (schema, data) => {
  const rootFiles = _.chain(data)
    .pick(_.keys(_.omitBy(schema, exports.isSchemaFileVirtual)))
    .mapValues((fileData, fileId) => {
      return _.set(_.get(schema, fileId), 'data', fileData);
    })
    .value();

  return _.chain(schema)
    .pickBy(exports.isSchemaFileVirtual)
    .reduce((accumulator, fileDeclaration, fileId) => {
      const fileContents = _.get(data, fileId);
      const finalPath = _.concat([ fileDeclaration.location.parent, 'data' ], fileDeclaration.location.property);
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

// See: http://stackoverflow.com/a/38278831/1641422
const removeEmptyObjects = (object) => {
  return _.chain(object)
    .pickBy(_.isObject)
    .mapValues(removeEmptyObjects)
    .omitBy(_.isEmpty)
    .assign(_.omitBy(object, _.isObject))
    .value();
};

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
