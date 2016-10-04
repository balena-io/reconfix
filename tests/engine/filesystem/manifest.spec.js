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
  const fixturePath = path.join(__dirname, 'fixtures', name);
  const files = {
    manifest: require(path.join(fixturePath, 'files.json')),
    wet: require(path.join(fixturePath, 'wet.json')),
    schema: require(path.join(fixturePath, 'schema.json'))
  };

  ava.test(`.generateFilesManifest() (${name}) should generate files manifest`, (test) => {
    test.deepEqual(filesystem.generateFilesManifest(files.schema, files.wet), files.manifest);
  });

  ava.test(`.parseFilesManifest() (${name}) should parse files manifest`, (test) => {
    test.deepEqual(filesystem.parseFilesManifest(files.schema, files.manifest), files.wet);
  });
};

testFixture('resinos-v1');
