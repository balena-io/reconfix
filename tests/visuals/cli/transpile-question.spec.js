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
const cli = require('../../../visuals/cli');

ava('should throw if type is not recognised', (test) => {
  test.throws(() => {
    cli.transpileQuestion({
      title: 'Foo',
      name: 'foo',
      type: 'foo'
    });
  }, {
    message: 'Unknown question type: foo'
  });
});

ava('should throw if question has no title', (test) => {
  test.throws(() => {
    cli.transpileQuestion({
      name: 'foo',
      type: 'text'
    });
  }, {
    message: 'Invalid question title: undefined'
  });
});

ava('should throw if question has an invalid title', (test) => {
  test.throws(() => {
    cli.transpileQuestion({
      title: [ 'foo', 'bar' ],
      name: 'foo',
      type: 'text'
    });
  }, {
    message: 'Invalid question title: foo,bar'
  });
});

ava('should throw if question has no name', (test) => {
  test.throws(() => {
    cli.transpileQuestion({
      title: 'Foo',
      type: 'text'
    });
  }, {
    message: 'Invalid question name: undefined'
  });
});

ava('should throw if question has an invalid name', (test) => {
  test.throws(() => {
    cli.transpileQuestion({
      title: 'Foo',
      name: [ 'foo', 'bar' ],
      type: 'text'
    });
  }, {
    message: 'Invalid question name: foo,bar'
  });
});

ava('it should transpile a basic text question', (test) => {
  test.deepEqual(cli.transpileQuestion({
    title: 'Wifi SSID',
    name: 'ssid',
    type: 'text'
  }), {
    message: 'Wifi SSID',
    name: 'ssid',
    type: 'input'
  });
});

ava('it should transpile a basic password question', (test) => {
  test.deepEqual(cli.transpileQuestion({
    title: 'Wifi Key',
    name: 'key',
    type: 'password'
  }), {
    message: 'Wifi Key',
    name: 'key',
    type: 'password'
  });
});

ava('it should transpile a basic number question', (test) => {
  const question = cli.transpileQuestion({
    title: 'Update Poll Interval',
    name: 'updatePollInterval',
    type: 'number'
  });

  test.deepEqual(_.omitBy(question, _.isFunction), {
    message: 'Update Poll Interval',
    name: 'updatePollInterval',
    type: 'input'
  });
});

_.each([
  {
    value: '123',
    expected: true
  },
  {
    value: '0',
    expected: true
  },
  {
    value: '-1',
    expected: true
  },
  {
    value: '3.5',
    expected: true
  },
  {
    value: '.5',
    expected: true
  },
  {
    value: '-10.3',
    expected: true
  },
  {
    value: 5,
    expected: true
  },
  {
    value: 'foo',
    expected: 'Invalid number'
  }
], (testCase) => {
  ava(`it should return ${testCase.expected} for ${testCase.value} number validation`, (test) => {
    const question = cli.transpileQuestion({
      title: 'Update Poll Interval',
      name: 'updatePollInterval',
      type: 'number'
    });

    test.is(question.validate(testCase.value), testCase.expected);
  });
});

_.each([
  {
    value: '123',
    expected: 123
  },
  {
    value: '0',
    expected: 0
  },
  {
    value: '-1',
    expected: -1
  },
  {
    value: '3.5',
    expected: 3.5
  },
  {
    value: '.5',
    expected: 0.5
  },
  {
    value: '-10.3',
    expected: -10.3
  }
], (testCase) => {
  ava(`it should return ${testCase.expected} for ${testCase.value} number filter`, (test) => {
    const question = cli.transpileQuestion({
      title: 'Update Poll Interval',
      name: 'updatePollInterval',
      type: 'number'
    });

    test.is(question.filter(testCase.value), testCase.expected);
  });
});

ava('it should transpile a basic editor question', (test) => {
  test.deepEqual(cli.transpileQuestion({
    title: 'Welcome message',
    name: 'welcome',
    type: 'editor'
  }), {
    message: 'Welcome message',
    name: 'welcome',
    type: 'editor'
  });
});

ava('it should allow a default text value', (test) => {
  test.deepEqual(cli.transpileQuestion({
    title: 'Wifi SSID',
    name: 'ssid',
    type: 'text',
    default: 'mynetwork'
  }), {
    message: 'Wifi SSID',
    name: 'ssid',
    type: 'input',
    default: 'mynetwork'
  });
});

ava('it should allow a default password value', (test) => {
  test.deepEqual(cli.transpileQuestion({
    title: 'Wifi Key',
    name: 'key',
    type: 'password',
    default: 'secret'
  }), {
    message: 'Wifi Key',
    name: 'key',
    type: 'password',
    default: 'secret'
  });
});

ava('it should allow a default number value', (test) => {
  const question = cli.transpileQuestion({
    title: 'Update Poll Interval',
    name: 'updatePollInterval',
    type: 'number',
    default: 60000
  });

  test.deepEqual(_.omitBy(question, _.isFunction), {
    message: 'Update Poll Interval',
    name: 'updatePollInterval',
    type: 'input',
    default: 60000
  });
});

ava('it should allow a default editor value', (test) => {
  test.deepEqual(cli.transpileQuestion({
    title: 'Welcome message',
    name: 'welcome',
    type: 'editor',
    default: 'Welcome!'
  }), {
    message: 'Welcome message',
    name: 'welcome',
    type: 'editor',
    default: 'Welcome!'
  });
});

ava('it should transpile a basic list question', (test) => {
  test.deepEqual(cli.transpileQuestion({
    title: 'Network Type',
    name: 'networkType',
    type: 'list',
    choices: [
      {
        title: 'Wifi',
        name: 'wifi'
      },
      {
        title: 'Ethernet',
        name: 'ethernet'
      }
    ]
  }), {
    message: 'Network Type',
    name: 'networkType',
    type: 'list',
    choices: [
      {
        name: 'Wifi',
        value: 'wifi'
      },
      {
        name: 'Ethernet',
        value: 'ethernet'
      }
    ]
  });
});

ava('it should allow a default list value', (test) => {
  test.deepEqual(cli.transpileQuestion({
    title: 'Network Type',
    name: 'networkType',
    type: 'list',
    default: 'wifi',
    choices: [
      {
        title: 'Wifi',
        name: 'wifi'
      },
      {
        title: 'Ethernet',
        name: 'ethernet'
      }
    ]
  }), {
    message: 'Network Type',
    name: 'networkType',
    type: 'list',
    default: 'wifi',
    choices: [
      {
        name: 'Wifi',
        value: 'wifi'
      },
      {
        name: 'Ethernet',
        value: 'ethernet'
      }
    ]
  });
});

ava('it should transpile a basic checkbox question', (test) => {
  test.deepEqual(cli.transpileQuestion({
    title: 'Enable HDMI',
    name: 'hdmi',
    type: 'checkbox'
  }), {
    message: 'Enable HDMI',
    name: 'hdmi',
    type: 'checkbox'
  });
});

ava('it should allow a default checkbox value', (test) => {
  test.deepEqual(cli.transpileQuestion({
    title: 'Enable HDMI',
    name: 'hdmi',
    type: 'checkbox',
    default: false
  }), {
    message: 'Enable HDMI',
    name: 'hdmi',
    type: 'checkbox',
    default: false
  });
});
