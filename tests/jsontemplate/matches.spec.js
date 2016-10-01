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
const _ = require('lodash');
const jsontemplate = require('../../lib/jsontemplate');
const fixturesPath = path.join(__dirname, 'fixtures', 'matches');

const testFixture = (name) => {
  const fixture = require(path.join(fixturesPath, `${name}.json`));

  _.each(fixture.choices, (choice) => {
    ava.test(`.matches() (${name}) should be ${choice.matches} for ${choice.title}`, (test) => {
      test.is(jsontemplate.matches(choice.template, fixture.object), choice.matches);
    });
  });
};

testFixture('resinos-v1');
