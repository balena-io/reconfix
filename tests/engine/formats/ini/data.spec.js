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
const ini = require('../../../../lib/engine/formats/ini');

const testFile = (filename) => {
  ava.test(`should parse ${filename}`, (test) => {
    const absolutePath = path.join(__dirname, 'fixtures', filename);

    return Bluebird.props({
      ini: fs.readFileAsync(`${absolutePath}.ini`, {
        encoding: 'utf8'
      }),
      json: fs.readFileAsync(`${absolutePath}.json`, {
        encoding: 'utf8'
      }).then(JSON.parse)
    }).then((contents) => {
      test.deepEqual(ini.parse(contents.ini), contents.json);
      test.deepEqual(ini.serialise(contents.json) + '\n', contents.ini.replace(/\r/g, ''));
    });
  });
};

testFile('connman');
testFile('desktop');
testFile('sample');
testFile('cellular');
