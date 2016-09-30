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
const string = require('../../../lib/jsontemplate/string');

ava.test('.deinterpolate() should throw if strings do not match', (test) => {
  test.throws(() => {
    string.deinterpolate('Hello {{name}}!', 'Hi John Doe!');
  }, 'No match for \'name\'');
});

ava.test('.deinterpolate() should throw if interpolation result is missing', (test) => {
  test.throws(() => {
    string.deinterpolate('Hello {{name}}!', 'Hi !');
  }, 'No match for \'name\'');
});
