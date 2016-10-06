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
const configuration = require('../../../lib/engine/configuration');

ava.test('.extract() throw if current data does not match the schema', (test) => {

  /* eslint-disable camelcase */

  const schema = [
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
              type: 'ethernet',
              nameservers: '8.8.8.8,8.8.4.4'
            }
          }
        },
        {
          value: 'wifi',
          template: {
            service_home_ethernet: {
              type: 'ethernet',
              nameservers: '8.8.8.8,8.8.4.4'
            },
            service_home_wifi: {
              hidden: true,
              type: 'wifi',
              name: '{{networkSsid}}',
              passphrase: '{{networkKey}}',
              nameservers: '8.8.8.8,8.8.4.4'
            }
          }
        }
      ]
    }
  ];

  const wet = {
    network_config: {
      service_home_ethernet: {
        type: 'ethernet',
        nameservers: '8.8.8.8,8.8.4.4'
      },
      service_work_wifi: {
        type: 'wifi',
        name: 'resin',
        passphrase: 'secret',
        nameservers: '8.8.8.8'
      }
    }
  };

  /* eslint-enable camelcase */

  test.throws(() => {
    configuration.extract(schema, wet);
  }, [
    'The current state doesn\'t match the schema.',
    '',
    'Current configuration:',
    '',
    '{',
    '  "service_home_ethernet": {',
    '    "type": "ethernet",',
    '    "nameservers": "8.8.8.8,8.8.4.4"',
    '  },',
    '  "service_work_wifi": {',
    '    "type": "wifi",',
    '    "name": "resin",',
    '    "passphrase": "secret",',
    '    "nameservers": "8.8.8.8"',
    '  }',
    '}',
    '',
    'Schema choices:',
    '',
    '[',
    '  {',
    '    "value": "ethernet",',
    '    "template": {',
    '      "service_home_ethernet": {',
    '        "type": "ethernet",',
    '        "nameservers": "8.8.8.8,8.8.4.4"',
    '      }',
    '    }',
    '  },',
    '  {',
    '    "value": "wifi",',
    '    "template": {',
    '      "service_home_ethernet": {',
    '        "type": "ethernet",',
    '        "nameservers": "8.8.8.8,8.8.4.4"',
    '      },',
    '      "service_home_wifi": {',
    '        "hidden": true,',
    '        "type": "wifi",',
    '        "name": "{{networkSsid}}",',
    '        "passphrase": "{{networkKey}}",',
    '        "nameservers": "8.8.8.8,8.8.4.4"',
    '      }',
    '    }',
    '  }',
    ']'
  ].join('\n'));
});
