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

const ava = require('ava');
const _ = require('lodash');
const interpolation = require('../../lib/jsontemplate/interpolation');

_.each([

  // -------------------------------------------------------------------
  // No interpolation
  // -------------------------------------------------------------------

  {
    template: 'Hello world',
    data: {},
    result: 'Hello world'
  },

  // -------------------------------------------------------------------
  // Top level string interpolation
  // -------------------------------------------------------------------

  {
    template: '{{name}}',
    data: {
      name: 'John Doe'
    },
    result: 'John Doe'
  },
  {
    template: 'Hello, {{name}}',
    data: {
      name: 'John Doe'
    },
    result: 'Hello, John Doe'
  },
  {
    template: 'Hello, {{name}}!',
    data: {
      name: 'John Doe'
    },
    result: 'Hello, John Doe!'
  },
  {
    template: 'Foo{{name}}Foo',
    data: {
      name: 'John Doe'
    },
    result: 'FooJohn DoeFoo'
  },
  {
    template: 'Foo{{word}}Foo',
    data: {
      word: 'Foo'
    },
    result: 'FooFooFoo'
  },

  // -------------------------------------------------------------------
  // Top level number interpolation
  // -------------------------------------------------------------------

  {
    template: '{{number}}',
    data: {
      number: 0
    },
    result: 0
  },
  {
    template: '{{age}}',
    data: {
      age: 17
    },
    result: 17
  },
  {
    template: '{{age}}',
    data: {
      age: 21.5
    },
    result: 21.5
  },
  {
    template: '{{number}}',
    data: {
      number: -14
    },
    result: -14
  },
  {
    template: '{{number}}',
    data: {
      number: 5.0
    },
    result: 5.0
  },

  // -------------------------------------------------------------------
  // Top level boolean interpolation
  // -------------------------------------------------------------------

  {
    template: '{{bool}}',
    data: {
      bool: true
    },
    result: true
  },
  {
    template: '{{bool}}',
    data: {
      bool: false
    },
    result: false
  },

  // -------------------------------------------------------------------
  // Special characters in key
  // -------------------------------------------------------------------

  /* eslint-disable camelcase */

  {
    template: '{{$name}}',
    data: {
      $name: 'John Doe'
    },
    result: 'John Doe'
  },
  {
    template: '{{full_name}}',
    data: {
      full_name: 'John Doe'
    },
    result: 'John Doe'
  },

  /* eslint-enable camelcase */

  // -------------------------------------------------------------------
  // Nested string interpolation
  // -------------------------------------------------------------------

  {
    template: '{{foo.bar.baz.name}}',
    data: {
      foo: {
        bar: {
          baz: {
            name: 'John Doe'
          }
        }
      }
    },
    result: 'John Doe'
  },

  // -------------------------------------------------------------------
  // Nested number interpolation
  // -------------------------------------------------------------------

  {
    template: '{{foo.bar.baz.age}}',
    data: {
      foo: {
        bar: {
          baz: {
            age: 21
          }
        }
      }
    },
    result: 21
  },

  // -------------------------------------------------------------------
  // Multiple interpolations
  // -------------------------------------------------------------------

  {
    template: 'Hello, I\'m {{name}} and I\'m {{age}} years old',
    data: {
      name: 'John Doe',
      age: '43'
    },
    result: 'Hello, I\'m John Doe and I\'m 43 years old'
  },
  {
    template: 'These are {{person1.name}} and {{person2.name}}',
    data: {
      person1: {
        name: 'John Doe'
      },
      person2: {
        name: 'Jane Doe'
      }
    },
    result: 'These are John Doe and Jane Doe'
  }

], (testCase) => {

  ava.test(`.interpolateString() should interpolate ${testCase.template}`, (test) => {
    test.deepEqual(interpolation.interpolateString(
      testCase.template,
      testCase.data
    ), testCase.result);
  });

  ava.test(`.deinterpolateString() should deinterpolate ${testCase.result}`, (test) => {
    test.deepEqual(interpolation.deinterpolateString(
      testCase.template,
      testCase.result
    ), testCase.data);
  });

});

ava.test('.interpolateString() should cast positive integer to string if interpolation has context', (test) => {
  test.deepEqual(interpolation.interpolateString('My age is {{age}}', {
    age: 21
  }), 'My age is 21');
});

ava.test('.interpolateString() should cast negative integer to string if interpolation has context', (test) => {
  test.deepEqual(interpolation.interpolateString('The temperature is {{temperature}}', {
    temperature: -5
  }), 'The temperature is -5');
});

ava.test('.interpolateString() should cast positive float to string if interpolation has context', (test) => {
  test.deepEqual(interpolation.interpolateString('Foo {{bar}} baz', {
    bar: 5.1
  }), 'Foo 5.1 baz');
});

ava.test('.interpolateString() should cast negative float to string if interpolation has context', (test) => {
  test.deepEqual(interpolation.interpolateString('Foo {{bar}} baz', {
    bar: -3.3
  }), 'Foo -3.3 baz');
});

ava.test('.interpolateString() should cast true to string if interpolation has context', (test) => {
  test.deepEqual(interpolation.interpolateString('Foo {{bool}} baz', {
    bool: true
  }), 'Foo true baz');
});

ava.test('.interpolateString() should cast false to string if interpolation has context', (test) => {
  test.deepEqual(interpolation.interpolateString('Foo {{bool}} baz', {
    bool: false
  }), 'Foo false baz');
});

ava.test('.interpolateString() should throw if a referenced variable does not exist', (test) => {
  test.throws(() => {
    interpolation.interpolateString('{{foo}}', {});
  }, 'Missing variable foo');
});

ava.test('.interpolateString() should throw if a referenced variable is null', (test) => {
  test.throws(() => {
    interpolation.interpolateString('{{foo}}', {
      foo: null
    });
  }, 'Missing variable foo');
});

ava.test('.interpolateString() should throw if a referenced nested variable does not exist', (test) => {
  test.throws(() => {
    interpolation.interpolateString('{{foo.bar.baz}}', {});
  }, 'Missing variable foo.bar.baz');
});

ava.test('.interpolateString() should ignore unused data variables', (test) => {
  const result = interpolation.interpolateString('{{foo}} {{bar}}', {
    foo: 'FOO',
    bar: 'BAR',
    baz: 'BAZ',
    data: {
      hello: 'world'
    }
  });

  test.deepEqual(result, 'FOO BAR');
});

ava.test('.deinterpolateString() should throw if strings do not match', (test) => {
  test.throws(() => {
    interpolation.deinterpolateString('Hello {{name}}!', 'Hi John Doe!');
  }, 'No match for \'name\'');
});

ava.test('.deinterpolateString() should throw if interpolation result is missing', (test) => {
  test.throws(() => {
    interpolation.deinterpolateString('Hello {{name}}!', 'Hi !');
  }, 'No match for \'name\'');
});
