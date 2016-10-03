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
const configuration = require('../../../lib/engine/configuration');

const testFixture = (name) => {
  const fixturePath = path.join(__dirname, 'fixtures', name);
  const files = {
    dry: require(path.join(fixturePath, 'dry.json')),
    wet: require(path.join(fixturePath, 'wet.json')),
    wetExtra: require(path.join(fixturePath, 'wet-extra.json')),
    schema: require(path.join(fixturePath, 'schema.json'))
  };

  ava.test(`(${name}) should generate configuration`, (test) => {
    test.deepEqual(configuration.generate(files.schema, files.dry), files.wet);
  });

  ava.test(`(${name}) should extract settings`, (test) => {
    test.deepEqual(configuration.extract(files.schema, files.wet), files.dry);
  });

  ava.test(`(${name}) should ignore extra settings when extracting`, (test) => {
    test.deepEqual(configuration.extract(files.schema, files.wetExtra), files.dry);
  });
};

testFixture('resinos-v1');
