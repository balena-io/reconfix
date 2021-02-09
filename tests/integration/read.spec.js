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
const _ = require('lodash');
const path = require('path');
const reconfix = require('../../lib');

_.each([
  'resinos-v1-ethernet',
  'resinos-v1-wifi',
  'resinos-v2'
], (fixtureName) => {

  ava(`(${fixtureName}) should read settings`, (test) => {
    const fixturePath = path.join(__dirname, 'fixtures', fixtureName);
    const imagePath = path.join(fixturePath, 'image.img');
    const schema = require(path.join(fixturePath, 'schema.json'));
    const data = require(path.join(fixturePath, 'data.json'));

    return reconfix.readConfiguration(schema, imagePath).then((configuration) => {
      test.deepEqual(configuration, data);
    });
  });

});
