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

const chai = require('chai');
const state = require('../lib/state');

describe('State', function() {

  describe('bidirectional property', function() {

    const bidirectional = (title, property, settings, object) => {
      it(`.compile() ${title}`, function() {
        chai.expect(state.compile(property, settings)).to.deep.equal(object);
      });

      it(`.decompile() ${title}`, function() {
        chai.expect(state.decompile(property, object)).to.deep.equal(settings);
      });
    };

    bidirectional('should compile a single simple mapping', {
      bar: {
        type: [ 'number' ],
        mapping: [
          [ 'baz' ]
        ]
      }
    }, {
      bar: 3
    }, {
      baz: 3
    });

    bidirectional('should compile a single complex mapping', {
      property: {
        type: [ 'number' ],
        mapping: [
          [ 'foo', 'bar' ],
          [ 'baz' ]
        ]
      }
    }, {
      property: 13
    }, {
      foo: {
        bar: 13
      },
      baz: 13
    });

    bidirectional('should compile multiple simple mappings', {
      foo: {
        type: [ 'number' ],
        mapping: [
          [ 'xxx' ]
        ]
      },
      bar: {
        type: [ 'string' ],
        mapping: [
          [ 'yyy' ]
        ]
      },
      baz: {
        type: [ 'string' ],
        mapping: [
          [ 'zzz' ]
        ]
      }
    }, {
      foo: 7,
      bar: 'hello',
      baz: 'world'
    }, {
      xxx: 7,
      yyy: 'hello',
      zzz: 'world'
    });

  });

  describe('.compile()', function() {

    it('should throw if there is a type mismatch', function() {
      chai.expect(() => {
        state.compile({
          bar: {
            type: [ 'number' ],
            mapping: [
              [ 'baz' ]
            ]
          }
        }, {
          bar: 'foo'
        });
      }).to.throw('Type mismatch for "bar": expected number, but got "foo"');
    });

  });

  describe('.decompile()', function() {

    it('should throw if there is a type mismatch', function() {
      chai.expect(() => {
        state.decompile({
          bar: {
            type: [ 'number' ],
            mapping: [
              [ 'baz' ]
            ]
          }
        }, {
          baz: 'foo'
        });
      }).to.throw('Type mismatch for "bar": expected number, but got "foo"');
    });

  });

});
