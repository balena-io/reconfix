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
const Mapping = require('../lib/mapping');

describe('Mapping', function() {

  describe('bidirectional property', function() {

    const bidirectional = (title, map, object, value) => {
      it(`.map() ${title}`, function() {
        chai.expect(Mapping.map(map, value)).to.deep.equal(object);
      });

      it(`.unmap() ${title}`, function() {
        chai.expect(Mapping.unmap(map, object)).to.deep.equal(value);
      });
    };

    bidirectional('should process a single simple mapping', [
      [ 'foo' ]
    ], {
      foo: 3
    }, 3);

    bidirectional('should process a single nested mapping', [
      [ 'foo', 'bar', 'baz' ]
    ], {
      foo: {
        bar: {
          baz: 3
        }
      }
    }, 3);

    bidirectional('should process multiple simple mappings', [
      [ 'foo' ],
      [ 'bar' ]
    ], {
      foo: 'baz',
      bar: 'baz'
    }, 'baz');

    bidirectional('should process multiple non-scalar mappings', [
      [ 'foo' ],
      [ 'bar' ]
    ], {
      foo: {
        hello: 'world'
      },
      bar: {
        hello: 'world'
      }
    }, {
      hello: 'world'
    });

    bidirectional('should process a boolean template mapping', [
      {
        value: true,
        template: {
          foo: 1
        }
      },
      {
        value: false,
        template: {
          foo: 2
        }
      }
    ], {
      foo: 2
    }, false);

    bidirectional('should process a template and direct mapping', [
      [ 'foo', 'bar', 'baz' ],
      {
        value: 'johndoe',
        template: {
          name: 'John'
        }
      },
      {
        value: 'janedoe',
        template: {
          name: 'Jane'
        }
      }
    ], {
      name: 'John',
      foo: {
        bar: {
          baz: 'johndoe'
        }
      }
    }, 'johndoe');

    bidirectional('should disambiguate matching templates based on templates degrees', [
      {
        value: 'foo',
        template: {
          foo: 1
        }
      },
      {
        value: 'foobar',
        template: {
          foo: 1,
          bar: 2
        }
      }
    ], {
      foo: 1,
      bar: 2
    }, 'foobar');
  });

  describe('.map()', function() {

    it('should ignore null mappings', function() {
      chai.expect(Mapping.map([
        [ 'foo' ]
      ], null)).to.deep.equal({});
    });

    it('should throw if more than one template matches the value', function() {
      chai.expect(() => {
        Mapping.map([
          {
            value: 1,
            template: {
              foo: 1
            }
          },
          {
            value: 1,
            template: {
              foo: 5
            }
          }
        ], 1);
      }).to.throw('Ambiguous mapping for value: 1');
    });

    it('should do nothing if no template matches the value', function() {
      chai.expect(Mapping.map([
        {
          value: 1,
          template: {
            foo: 1
          }
        },
        {
          value: 2,
          template: {
            foo: 2
          }
        }
      ], 5)).to.deep.equal({});
    });

  });

  describe('.unmap()', function() {

    it('should return null if the mapping path leads to undefined', function() {
      chai.expect(Mapping.unmap([
        [ 'foo' ]
      ], {
        foo: undefined
      })).to.deep.equal(null);
    });

    it('should throw if there is an unmapping ambiguity', function() {
      chai.expect(() => {
        Mapping.unmap([
          [ 'foo' ],
          [ 'bar' ]
        ], {
          foo: 3,
          bar: 5
        });
      }).to.throw('Ambiguous values: 3,5');
    });

    it('should throw if there was no template match', function() {
      chai.expect(() => {
        Mapping.unmap([
          {
            value: 1,
            template: {
              foo: 1
            }
          },
          {
            value: 2,
            template: {
              foo: 2
            }
          }
        ], {
          foo: 3
        });
      }).to.throw('No match found');
    });

    it('should throw if there were multiple template matches with the same degree', function() {
      chai.expect(() => {
        Mapping.unmap([
          {
            value: 1,
            template: {
              foo: 1
            }
          },
          {
            value: 2,
            template: {
              bar: 2
            }
          }
        ], {
          foo: 1,
          bar: 2
        });
      }).to.throw('Ambiguous values: 1,2');
    });

    it('should throw if there are equal templates pointing to different values', function() {
      chai.expect(() => {
        Mapping.unmap([
          {
            value: 1,
            template: {
              foo: 1
            }
          },
          {
            value: 2,
            template: {
              foo: 1
            }
          }
        ], {
          foo: 1
        });
      }).to.throw('Ambiguous duplicated template: {"foo":1}');
    });

    it('should not throw if there are equal templates pointing to equal values', function() {
      chai.expect(Mapping.unmap([
        {
          value: 1,
          template: {
            foo: 1
          }
        },
        {
          value: 1,
          template: {
            foo: 1
          }
        }
      ], {
        foo: 1
      })).to.deep.equal(1);
    });

  });

  describe('.getRelationshipsForValue()', function() {

    it('should be able to find no matching relationship', function() {
      chai.expect(Mapping.getRelationshipsForValue([
        {
          value: 'foo',
          template: {
            foo: 1
          }
        },
        {
          value: 'bar',
          template: {
            bar: 2
          }
        }
      ], 'qux')).to.deep.equal([]);
    });

    it('should ignore direct mappings', function() {
      chai.expect(Mapping.getRelationshipsForValue([
        [ 'foo', 'bar' ],
        [ 'bar', 'baz' ],
        {
          value: 'foo',
          template: {
            foo: 1
          }
        },
        {
          value: 'bar',
          template: {
            bar: 2
          }
        }
      ], 'foo')).to.deep.equal([
        {
          value: 'foo',
          template: {
            foo: 1
          }
        }
      ]);
    });

    it('should be able to find a single matching relationship', function() {
      chai.expect(Mapping.getRelationshipsForValue([
        {
          value: true,
          template: {
            foo: 1
          }
        },
        {
          value: false,
          template: {
            foo: 2
          }
        }
      ], true)).to.deep.equal([
        {
          value: true,
          template: {
            foo: 1
          }
        }
      ]);
    });

    it('should be able to find multiple matching relationships', function() {
      chai.expect(Mapping.getRelationshipsForValue([
        {
          value: 'foo',
          template: {
            foo: 1
          }
        },
        {
          value: 'foo',
          template: {
            foo: 2
          }
        },
        {
          value: 'bar',
          template: {
            bar: 1
          }
        }
      ], 'foo')).to.deep.equal([
        {
          value: 'foo',
          template: {
            foo: 1
          }
        },
        {
          value: 'foo',
          template: {
            foo: 2
          }
        }
      ]);
    });

  });

  describe('.getTemplateValue()', function() {

    it('should get the value of an un-ambiguous template', function() {
      chai.expect(Mapping.getTemplateValue([
        {
          value: 'foo',
          template: {
            foo: 1
          }
        },
        {
          value: 'bar',
          template: {
            bar: 1
          }
        }
      ], {
        foo: 1
      })).to.deep.equal('foo');
    });

    it('should get the value of an ambiguous template that always points to the same value', function() {
      chai.expect(Mapping.getTemplateValue([
        {
          value: 'foo',
          template: {
            foo: 1
          }
        },
        {
          value: 'foo',
          template: {
            foo: 1
          }
        }
      ], {
        foo: 1
      })).to.deep.equal('foo');
    });

    it('should throw if the mapping is ambiguous', function() {
      chai.expect(() => {
        Mapping.getTemplateValue([
          {
            value: 'foo',
            template: {
              foo: 1
            }
          },
          {
            value: 'bar',
            template: {
              foo: 1
            }
          }
        ], {
          foo: 1
        });
      }).to.throw('Ambiguous duplicated template: {"foo":1}');
    });

  });

  describe('.isTemplateMapping()', function() {

    it('should return true if mapping is templated based', function() {
      chai.expect(Mapping.isTemplateMapping([
        {
          value: 1,
          template: {
            foo: 'bar'
          }
        },
        {
          value: 2,
          template: {
            foo: 'baz'
          }
        }
      ])).to.be.true;
    });

    it('should return false if mapping is mixed', function() {
      chai.expect(Mapping.isTemplateMapping([
        [ 'foo' ],
        {
          value: 1,
          template: {
            foo: 'bar'
          }
        },
        {
          value: 2,
          template: {
            foo: 'baz'
          }
        }
      ])).to.be.false;
    });

    it('should return false if mapping is direct', function() {
      chai.expect(Mapping.isTemplateMapping([
        [ 'foo' ]
      ])).to.be.false;
    });

  });

  describe('.isDirectMapping()', function() {

    it('should return false if mapping is templated based', function() {
      chai.expect(Mapping.isDirectMapping([
        {
          value: 1,
          template: {
            foo: 'bar'
          }
        },
        {
          value: 2,
          template: {
            foo: 'baz'
          }
        }
      ])).to.be.false;
    });

    it('should return false if mapping is mixed', function() {
      chai.expect(Mapping.isDirectMapping([
        [ 'foo' ],
        {
          value: 1,
          template: {
            foo: 'bar'
          }
        },
        {
          value: 2,
          template: {
            foo: 'baz'
          }
        }
      ])).to.be.false;
    });

    it('should return true if mapping is direct', function() {
      chai.expect(Mapping.isDirectMapping([
        [ 'foo' ]
      ])).to.be.true;
    });

  });

  describe('.isMixedMapping()', function() {

    it('should return false if mapping is templated based', function() {
      chai.expect(Mapping.isMixedMapping([
        {
          value: 1,
          template: {
            foo: 'bar'
          }
        },
        {
          value: 2,
          template: {
            foo: 'baz'
          }
        }
      ])).to.be.false;
    });

    it('should return true if mapping is mixed', function() {
      chai.expect(Mapping.isMixedMapping([
        [ 'foo' ],
        {
          value: 1,
          template: {
            foo: 'bar'
          }
        },
        {
          value: 2,
          template: {
            foo: 'baz'
          }
        }
      ])).to.be.true;
    });

    it('should return false if mapping is direct', function() {
      chai.expect(Mapping.isMixedMapping([
        [ 'foo' ]
      ])).to.be.false;
    });

  });

});
