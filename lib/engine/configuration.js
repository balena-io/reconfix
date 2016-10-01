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
 * @module Reconfix.Engine.Configuration
 */

const _ = require('lodash');
const jsontemplate = require('../jsontemplate');

const mergePath = (destination, path, source) => {
  if (!_.has(destination, path)) {
    _.set(destination, path, {});
  }

  _.merge(_.get(destination, path), source);
};

const getDomainFilenames = (domain) => {
  return _.uniqBy(_.map(domain, (domainPath) => {
    return _.initial(domainPath);
  }), _.isEqual);
};

exports.generate = (schema, settings) => {
  return _.reduce(schema, (accumulator, correspondence) => {
    const domainFilenamesPaths = getDomainFilenames(correspondence.domain);

    _.each(domainFilenamesPaths, (domainFilenamePath) => {
      if (correspondence.choice) {
        correspondence.template = _.find(correspondence.choice, {
          value: _.get(settings, correspondence.property)
        }).template;
      }

      const configuration = jsontemplate.compile(correspondence.template, settings);
      mergePath(accumulator, domainFilenamePath, configuration);
    });

    return accumulator;
  }, {});
};

exports.extract = (schema, configuration) => {
  return _.reduce(schema, (accumulator, correspondence) => {
    const domainFilenamesPaths = getDomainFilenames(correspondence.domain);

    _.each(domainFilenamesPaths, (domainFilenamePath) => {
      const domain = _.get(configuration, domainFilenamePath);

      if (correspondence.choice) {
        const matches = _.filter(correspondence.choice, (choice) => {
          return jsontemplate.matches(choice.template, domain);
        });

        if (matches.length > 1) {
          throw new Error('Ambiguity!');
        }

        if (matches.length === 0) {
          throw new Error('No match, custom editor!');
        }

        const match = _.first(matches);

        _.set(accumulator, correspondence.property, match.value);
        correspondence.template = match.template;
      }

      _.merge(accumulator, jsontemplate.decompile(correspondence.template, domain));
    });

    return accumulator;
  }, {});
};
