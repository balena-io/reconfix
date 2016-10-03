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
const filesystem = require('../../../lib/engine/filesystem');

const testFixture = (name) => {
  const fixturePath = path.join(__dirname, 'fixtures', 'images', name);
  const files = {
    image: path.join(fixturePath, 'image.img'),
    schema: require(path.join(fixturePath, 'schema.json')),
    data: require(path.join(fixturePath, 'data.json'))
  };

  ava.test(`.readImageConfiguration() (${name}) should read files`, (test) => {
    return filesystem.readImageConfiguration(files.schema, files.image).then((data) => {
      test.deepEqual(data, files.data);
    });
  });
};

testFixture('resinos-v1');
