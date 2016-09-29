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
const inquirer = require('../../../../../lib/visuals/cli/integrations/inquirer');

ava.test('should throw if type is not recognised', (test) => {
  test.throws(() => {
    inquirer.transpileQuestion({
      title: 'Foo',
      name: 'foo',
      type: 'foo'
    });
  }, 'Unknown question type: foo');
});

ava.test('should throw if question has no title', (test) => {
  test.throws(() => {
    inquirer.transpileQuestion({
      name: 'foo',
      type: 'text'
    });
  }, 'Invalid question title: undefined');
});

ava.test('should throw if question has an invalid title', (test) => {
  test.throws(() => {
    inquirer.transpileQuestion({
      title: [ 'foo', 'bar' ],
      name: 'foo',
      type: 'text'
    });
  }, 'Invalid question title: foo,bar');
});

ava.test('should throw if question has no name', (test) => {
  test.throws(() => {
    inquirer.transpileQuestion({
      title: 'Foo',
      type: 'text'
    });
  }, 'Invalid question name: undefined');
});

ava.test('should throw if question has an invalid name', (test) => {
  test.throws(() => {
    inquirer.transpileQuestion({
      title: 'Foo',
      name: [ 'foo', 'bar' ],
      type: 'text'
    });
  }, 'Invalid question name: foo,bar');
});

ava.test('it should transpile a basic text question', (test) => {
  test.deepEqual(inquirer.transpileQuestion({
    title: 'Wifi SSID',
    name: 'ssid',
    type: 'text'
  }), {
    message: 'Wifi SSID',
    name: 'ssid',
    type: 'input'
  });
});

ava.test('it should transpile a basic password question', (test) => {
  test.deepEqual(inquirer.transpileQuestion({
    title: 'Wifi Key',
    name: 'key',
    type: 'password'
  }), {
    message: 'Wifi Key',
    name: 'key',
    type: 'password'
  });
});

ava.test('it should transpile a basic number question', (test) => {
  test.deepEqual(inquirer.transpileQuestion({
    title: 'Update Poll Interval',
    name: 'updatePollInterval',
    type: 'number'
  }), {
    message: 'Update Poll Interval',
    name: 'updatePollInterval',
    type: 'input'
  });
});

ava.test('it should transpile a basic editor question', (test) => {
  test.deepEqual(inquirer.transpileQuestion({
    title: 'Welcome message',
    name: 'welcome',
    type: 'editor'
  }), {
    message: 'Welcome message',
    name: 'welcome',
    type: 'editor'
  });
});

ava.test('it should allow a default text value', (test) => {
  test.deepEqual(inquirer.transpileQuestion({
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

ava.test('it should allow a default password value', (test) => {
  test.deepEqual(inquirer.transpileQuestion({
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

ava.test('it should allow a default number value', (test) => {
  test.deepEqual(inquirer.transpileQuestion({
    title: 'Update Poll Interval',
    name: 'updatePollInterval',
    type: 'number',
    default: 60000
  }), {
    message: 'Update Poll Interval',
    name: 'updatePollInterval',
    type: 'input',

    // Inquirer doesn't support a number input type, so this
    // test ensures the number is converted into a string.
    default: '60000'

  });
});

ava.test('it should allow a default editor value', (test) => {
  test.deepEqual(inquirer.transpileQuestion({
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

ava.test('it should transpile a basic list question', (test) => {
  test.deepEqual(inquirer.transpileQuestion({
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

ava.test('it should allow a default list value', (test) => {
  test.deepEqual(inquirer.transpileQuestion({
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

ava.test('it should transpile a basic checkbox question', (test) => {
  test.deepEqual(inquirer.transpileQuestion({
    title: 'Enable HDMI',
    name: 'hdmi',
    type: 'checkbox'
  }), {
    message: 'Enable HDMI',
    name: 'hdmi',
    type: 'checkbox'
  });
});

ava.test('it should allow a default checkbox value', (test) => {
  test.deepEqual(inquirer.transpileQuestion({
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
