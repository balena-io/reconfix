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
 * @module Reconfix.Visuals.CLI
 */

const _ = require('lodash');
const inquirer = require('inquirer');

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
 * @summary Check if an object is a subset of another object
 * @function
 * @private
 *
 * @param {Object} subset - subset
 * @param {Object} object - object
 * @returns {Boolean} whether the object is a subset of the other object
 *
 * @example
 * if (isSubset({
 *   foo: 'bar'
 * }, {
 *   foo: 'bar'
 *   bar: 'baz'
 * })) {
 *   console.log('The first object is a subet of the other one');
 * }
 */
const isSubset = (subset, object) => {
  return _.every(subset, (value, key) => {
    const objectValue = _.get(object, key);

    if (_.isArray(value)) {
      return _.differenceWith(value, objectValue, _.isEqual).length === 0;
    }

    if (_.isPlainObject(value)) {
      return isSubset(value, objectValue);
    }

    return _.isEqual(value, objectValue);
  });
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
 * const question = cli.transpileQuestion({
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

  if (question.type === 'number') {
    question.validate = (input) => {
      if (!/^-?(\d+\.?\d*)$|(\d*\.?\d+)$/.test(input)) {
        return 'Invalid number';
      }

      return true;
    };

    question.filter = parseFloat;
  }

  if (question.when) {
    question.whenFunction = _.partial(isSubset, question.when);
  }

  if (question.choices) {
    question.choices = _.map(question.choices, (choice) => {
      return {
        name: choice.title,
        value: choice.name
      };
    });
  }

  return _.omitBy({
    message: question.title,
    name: question.name,
    type: type,
    default: question.default,
    choices: question.choices,
    validate: question.validate,
    filter: question.filter,
    when: question.whenFunction
  }, _.isUndefined);
};

/**
 * @summary Flatten a nested reconfix set of questions
 * @function
 * @private
 *
 * @param {Object[]} questions - questions
 * @returns {Object[]} nested questions
 *
 * @example
 * const result = cli.flatten([
 *   {
 *     title: 'Network Type',
 *     name: 'networkType',
 *     type: 'list',
 *     choices: [
 *       {
 *         title: 'Wifi',
 *         name: 'wifi',
 *         questions: [
 *           {
 *             title: 'Wifi SSID',
 *             name: 'networkSsid',
 *             type: 'text'
 *           },
 *           {
 *             title: 'Wifi Key',
 *             name: 'networkKey',
 *             type: 'password'
 *           }
 *         ]
 *       },
 *       {
 *         title: 'Ethernet',
 *         name: 'ethernet'
 *       }
 *     ]
 *   }
 * ]);
 */
exports.flatten = (questions) => {
  return _.reduce(questions, (accumulator, question) => {
    const childrenQuestions = [];

    const flattenChoices = (composedQuestion, when) => {
      when = when || {};

      composedQuestion.choices = _.map(composedQuestion.choices, (choice) => {
        if (!_.isEmpty(choice.questions)) {
          _.each(choice.questions, (choiceQuestion) => {
            choiceQuestion.when = choiceQuestion.when || {};
            _.merge(choiceQuestion.when, when);
            choiceQuestion.when[composedQuestion.name] = choice.name;
            childrenQuestions.push(choiceQuestion);

            if (choiceQuestion.type === 'list') {
              flattenChoices(choiceQuestion, choiceQuestion.when);
            }
          });
        }

        return _.omit(choice, 'questions');
      });
    };

    if (question.type === 'list') {
      flattenChoices(question);
    }

    return _.concat(accumulator, [ question ], childrenQuestions);
  }, []);
};

/**
 * @summary Run a set of questions
 * @function
 * @public
 *
 * @param {Object[]} questions - questions
 * @param {Object} [defaults={}] - default answers
 * @fulfil {Object} - answers
 * @returns {Promise}
 *
 * @example
 * cli.run([
 *   {
 *     title: 'Network Type',
 *     name: 'networkType',
 *     type: 'list',
 *     choices: [
 *       {
 *         title: 'Wifi',
 *         name: 'wifi',
 *         questions: [
 *           {
 *             title: 'Wifi SSID',
 *             name: 'networkSsid',
 *             type: 'text'
 *           },
 *           {
 *             title: 'Wifi Key',
 *             name: 'networkKey',
 *             type: 'password'
 *           }
 *         ]
 *       },
 *       {
 *         title: 'Ethernet',
 *         name: 'ethernet'
 *       }
 *     ]
 *   }
 * ]).then((answers) => {
 *   console.log(answers);
 * });
 */
exports.run = (questions, defaults) => {
  defaults = defaults || {};
  return inquirer.prompt(_.map(exports.flatten(questions), (question) => {
    question.default = _.get(defaults, question.name) || question.default;
    return exports.transpileQuestion(question);
  }));
};
