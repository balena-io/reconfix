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
const Properties = require('../lib/properties');

describe('Properties', function() {

  describe('.isLeafProperty()', function() {

    it('should return true if property is a leaf property', function() {
      chai.expect(Properties.isLeafProperty({
        type: [ 'number' ]
      })).to.be.true;
    });

    it('should return false if property is not leaf property', function() {
      chai.expect(Properties.isLeafProperty({
        foo: {
          type: [ 'number' ]
        }
      })).to.be.false;
    });

    it('should return false if property is not leaf property but contains a "type" child', function() {
      chai.expect(Properties.isLeafProperty({
        type: {
          type: [ 'number' ]
        }
      })).to.be.false;
    });

  });

  describe('.listPropertyPaths()', function() {

    it('should be able to list a single property', function() {
      chai.expect(Properties.listPropertyPaths({
        foo: {
          type: [ 'number' ]
        }
      })).to.deep.equal([
        [ 'foo' ]
      ]);
    });

    it('should be able to list a multiple properties', function() {
      chai.expect(Properties.listPropertyPaths({
        foo: {
          type: [ 'number' ]
        },
        bar: {
          type: [ 'number' ]
        },
        baz: {
          type: [ 'number' ]
        }
      })).to.deep.equal([
        [ 'foo' ],
        [ 'bar' ],
        [ 'baz' ]
      ]);
    });

    it('should be able to list a single nested property', function() {
      chai.expect(Properties.listPropertyPaths({
        foo: {
          bar: {
            baz: {
              type: [ 'number' ]
            }
          }
        }
      })).to.deep.equal([
        [ 'foo', 'bar', 'baz' ]
      ]);
    });

    it('should be able to list multiple nested property', function() {
      chai.expect(Properties.listPropertyPaths({
        qux: {
          type: [ 'boolean' ]
        },
        foo: {
          hello: {
            type: [ 'string' ]
          },
          bar: {
            baz: {
              type: [ 'number' ]
            }
          }
        }
      })).to.deep.equal([
        [ 'qux' ],
        [ 'foo', 'hello' ],
        [ 'foo', 'bar', 'baz' ]
      ]);
    });

  });

  describe('.getPropertyMapping()', function() {

    it('should return a direct mapping', function() {
      chai.expect(Properties.getPropertyMapping({
        foo: {
          type: [ 'string' ],
          mapping: [
            [ 'option1' ]
          ]
        }
      }, [
        'foo'
      ])).to.deep.equal([
        [ 'option1' ]
      ]);
    });

    it('should return a nested direct mapping', function() {
      chai.expect(Properties.getPropertyMapping({
        foo: {
          bar: {
            type: [ 'string' ],
            mapping: [
              [ 'option1' ]
            ]
          }
        }
      }, [
        'foo',
        'bar'
      ])).to.deep.equal([
        [ 'option1' ]
      ]);
    });

    it('should return undefined is property does not exist', function() {
      chai.expect(Properties.getPropertyMapping({
        foo: {
          type: [ 'string' ],
          mapping: [
            [ 'option1' ]
          ]
        }
      }, [
        'foo',
        'bar'
      ])).to.deep.equal(undefined);
    });

  });

  describe('.getPropertyPaths()', function() {

    it('should keep direct mappings in order', function() {
      chai.expect(Properties.getPropertyPaths({
        foo: {
          type: [ 'string' ],
          mapping: [
            [ 'option1' ]
          ]
        },
        bar: {
          type: [ 'string' ],
          mapping: [
            [ 'option2' ]
          ]
        },
        baz: {
          type: [ 'string' ],
          mapping: [
            [ 'option3' ]
          ]
        }
      })).to.deep.equal([
        [ 'foo' ],
        [ 'bar' ],
        [ 'baz' ]
      ]);
    });

    it('should put template based properties first', function() {
      chai.expect(Properties.getPropertyPaths({
        foo: {
          type: [ 'string' ],
          mapping: [
            [ 'option1' ]
          ]
        },
        bar: {
          type: [ 'string' ],
          mapping: [
            {
              value: 'hello',
              template: {
                greeting: 'formal'
              }
            },
            {
              value: 'hey',
              template: {
                greeting: 'informal'
              }
            }
          ]
        },
        baz: {
          type: [ 'string' ],
          mapping: [
            [ 'option2' ]
          ]
        }
      })).to.deep.equal([
        [ 'bar' ],
        [ 'foo' ],
        [ 'baz' ]
      ]);
    });

    it('should put multiple template based properties first', function() {
      chai.expect(Properties.getPropertyPaths({
        foo: {
          type: [ 'string' ],
          mapping: [
            [ 'option1' ]
          ]
        },
        bar: {
          type: [ 'string' ],
          mapping: [
            {
              value: 'hello',
              template: {
                greeting: 'formal'
              }
            },
            {
              value: 'hey',
              template: {
                greeting: 'informal'
              }
            }
          ]
        },
        baz: {
          type: [ 'string' ],
          mapping: [
            [ 'option2' ]
          ]
        },
        qux: {
          type: [ 'string' ],
          mapping: [
            {
              value: 1,
              template: {
                foo: true
              }
            },
            {
              value: 0,
              template: {
                foo: false
              }
            }
          ]
        }
      })).to.deep.equal([
        [ 'bar' ],
        [ 'qux' ],
        [ 'foo' ],
        [ 'baz' ]
      ]);
    });

    it('should put mixed based properties first', function() {
      chai.expect(Properties.getPropertyPaths({
        foo: {
          type: [ 'string' ],
          mapping: [
            [ 'option1' ]
          ]
        },
        bar: {
          type: [ 'string' ],
          mapping: [
            {
              value: 'hello',
              template: {
                greeting: 'formal'
              }
            },
            {
              value: 'hey',
              template: {
                greeting: 'informal'
              }
            }
          ]
        },
        baz: {
          type: [ 'string' ],
          mapping: [
            [ 'option2' ]
          ]
        },
        qux: {
          type: [ 'string' ],
          mapping: [
            [ 'foo' ],
            {
              value: 1,
              template: {
                foo: true
              }
            },
            {
              value: 0,
              template: {
                foo: false
              }
            }
          ]
        }
      })).to.deep.equal([
        [ 'qux' ],
        [ 'bar' ],
        [ 'foo' ],
        [ 'baz' ]
      ]);
    });

  });

});
