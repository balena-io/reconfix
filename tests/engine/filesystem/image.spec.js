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
const path = require('path');

const utils = require('../../../lib/test-utils');
const filesystem = require('../../../lib/engine/filesystem');

const testReadFixture = (name) => {
  const fixturePath = path.join(__dirname, 'fixtures', 'images', name);
  const files = {
    image: path.join(fixturePath, 'image.img'),
    schema: require(path.join(fixturePath, 'schema.json')),
    wet: require(path.join(fixturePath, 'wet.json')),
    data: require(path.join(fixturePath, 'data.json'))
  };

  ava.test(`.readImageData() (${name}) should read files`, (test) => {
    return utils.testBoth(files.image, (imagePath) => {
      return filesystem.readImageData(files.schema, imagePath)
      .then((data) => {
        test.deepEqual(data, files.data);
      });
    });
  });

  ava.test(`.readImageConfiguration() (${name}) should read configuration`, (test) => {
    return utils.testBoth(files.image, (imagePath) => {
      return filesystem.readImageConfiguration(files.schema, imagePath)
      .then((data) => {
        test.deepEqual(data, files.wet);
      });
    });
  });
};

testReadFixture('resinos-v1');
testReadFixture('resinos-v2');

const testWriteFixture = (name, partition) => {
  const fixturePath = path.join(__dirname, 'fixtures', 'images', name);
  const files = {
    image: path.join(fixturePath, 'image.img'),
    schema: require(path.join(fixturePath, 'schema.json')),
    wet: require(path.join(fixturePath, 'wet.json'))
  };
  if (partition !== undefined) {
    files.schema.config_txt.location.partition = partition;
    files.schema.config_txt.location.path = '/' + files.schema.config_txt.location.path;
  }

  ava.test(`(${name}) should write/read settings`, (test) => {
    return utils.testBoth(files.image, (temporaryFilePath) => {
      return filesystem.writeImageConfiguration(files.schema, temporaryFilePath, files.wet)
      .then(() => {
        return filesystem.readImageConfiguration(files.schema, temporaryFilePath);
      })
      .then((data) => {
        test.deepEqual(data, files.wet);
      });
    });
  });
};

testWriteFixture('resinos-v1-empty');

// Test writing on a ext2 partition.
testWriteFixture('resinos-v1-empty', 6);
