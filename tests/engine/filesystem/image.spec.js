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

const ava = require('ava');
const Bluebird = require('bluebird');
const fs = require('fs');
const rindle = require('rindle');
const path = require('path');
const tmp = Bluebird.promisifyAll(require('tmp'));
const filesystem = require('../../../lib/engine/filesystem');

const testReadFixture = (name) => {
  const fixturePath = path.join(__dirname, 'fixtures', 'images', name);
  const files = {
    image: path.join(fixturePath, 'image.img'),
    schema: require(path.join(fixturePath, 'schema.json')),
    wet: require(path.join(fixturePath, 'wet.json')),
    data: require(path.join(fixturePath, 'data.json'))
  };

  ava(`.readImageData() (${name}) should read files`, (test) => {
    return filesystem.readImageData(files.schema, files.image).then((data) => {
      test.deepEqual(data, files.data);
    });
  });

  ava(`.readImageConfiguration() (${name}) should read configuration`, (test) => {
    return filesystem.readImageConfiguration(files.schema, files.image).then((data) => {
      test.deepEqual(data, files.wet);
    });
  });
};

testReadFixture('resinos-v1');
testReadFixture('resinos-v2');

const createTemporaryFileFromFile = (file) => {
  return tmp.fileAsync().tap((temporaryFilePath) => {
    const stream = fs.createReadStream(file)
      .pipe(fs.createWriteStream(temporaryFilePath));
    return rindle.wait(stream);
  });
};

const testWriteFixture = (name) => {
  const fixturePath = path.join(__dirname, 'fixtures', 'images', name);
  const files = {
    image: path.join(fixturePath, 'image.img'),
    schema: require(path.join(fixturePath, 'schema.json')),
    wet: require(path.join(fixturePath, 'wet.json'))
  };

  ava(`(${name}) should write/read settings`, (test) => {
    return createTemporaryFileFromFile(files.image).then((temporaryFilePath) => {
      return filesystem.writeImageConfiguration(files.schema, temporaryFilePath, files.wet).then(() => {
        return filesystem.readImageConfiguration(files.schema, temporaryFilePath).then((data) => {
          test.deepEqual(data, files.wet);
        });
      });
    });
  });
};

testWriteFixture('resinos-v1-empty');
