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

const _ = require('lodash');
const Bluebird = require('bluebird');
const chai = require('chai');
const sinon = require('sinon');
const Connector = require('../lib/connector');

describe('Connector', function() {

  describe('.BUILTIN_CONNECTORS', function() {

    it('should not be empty', function() {
      chai.expect(_.isEmpty(Connector.BUILTIN_CONNECTORS)).to.be.false;
    });

    it('should be a plain object', function() {
      chai.expect(_.isPlainObject(Connector.BUILTIN_CONNECTORS)).to.be.true;
    });

    it('should contain objects with an "set" function', function() {
      _.each(Connector.BUILTIN_CONNECTORS, (connector) => {
        chai.expect(_.isFunction(connector.set)).to.be.true;
      });
    });

  });

  describe('.getType()', function() {

    it('should return the type of the connector', function() {
      chai.expect(Connector.getType({
        type: 'json',
        path: [ 'config.txt' ],
        partition: {
          primary: 1
        }
      })).to.equal('json');
    });

  });

  describe('.getOptions()', function() {

    it('should return the connector options', function() {
      chai.expect(Connector.getOptions({
        type: 'json',
        path: [ 'config.txt' ],
        partition: {
          primary: 1
        }
      })).to.deep.equal({
        path: [ 'config.txt' ],
        partition: {
          primary: 1
        }
      });
    });

    it('should return an empty object if the connect has no options', function() {
      chai.expect(Connector.getOptions({
        type: 'null'
      })).to.deep.equal({});
    });

  });

  describe('.set()', function() {

    it('should throw if the connector type does not exist', function() {
      chai.expect(() => {
        Connector.set({
          type: 'foobar',
          url: 'foobar.com'
        }, {
          foo: {
            bar: 'baz'
          }
        }, {
          connectors: {}
        });
      }).to.throw('Unknown connector type: "foobar"');
    });

    it('should throw if the connector type does not contain an set function', function() {
      chai.expect(() => {
        Connector.set({
          type: 'foobar',
          url: 'foobar.com'
        }, {
          foo: {
            bar: 'baz'
          }
        }, {
          connectors: {
            foobar: {
              set: 1
            }
          }
        });
      }).to.throw('Invalid connector type: "foobar", "set" is not a function');
    });

    it('should pass the options and the data to the connector', function(done) {
      const fakeExecutor = sinon.stub();
      fakeExecutor.returns(Bluebird.resolve());

      Connector.set({
        type: 'stub',
        option1: 'value1',
        option2: 'value2'
      }, {
        foo: {
          bar: {
            baz: 1
          }
        }
      }, {
        connectors: {
          stub: {
            set: fakeExecutor
          }
        }
      }).finally(() => {
        chai.expect(fakeExecutor.callCount).to.equal(1);
        chai.expect(fakeExecutor.firstCall.args).to.deep.equal([
          {
            option1: 'value1',
            option2: 'value2'
          },
          {
            foo: {
              bar: {
                baz: 1
              }
            }
          }
        ]);
        done();
      });
    });

    it('should be rejected if the executor rejects', function(done) {
      const fakeExecutor = sinon.stub();
      fakeExecutor.returns(Bluebird.reject(new Error('Executor error')));

      Connector.set({
        type: 'stub',
        option1: 'value1',
        option2: 'value2'
      }, {
        foo: {
          bar: {
            baz: 1
          }
        }
      }, {
        connectors: {
          stub: {
            set: fakeExecutor
          }
        }
      }).catch((error) => {
        chai.expect(error).to.be.an.instanceof(Error);
        chai.expect(error.message).to.equal('Executor error');
        done();
      });
    });

  });

});
