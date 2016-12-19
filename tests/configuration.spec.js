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
const Configuration = require('../lib/configuration');

describe('Configuration', function() {

  describe('bidirectional property', function() {

    const bidirectional = (title, schema, state, source) => {
      it(`.generate() ${title}`, function() {
        chai.expect(Configuration.generate(schema, state)).to.deep.equal(source);
      });

      it(`.extract() ${title}`, function() {
        chai.expect(Configuration.extract(schema, source)).to.deep.equal({
          tainted: [],
          result: state
        });
      });
    };

    bidirectional('should process a single simple mapping on a single file', {
      foo: {
        properties: {
          bar: {
            type: [ 'number' ],
            mapping: [
              [ 'baz' ]
            ]
          }
        }
      }
    }, {
      bar: 3
    }, {
      foo: {
        baz: 3
      }
    });

    bidirectional('should process a multiple simple mappings on multiple files', {
      foo: {
        properties: {
          name: {
            type: [ 'string' ],
            mapping: [
              [ 'fullname' ]
            ]
          },
          job: {
            type: [ 'string' ],
            mapping: [
              [ 'job', 'title' ]
            ]
          }
        }
      },
      bar: {
        properties: {
          age: {
            type: [ 'number' ],
            mapping: [
              [ 'person', 'age' ]
            ]
          }
        }
      }
    }, {
      name: 'John Doe',
      job: 'Software Engineer',
      age: 35
    }, {
      foo: {
        fullname: 'John Doe',
        job: {
          title: 'Software Engineer'
        }
      },
      bar: {
        person: {
          age: 35
        }
      }
    });

  });

  describe('.extract()', function() {

    it('should add a file to the "tainted" list if something failed', function() {
      chai.expect(Configuration.extract({
        foo: {
          properties: {
            bar: {
              type: [ 'number' ],
              mapping: [
                [ 'xxx' ]
              ]
            }
          }
        },
        bar: {
          properties: {
            baz: {
              type: [ 'string' ],
              mapping: [
                [ 'yyy' ]
              ]
            }
          }
        }
      }, {
        foo: {
          xxx: 'foo'
        },
        bar: {
          yyy: 'bar'
        }
      })).to.deep.equal({
        tainted: [ 'foo' ],
        result: {
          baz: 'bar'
        }
      });
    });

    it('should be able to mark more than one file as tainted', function() {
      chai.expect(Configuration.extract({
        foo: {
          properties: {
            bar: {
              type: [ 'number' ],
              mapping: [
                [ 'xxx' ]
              ]
            }
          }
        },
        bar: {
          properties: {
            baz: {
              type: [ 'string' ],
              mapping: [
                [ 'yyy' ],
                [ 'zzz' ]
              ]
            }
          }
        },
        success: {
          properties: {
            value: {
              type: [ 'string' ],
              mapping: [
                [ 'the', 'value' ]
              ]
            }
          }
        }
      }, {
        foo: {
          xxx: 'foo'
        },
        bar: {
          yyy: 'bar',
          zzz: 'qux'
        },
        success: {
          the: {
            value: 'success'
          }
        }
      })).to.deep.equal({
        tainted: [ 'foo', 'bar' ],
        result: {
          value: 'success'
        }
      });
    });

    it('should return an empty object if everything is tainted', function() {
      chai.expect(Configuration.extract({
        foo: {
          properties: {
            bar: {
              type: [ 'number' ],
              mapping: [
                [ 'xxx' ]
              ]
            }
          }
        },
        bar: {
          properties: {
            baz: {
              type: [ 'string' ],
              mapping: [
                [ 'yyy' ],
                [ 'zzz' ]
              ]
            }
          }
        }
      }, {
        foo: {
          xxx: 'foo'
        },
        bar: {
          yyy: 'bar',
          zzz: 'qux'
        }
      })).to.deep.equal({
        tainted: [ 'foo', 'bar' ],
        result: {}
      });
    });

  });

});
