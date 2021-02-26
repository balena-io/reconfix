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

const parseLines = (lines) => {
  return _.join(lines, '\n');
};

exports.testParse = (module, title, lines, object) => {
  ava(`should parse ${title}`, (test) => {
    test.deepEqual(module.parse(parseLines(lines)), object);
  });
};

exports.testSerialize = (module, title, lines, object) => {
  ava(`should serialise ${title}`, (test) => {
    test.deepEqual(module.serialise(object), parseLines(lines));
  });
};

exports.testBidirectional = (module, title, lines, object) => {
  exports.testParse(module, title, lines, object);
  exports.testSerialize(module, title, lines, object);
};
