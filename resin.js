/*
 * Copyright 2016 Resin.io
 *
 * Licensed under the Apache License, Version 2.0 (the 'License');
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *    http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an 'AS IS' BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

'use strict';

const filesystem = require('../lib/engine/filesystem');
const configuration = require('../lib/engine/configuration');
const visuals = require('../visuals/cli');
const reconfix = require('../lib');
const ARGV_IMAGE = process.argv[2];

if (!ARGV_IMAGE) {
  console.error('Usage: [image]');
  process.exit(1);
}

const schema = {
  questions: [
    {
      title: 'GPU Mem',
      name: 'gpuMem',
      type: 'number'
    },
    {
      title: 'Application Update Poll Interval',
      name: 'appUpdatePollInterval',
      type: 'text'
    },
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
  ],
  mapper: [
    {
      template: {
        gpu_mem: '{{gpuMem}}'
      },
      domain: [
        [ 'config_txt', 'gpu_mem' ]
      ]
    },
    {
      template: {
        appUpdatePollInterval: '{{appUpdatePollInterval}}'
      },
      domain: [
        [ 'config_json', 'appUpdatePollInterval' ]
      ]
    },
    {
      property: [ 'networkType' ],
      domain: [
        [ 'network_config', 'service_home_ethernet' ],
        [ 'network_config', 'service_home_wifi' ]
      ],
      choice: [
        {
          value: 'ethernet',
          template: {
            service_home_ethernet: {
              Type: 'ethernet',
              Nameservers: '8.8.8.8,8.8.4.4'
            }
          }
        },
        {
          value: 'wifi',
          template: {
            service_home_ethernet: {
              Type: 'ethernet',
              Nameservers: '8.8.8.8,8.8.4.4'
            },
            service_home_wifi: {
              Hidden: true,
              Type: 'wifi',
              Name: '{{networkSsid}}',
              Passphrase: '{{networkKey}}',
              Nameservers: '8.8.8.8,8.8.4.4'
            }
          }
        }
      ]
    }
  ],
  files: {
    config_txt: {
      type: 'ini',
      location: {
        path: 'config.txt',
        partition: 1
      }
    },
    network_config: {
      type: 'ini',
      location: {
        parent: 'config_json',
        property: [ 'files', 'network/network.config' ]
      }
    },
    config_json: {
      type: 'json',
      location: {
        path: 'config.json',
        partition: 5
      }
    }
  }
};

reconfix.readConfiguration(schema, ARGV_IMAGE).then((data) => {
  return visuals.run(schema.questions, data).then((answers) => {
    return reconfix.writeConfiguration(schema, answers, ARGV_IMAGE);
  });
}).then(() => {
  console.log('Done!');
});
