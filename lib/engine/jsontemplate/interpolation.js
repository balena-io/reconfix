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
const EXPRESSION_REGEX = /\[([\w$_\.\[\]]+)\]/g;

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

exports.interpolateString = (string, data) => {
  ensureDataIsValid(data);

  return _.template(string, {
    interpolate: EXPRESSION_REGEX
  })(data);
};

exports.deinterpolateString = (template, string) => {
  const templateRegexString = template.replace(EXPRESSION_REGEX, '(.+)');
  const templateRegex = new RegExp(templateRegexString);
  const allExpressions = template.match(EXPRESSION_REGEX);
  const allValues = _.tail(templateRegex.exec(string));

  return _.reduce(_.zip(allExpressions, allValues), (data, pair) => {

    // Reset global RegExp index
    // See: http://stackoverflow.com/a/11477448/1641422
    EXPRESSION_REGEX.lastIndex = 0;

    const key = _.nth(EXPRESSION_REGEX.exec(_.first(pair)), 1);
    _.set(data, key, _.last(pair));
    return data;
  }, {});
};
