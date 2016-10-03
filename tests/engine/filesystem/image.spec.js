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
const Bluebird = require('bluebird');
const fs = require('fs');
const rindle = require('rindle');
const tmp = Bluebird.promisifyAll(require('tmp'));
const filesystem = require('../../../lib/engine/filesystem');

const testFixture = (name) => {
  const fixturePath = path.join(__dirname, 'fixtures', 'images', name);
  const files = {
    image: path.join(fixturePath, 'image.img'),
    schema: require(path.join(fixturePath, 'schema.json')),
    wet: require(path.join(fixturePath, 'wet.json')),
    data: require(path.join(fixturePath, 'data.json'))
  };

  ava.test(`.readImageData() (${name}) should read files`, (test) => {
    return filesystem.readImageData(files.schema, files.image).then((data) => {
      test.deepEqual(data, files.data);
    });
  });

  ava.test(`.readImageConfiguration() (${name}) should read configuration`, (test) => {
    return filesystem.readImageConfiguration(files.schema, files.image).then((data) => {
      test.deepEqual(data, files.wet);
    });
  });
};

testFixture('resinos-v1');

const createTemporaryFileFromFile = (file) => {
  return tmp.fileAsync().tap((temporaryFilePath) => {
    const stream = fs.createReadStream(file)
      .pipe(fs.createWriteStream(temporaryFilePath));
    return rindle.wait(stream);
  });
};

/* eslint-disable camelcase */

ava.test('.writeImageData() should write a files manifest to an empty image', (test) => {
  const imagePath = path.join(__dirname, 'fixtures', 'images', 'empty', 'image.img');

  const manifest = {
    config_txt: {
      location: {
        path: 'config.txt',
        partition: {
          primary: 1
        }
      },
      data: 'gpu_mem_1024=64'
    },
    config_json: {
      location: {
        path: 'config.json',
        partition: {
          primary: 4,
          logical: 1
        }
      },
      data: '{\n  "foo":"bar"\n}'
    }
  };

  const schema = {
    config_txt: {
      type: 'ini',
      location: {
        path: 'config.txt',
        partition: {
          primary: 1
        }
      }
    },
    network_config: {
      type: 'ini',
      location: {
        parent: 'config_json',
        property: [ 'files', 'network/network.config' ]
      }
    },
    config_json: {
      type: 'json',
      location: {
        path: 'config.json',
        partition: {
          primary: 4,
          logical: 1
        }
      }
    }
  };

  return createTemporaryFileFromFile(imagePath).then((temporaryFilePath) => {
    return filesystem.writeImageData(manifest, temporaryFilePath).then(() => {
      return filesystem.readImageData(schema, temporaryFilePath).then((data) => {
        test.deepEqual(data, manifest);
      });
    });
  });
});

/* eslint-enable camelcase */
