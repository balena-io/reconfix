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
const cli = require('../../../visuals/cli');

ava('should flatten a list without nested questions', (test) => {
  test.deepEqual(cli.flatten([
    {
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
    }
  ]), [
    {
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
    }
  ]);
});

ava('should flatten a list with single-level nested questions', (test) => {
  test.deepEqual(cli.flatten([
    {
      title: 'Network Type',
      name: 'networkType',
      type: 'list',
      choices: [
        {
          title: 'Wifi',
          name: 'wifi',
          questions: [
            {
              title: 'Wifi SSID',
              name: 'networkSsid',
              type: 'text'
            },
            {
              title: 'Wifi Key',
              name: 'networkKey',
              type: 'password'
            }
          ]
        },
        {
          title: 'Ethernet',
          name: 'ethernet'
        }
      ]
    }
  ]), [
    {
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
    },
    {
      title: 'Wifi SSID',
      name: 'networkSsid',
      type: 'text',
      when: {
        networkType: 'wifi'
      }
    },
    {
      title: 'Wifi Key',
      name: 'networkKey',
      type: 'password',
      when: {
        networkType: 'wifi'
      }
    }
  ]);
});

ava('should flatten a list with two-level nested questions', (test) => {
  test.deepEqual(cli.flatten([
    {
      title: 'Network Type',
      name: 'networkType',
      type: 'list',
      choices: [
        {
          title: 'Wifi',
          name: 'wifi',
          questions: [
            {
              title: 'Wifi SSID',
              name: 'networkSsid',
              type: 'text'
            },
            {
              title: 'Wifi Key',
              name: 'networkKey',
              type: 'password'
            },
            {
              title: 'Wifi Type',
              name: 'networkWifiType',
              type: 'list',
              choices: [
                {
                  title: 'WEP',
                  name: 'wep'
                },
                {
                  title: 'WPA',
                  name: 'wpa'
                },
                {
                  title: 'WPA2',
                  name: 'wpa2',
                  questions: [
                    {
                      title: 'Wlan Interface',
                      name: 'wlanInterface',
                      type: 'text'
                    }
                  ]
                }
              ]
            }
          ]
        },
        {
          title: 'Ethernet',
          name: 'ethernet'
        }
      ]
    }
  ]), [
    {
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
    },
    {
      title: 'Wifi SSID',
      name: 'networkSsid',
      type: 'text',
      when: {
        networkType: 'wifi'
      }
    },
    {
      title: 'Wifi Key',
      name: 'networkKey',
      type: 'password',
      when: {
        networkType: 'wifi'
      }
    },
    {
      title: 'Wifi Type',
      name: 'networkWifiType',
      type: 'list',
      choices: [
        {
          title: 'WEP',
          name: 'wep'
        },
        {
          title: 'WPA',
          name: 'wpa'
        },
        {
          title: 'WPA2',
          name: 'wpa2'
        }
      ],
      when: {
        networkType: 'wifi'
      }
    },
    {
      title: 'Wlan Interface',
      name: 'wlanInterface',
      type: 'text',
      when: {
        networkType: 'wifi',
        networkWifiType: 'wpa2'
      }
    }
  ]);
});
