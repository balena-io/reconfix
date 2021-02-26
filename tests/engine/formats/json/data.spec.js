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
const fs = Bluebird.promisifyAll(require('fs'));
const json = require('../../../../lib/engine/formats/json');

const testFile = (filename) => {
  ava(`should parse ${filename}`, (test) => {
    const absolutePath = path.join(__dirname, 'fixtures', filename);

    return fs.readFileAsync(`${absolutePath}.json`, {
      encoding: 'utf8'
    }).then((contents) => {
      const parsedJSON = json.parse(contents);
      const serialisedJSON = json.serialise(parsedJSON);
      test.deepEqual(serialisedJSON + '\n', contents.replace(/\r/g, ''));
    });
  });
};

testFile('config');
