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
 * @module Reconfix.Visuals.Cli.Integrations.Inquirer
 */

const _ = require('lodash');

/**
 * @summary Reconfix to Inquirer input type map
 * @type Object
 * @constant
 * @private
 */
const INQUIRER_TYPE_MAP = {
  text: 'input',
  password: 'password',
  editor: 'editor',
  list: 'list',
  checkbox: 'checkbox',

  // InquirerJS doesn't support a "number"
  // input so we fallback to a text field.
  number: 'input'

};

/**
 * @summary Transpile a Reconfix question to an InquirerJS question
 * @function
 * @private
 *
 * @param {Object} question - question
 * @returns {Object} inquirer question
 *
 * @example
 * const question = inquirer.transpileQuestion({
 *   title: 'Wifi SSID',
 *   name: 'ssid',
 *   type: 'text',
 *   when: {
 *     networkType: 'wifi'
 *   }
 * });
 */
exports.transpileQuestion = (question) => {
  if (!question.title || !_.isString(question.title)) {
    throw new Error(`Invalid question title: ${question.title}`);
  }

  if (!question.name || !_.isString(question.name)) {
    throw new Error(`Invalid question name: ${question.name}`);
  }

  const type = _.get(INQUIRER_TYPE_MAP, question.type);

  if (!type) {
    throw new Error(`Unknown question type: ${question.type}`);
  }

  if (type === 'input' && question.default && _.isNumber(question.default)) {
    question.default = String(question.default);
  }

  if (question.when) {
    question.whenFunction = (answers) => {
      return _.every(question.when, (value, key) => {
        const answer = _.get(answers, key);

        if (_.isArray(value)) {
          return _.differenceWith(value, answer, _.isEqual).length === 0;
        }

        return _.isEqual(value, answer);
      });
    };
  }

  return _.omitBy({
    message: question.title,
    name: question.name,
    type: type,
    default: question.default,
    choices: question.choices,
    when: question.whenFunction
  }, _.isUndefined);
};
