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
const Template = require('../lib/template');

describe('Template', function() {

  describe('.isTypeWildcard()', function() {

    it('should return false if type wildcard is empty', function() {
      chai.expect(Template.isTypeWildcard('[[]]')).to.be.false;
    });

    it('should return true if string is a type wildcard', function() {
      chai.expect(Template.isTypeWildcard('[[string]]')).to.be.true;
    });

    it('should return false if string is lacking a set of brackets', function() {
      chai.expect(Template.isTypeWildcard('[string]')).to.be.false;
    });

    it('should return false if string contains spaces', function() {
      chai.expect(Template.isTypeWildcard('[[str  ing]]')).to.be.false;
    });

    it('should return false if string contains numbers', function() {
      chai.expect(Template.isTypeWildcard('[[number123]]')).to.be.false;
    });

    it('should return false if string contains non-alphanumeric characters', function() {
      chai.expect(Template.isTypeWildcard('[[my-type$]]')).to.be.false;
    });

    it('should return false if string has outer leading space', function() {
      chai.expect(Template.isTypeWildcard('  [[string]]')).to.be.false;
    });

    it('should return false if string has inner leading space', function() {
      chai.expect(Template.isTypeWildcard('[[   string]]')).to.be.false;
    });

    it('should return false if string has outer trailing space', function() {
      chai.expect(Template.isTypeWildcard('[[string]]   ')).to.be.false;
    });

    it('should return false if string has inner trailing space', function() {
      chai.expect(Template.isTypeWildcard('[[string   ]]')).to.be.false;
    });

    it('should accept a | separated list of types', function() {
      chai.expect(Template.isTypeWildcard('[[string|number|object]]')).to.be.true;
    });

  });

  describe('.getWildcardType()', function() {

    it('should return undefined given an invalid wildcard', function() {
      chai.expect(Template.getWildcardType('foobar')).to.be.undefined;
    });

    it('should extract a single valid wildcard type', function() {
      chai.expect(Template.getWildcardType('[[number]]')).to.deep.equal([ 'number' ]);
    });

    it('should extract a multiple valid wildcard types', function() {
      chai.expect(Template.getWildcardType('[[number|string|object]]')).to.deep.equal([
        'number',
        'string',
        'object'
      ]);
    });

  });

  describe('.matches()', function() {

    it('should return true if subset matches', function() {
      chai.expect(Template.matches({
        foo: {
          bar: 2,
          baz: {
            qux: 5
          }
        }
      }, {
        foo: {
          bar: 2
        }
      })).to.be.true;
    });

    it('should return false if the template is a superset of the data', function() {
      chai.expect(Template.matches({
        foo: {
          bar: 2
        }
      }, {
        foo: {
          bar: 2
        },
        baz: {
          qux: 5
        }
      })).to.be.false;
    });

    it('should return true if subset does not match', function() {
      chai.expect(Template.matches({
        foo: {
          bar: 2,
          baz: {
            qux: 5
          }
        }
      }, {
        foo: {
          qux: 2
        }
      })).to.be.false;
    });

    it('should return false if an array is a subset of the other', function() {
      chai.expect(Template.matches({
        foo: {
          bar: [ 1, 2, 3 ]
        }
      }, {
        foo: {
          bar: [ 1, 2 ]
        }
      })).to.be.false;
    });

    it('should return false if array do not match', function() {
      chai.expect(Template.matches({
        foo: {
          bar: [ 1, 2, 3 ]
        }
      }, {
        foo: {
          bar: [ 0, 1, 2 ]
        }
      })).to.be.false;
    });

    it('should return true if arrays match entirely', function() {
      chai.expect(Template.matches({
        foo: {
          bar: [ 1, 2, 3 ]
        }
      }, {
        foo: {
          bar: [ 1, 2, 3 ]
        }
      })).to.be.true;
    });

    it('should throw if a wildcard type is invalid', function() {
      chai.expect(() => {
        Template.matches({
          foo: {
            bar: 'baz'
          }
        }, {
          foo: {
            bar: '[[foo]]'
          }
        });
      }).to.throw('Invalid type: foo');
    });

    it('should return false if a wildcard gets matched with "undefined"', function() {
      chai.expect(Template.matches({
        foo: {
          bar: undefined
        }
      }, {
        foo: {
          bar: '[[string]]'
        }
      })).to.be.false;
    });

    it('should return true if a string wildcard matches', function() {
      chai.expect(Template.matches({
        foo: {
          bar: 'foo bar'
        }
      }, {
        foo: {
          bar: '[[string]]'
        }
      })).to.be.true;
    });

    it('should return true if a string wildcard does not match', function() {
      chai.expect(Template.matches({
        foo: {
          bar: 1
        }
      }, {
        foo: {
          bar: '[[string]]'
        }
      })).to.be.false;
    });

    it('should return true if a number wildcard matches', function() {
      chai.expect(Template.matches({
        foo: {
          bar: 3
        }
      }, {
        foo: {
          bar: '[[number]]'
        }
      })).to.be.true;
    });

    it('should return true if a number wildcard does not match', function() {
      chai.expect(Template.matches({
        foo: {
          bar: '3'
        }
      }, {
        foo: {
          bar: '[[number]]'
        }
      })).to.be.false;
    });

    it('should return true if a boolean wildcard matches', function() {
      chai.expect(Template.matches({
        foo: {
          bar: false
        }
      }, {
        foo: {
          bar: '[[boolean]]'
        }
      })).to.be.true;
    });

    it('should return false if a boolean wildcard does not match', function() {
      chai.expect(Template.matches({
        foo: {
          bar: 'true'
        }
      }, {
        foo: {
          bar: '[[boolean]]'
        }
      })).to.be.false;
    });

    it('should return true if an object wildcard matches', function() {
      chai.expect(Template.matches({
        foo: {
          bar: {
            qux: 1,
            hello: {
              world: 3
            }
          }
        }
      }, {
        foo: {
          bar: '[[object]]'
        }
      })).to.be.true;
    });

    it('should return false if an object wildcard does not match', function() {
      chai.expect(Template.matches({
        foo: {
          bar: [ 1, 2, 3 ]
        }
      }, {
        foo: {
          bar: '[[object]]'
        }
      })).to.be.false;
    });

    it('should return true if an array wildcard matches', function() {
      chai.expect(Template.matches({
        foo: {
          bar: [ 1, 2, 3 ]
        }
      }, {
        foo: {
          bar: '[[array]]'
        }
      })).to.be.true;
    });

    it('should return false if an array wildcard does not match', function() {
      chai.expect(Template.matches({
        foo: {
          bar: {
            baz: [ 1, 2, 3 ]
          }
        }
      }, {
        foo: {
          bar: '[[array]]'
        }
      })).to.be.false;
    });

  });

  describe('.getTemplateDegree()', function() {

    it('should return zero for an empty template', function() {
      chai.expect(Template.getTemplateDegree({})).to.equal(0);
    });

    it('should calculate the degree of a simple template', function() {
      chai.expect(Template.getTemplateDegree({
        foo: 1,
        bar: 2
      })).to.equal(2);
    });

    it('should calculate the degree of a nested template', function() {
      chai.expect(Template.getTemplateDegree({
        one: {
          two: {
            three: 3
          },
          four: {
            five: 5,
            six: [ 6 ],
            seven: {
              eight: 8
            }
          }
        }
      })).to.equal(8);
    });

  });

  describe('.getHighestDegreeMatchingTemplates()', function() {

    it('should analyse two templates with equal degrees', function() {
      chai.expect(Template.getHighestDegreeMatchingTemplates({
        foo: 2
      }, [
        {
          foo: 1
        },
        {
          foo: 2
        }
      ])).to.deep.equal([
        {
          foo: 2
        }
      ]);
    });

    it('should analyse two matching templates with different degrees', function() {
      chai.expect(Template.getHighestDegreeMatchingTemplates({
        foo: 1,
        bar: 2
      }, [
        {
          foo: 1
        },
        {
          foo: 1,
          bar: 2
        }
      ])).to.deep.equal([
        {
          foo: 1,
          bar: 2
        }
      ]);
    });

    it('should analyse two matching templates with equal degrees', function() {
      chai.expect(Template.getHighestDegreeMatchingTemplates({
        foo: 1,
        bar: 2,
        baz: 3
      }, [
        {
          foo: 1,
          bar: 2
        },
        {
          bar: 2,
          baz: 3
        }
      ])).to.deep.equal([
        {
          foo: 1,
          bar: 2
        },
        {
          bar: 2,
          baz: 3
        }
      ]);
    });

  });

});
