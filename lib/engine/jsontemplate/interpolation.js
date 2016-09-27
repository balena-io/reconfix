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

/**
 * @module Reconfix.Engine.JSONTemplate.Interpolation
 */

const _ = require('lodash');

const ensureDataIsValid = (data) => {
  _.each(data, (value) => {
    if (_.isPlainObject(value)) {
      return ensureDataIsValid(value);
    }

    if (!_.isString(value)) {
      throw new Error(`Invalid data value: ${value}`);
    }
  });
};

// See http://stackoverflow.com/a/3410547/1641422
exports.findAllIndexes = (string, substring) => {
  if (_.isEmpty(substring)) {
    return [];
  }

  return _.reduce(string, (indexes, character, index) => {
    if (string.substring(index, index + substring.length) === substring) {
      indexes.push(index);
    }

    return indexes;
  }, []);
};

exports.splitTemplateTokens = (template) => {
  return _.reduce(template, (accumulator, character, index) => {
    if (accumulator.isExpression) {
      accumulator.words[accumulator.words.length - 1] += character;

      if (character === ']') {
        if (index !== template.length - 1) {
          accumulator.words.push('');
        }

        accumulator.isExpression = false;
      }

      return accumulator;
    }

    if (character !== '[') {
      if (_.isUndefined(_.last(accumulator.words))) {
        accumulator.words.push('');
      }

      accumulator.words[accumulator.words.length - 1] += character;
      return accumulator;
    }

    accumulator.words.push(character);
    accumulator.isExpression = true;
    return accumulator;
  }, {
    words: [],
    isExpression: false
  }).words;
};

exports.interpolateString = (string, data) => {
  ensureDataIsValid(data);

  return _.template(string, {
    interpolate: /\[([\w$_\.]+)\]/g
  })(data);
};

// exports.deinterpolateString = (string, template) => {

// };
