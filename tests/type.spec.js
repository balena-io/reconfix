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
const Type = require('../lib/type');

describe('Type', function() {

  describe('.isValidType()', function() {

    it('should return true for "string"', function() {
      chai.expect(Type.isValidType('string')).to.be.true;
    });

    it('should return false for "STRING"', function() {
      chai.expect(Type.isValidType('STRING')).to.be.false;
    });

    it('should return false for "String"', function() {
      chai.expect(Type.isValidType('String')).to.be.false;
    });

    it('should return true for "number"', function() {
      chai.expect(Type.isValidType('number')).to.be.true;
    });

    it('should return true for "boolean"', function() {
      chai.expect(Type.isValidType('boolean')).to.be.true;
    });

    it('should return true for "object"', function() {
      chai.expect(Type.isValidType('object')).to.be.true;
    });

    it('should return true for "array"', function() {
      chai.expect(Type.isValidType('object')).to.be.true;
    });

  });

  describe('.matchesType()', function() {

    it('should throw if type is invalid', function() {
      chai.expect(() => {
        Type.matchesType('foo', 'bar');
      }).to.throw('Invalid type: foo');
    });

    it('should match a negative integer as a number', function() {
      chai.expect(Type.matchesType('number', -3)).to.be.true;
    });

    it('should match a negative float as a number', function() {
      chai.expect(Type.matchesType('number', -5.8)).to.be.true;
    });

    it('should match a positive integer as a number', function() {
      chai.expect(Type.matchesType('number', 3)).to.be.true;
    });

    it('should match a positive float as a number', function() {
      chai.expect(Type.matchesType('number', 5.8)).to.be.true;
    });

    it('should match a zero as a number', function() {
      chai.expect(Type.matchesType('number', 0)).to.be.true;
    });

    it('should match an empty string as a string', function() {
      chai.expect(Type.matchesType('string', '')).to.be.true;
    });

    it('should match a single character string as a string', function() {
      chai.expect(Type.matchesType('string', 'a')).to.be.true;
    });

    it('should match a multiple character string as a string', function() {
      chai.expect(Type.matchesType('string', 'foo bar')).to.be.true;
    });

    it('should match true as a boolean', function() {
      chai.expect(Type.matchesType('boolean', true)).to.be.true;
    });

    it('should match false as a boolean', function() {
      chai.expect(Type.matchesType('boolean', false)).to.be.true;
    });

    it('should not match undefined as a boolean', function() {
      chai.expect(Type.matchesType('boolean', undefined)).to.be.false;
    });

    it('should not match null as a boolean', function() {
      chai.expect(Type.matchesType('boolean', undefined)).to.be.false;
    });

    it('should not match a string as a number', function() {
      chai.expect(Type.matchesType('number', '567')).to.be.false;
    });

    it('should not match a number as a string', function() {
      chai.expect(Type.matchesType('string', 567)).to.be.false;
    });

    it('should match an empty object as an object', function() {
      chai.expect(Type.matchesType('object', {})).to.be.true;
    });

    it('should match an object as an object', function() {
      chai.expect(Type.matchesType('object', {
        foo: 2
      })).to.be.true;
    });

    it('should not match an empty array as an object', function() {
      chai.expect(Type.matchesType('object', [])).to.be.false;
    });

    it('should not match an empty object as an array', function() {
      chai.expect(Type.matchesType('array', {})).to.be.false;
    });

    it('should match an empty array as an array', function() {
      chai.expect(Type.matchesType('array', [])).to.be.true;
    });

    it('should match an empty array as an array', function() {
      chai.expect(Type.matchesType('array', [ 1, 2, 3 ])).to.be.true;
    });

  });

  describe('.matchesSomeType()', function() {

    it('should return true if one type matches', function() {
      chai.expect(Type.matchesSomeType([
        'string',
        'number'
      ], 3)).to.be.true;
    });

    it('should return false if no types match', function() {
      chai.expect(Type.matchesSomeType([
        'string',
        'number'
      ], true)).to.be.false;
    });

  });

});
