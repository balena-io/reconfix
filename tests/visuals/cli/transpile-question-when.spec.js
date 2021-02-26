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

_.attempt(() => {
  const question = cli.transpileQuestion({
    title: 'Wifi SSID',
    name: 'ssid',
    type: 'text',
    when: {
      networkType: 'wifi'
    }
  });

  ava('(string property) should return false for an empty object', (test) => {
    test.false(question.when({}));
  });

  ava('(string property) should return true if it matches', (test) => {
    test.true(question.when({
      networkType: 'wifi'
    }));
  });

  ava('(string property) should return true if is a subset', (test) => {
    test.true(question.when({
      networkType: 'wifi',
      wifiKey: 'foo bar'
    }));
  });

  ava('(string property) should return false if is not a subset', (test) => {
    test.false(question.when({
      wifiKey: 'foo bar'
    }));
  });

  ava('(string property) should return true if it does not match', (test) => {
    test.false(question.when({
      networkType: 'ethernet'
    }));
  });
});

_.attempt(() => {
  const question = cli.transpileQuestion({
    title: 'HDMI',
    name: 'hdmi',
    type: 'checkbox',
    when: {
      enableScreen: true
    }
  });

  ava('(boolean property) should return false for an empty object', (test) => {
    test.false(question.when({}));
  });

  ava('(boolean property) should return true if true', (test) => {
    test.true(question.when({
      enableScreen: true
    }));
  });

  ava('(boolean property) should return false if false', (test) => {
    test.false(question.when({
      enableScreen: false
    }));
  });

  ava('(boolean property) should return false if 0', (test) => {
    test.false(question.when({
      enableScreen: 0
    }));
  });

  ava('(boolean property) should return false if 1', (test) => {
    test.false(question.when({
      enableScreen: 0
    }));
  });

  ava('(boolean property) should return false if undefined', (test) => {
    test.false(question.when({
      enableScreen: undefined
    }));
  });

  ava('(boolean property) should return false if null', (test) => {
    test.false(question.when({
      enableScreen: undefined
    }));
  });
});

_.attempt(() => {
  const question = cli.transpileQuestion({
    title: 'HDMI',
    name: 'hdmi',
    type: 'checkbox',
    when: {
      capabilities: [
        'screen',
        'interactive',
        'touch'
      ]
    }
  });

  ava('(string array property) should return false if empty object', (test) => {
    test.false(question.when({}));
  });

  ava('(string array property) should return true if it matches', (test) => {
    test.true(question.when({
      capabilities: [
        'screen',
        'interactive',
        'touch'
      ]
    }));
  });

  ava('(string array property) should return false if subset', (test) => {
    test.false(question.when({
      capabilities: [
        'screen',
        'touch'
      ]
    }));
  });

  ava('(string array property) should return true if superset', (test) => {
    test.true(question.when({
      capabilities: [
        'screen',
        'interactive',
        'touch',
        'blink',
        'battery'
      ]
    }));
  });

});

_.attempt(() => {
  const question = cli.transpileQuestion({
    title: 'HDMI',
    name: 'hdmi',
    type: 'checkbox',
    when: {
      screen: {
        type: 'led',
        manufacturer: {
          name: 'Samsung',
          serial: 'xxxxxxx'
        }
      }
    }
  });

  ava('(nested object property) should return false if empty object', (test) => {
    test.false(question.when({}));
  });

  ava('(nested object property) should return true if it matches', (test) => {
    test.true(question.when({
      screen: {
        type: 'led',
        manufacturer: {
          name: 'Samsung',
          serial: 'xxxxxxx'
        }
      }
    }));
  });

  ava('(nested object property) should return false if subset', (test) => {
    test.false(question.when({
      screen: {
        manufacturer: {
          name: 'Samsung',
          serial: 'xxxxxxx'
        }
      }
    }));
  });

  ava('(nested object property) should return true if superset', (test) => {
    test.true(question.when({
      screen: {
        type: 'led',
        foo: 'bar',
        manufacturer: {
          foo: 'bar',
          name: 'Samsung',
          serial: 'xxxxxxx'
        }
      },
      foo: 'bar'
    }));
  });

});

_.attempt(() => {
  const question = cli.transpileQuestion({
    title: 'HDMI',
    name: 'hdmi',
    type: 'checkbox',
    when: {
      capabilities: [
        {
          name: 'screen'
        },
        {
          name: 'interactive'
        },
        {
          name: 'touch'
        }
      ]
    }
  });

  ava('(object array property) should return false if empty object', (test) => {
    test.false(question.when({}));
  });

  ava('(object array property) should return true if it matches', (test) => {
    test.true(question.when({
      capabilities: [
        {
          name: 'screen'
        },
        {
          name: 'interactive'
        },
        {
          name: 'touch'
        }
      ]
    }));
  });

  ava('(object array property) should return false if subset', (test) => {
    test.false(question.when({
      capabilities: [
        {
          name: 'screen'
        },
        {
          name: 'interactive'
        }
      ]
    }));
  });

  ava('(object array property) should return true if superset', (test) => {
    test.true(question.when({
      capabilities: [
        {
          name: 'screen'
        },
        {
          name: 'interactive'
        },
        {
          name: 'touch'
        },
        {
          name: 'blink'
        },
        {
          name: 'battery'
        }
      ]
    }));
  });

});

_.attempt(() => {
  const question = cli.transpileQuestion({
    title: 'Wifi SSID',
    name: 'ssid',
    type: 'text',
    when: {
      enableWifi: true,
      network: true,
      networkType: 'wifi'
    }
  });

  ava('(multiple string property) should return true if it matches', (test) => {
    test.true(question.when({
      enableWifi: true,
      network: true,
      networkType: 'wifi'
    }));
  });

  ava('(multiple string property) should return true if is a subset', (test) => {
    test.true(question.when({
      enableWifi: true,
      network: true,
      networkType: 'wifi',
      enableFoo: false,
      wifiKey: 'secret'
    }));
  });

  ava('(multiple string property) should return false if is not a subset', (test) => {
    test.false(question.when({
      enableWifi: true,
      networkType: 'wifi'
    }));
  });

  ava('(multiple string property) should return true if it does not match', (test) => {
    test.false(question.when({
      enableWifi: true,
      network: false,
      networkType: 'wifi'
    }));
  });
});
