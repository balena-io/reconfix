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
const Domain = require('../lib/domain');

describe('Domain', function() {

  describe('.mask()', function() {

    it('should return an empty object if the path list is empty', function() {
      chai.expect(Domain.mask({
        foo: {
          bar: 'baz'
        }
      }, [])).to.deep.equal({});
    });

    it('should ignore invalid paths', function() {
      chai.expect(Domain.mask({
        foo: 1,
        bar: 2,
        baz: 3
      }, [
        [ 'bar' ],
        [ 'qux' ]
      ])).to.deep.equal({
        bar: 2
      });
    });

    it('should mask an object with a single path', function() {
      chai.expect(Domain.mask({
        foo: 1,
        bar: 2,
        baz: 3
      }, [
        [ 'bar' ]
      ])).to.deep.equal({
        bar: 2
      });
    });

    it('should mask an object with multiple paths', function() {
      chai.expect(Domain.mask({
        foo: 1,
        bar: 2,
        baz: 3
      }, [
        [ 'foo' ],
        [ 'baz' ]
      ])).to.deep.equal({
        foo: 1,
        baz: 3
      });
    });

    it('should mask an object with a single nested path', function() {
      chai.expect(Domain.mask({
        foo: {
          bar: {
            qux: {
              foo: 3
            },
            baz: 1
          }
        }
      }, [
        [ 'foo', 'bar', 'baz' ]
      ])).to.deep.equal({
        foo: {
          bar: {
            baz: 1
          }
        }
      });
    });

  });

  describe('.getFromMapping()', function() {

    it('should return an empty array for a simple mapping', function() {
      chai.expect(Domain.getFromMapping([
        [ 'foo', 'bar' ],
        [ 'foo', 'baz' ]
      ])).to.deep.equal([]);
    });

    it('should avoid duplicates from a set of choices', function() {
      chai.expect(Domain.getFromMapping([
        {
          value: true,
          template: {
            foo: {
              bar: 1
            }
          }
        },
        {
          value: false,
          template: {
            foo: {
              bar: 2
            }
          }
        }
      ])).to.deep.equal([ [ 'foo' ] ]);
    });

    it('should get the domain of a set of choices with multi-key templates', function() {
      chai.expect(Domain.getFromMapping([
        {
          value: true,
          template: {
            foo: {
              bar: 1
            }
          }
        },
        {
          value: false,
          template: {
            foo: {
              bar: 1
            },
            baz: {
              qux: 5
            }
          }
        }
      ])).to.deep.equal([
        [ 'foo' ],
        [ 'baz' ]
      ]);
    });

  });

});
