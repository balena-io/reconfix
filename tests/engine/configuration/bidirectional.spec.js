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
const path = require('path');
const configuration = require('../../../lib/engine/configuration');

const testFixture = (name) => {
  const fixturePath = path.join(__dirname, 'fixtures', name);
  const files = {
    dry: require(path.join(fixturePath, 'dry.json')),
    wet: require(path.join(fixturePath, 'wet.json')),
    wetExtra: require(path.join(fixturePath, 'wet-extra.json')),
    schema: require(path.join(fixturePath, 'schema.json'))
  };

  ava(`(${name}) should generate configuration`, (test) => {
    test.deepEqual(configuration.generate(files.schema, files.dry), files.wet);
  });

  ava(`(${name}) should extract settings`, (test) => {
    test.deepEqual(configuration.extract(files.schema, files.wet), files.dry);
  });

  ava(`(${name}) should ignore extra settings when extracting`, (test) => {
    test.deepEqual(configuration.extract(files.schema, files.wetExtra), files.dry);
  });
};

testFixture('resinos-v1');

ava('.generate() should preserve custom defaults values', (test) => {

  /* eslint-disable camelcase */

  test.deepEqual(configuration.generate([
    {
      template: {
        gpu_mem_1024: '{{gpuMem1024}}'
      },
      domain: [
        [ 'config_txt', 'gpu_mem_1024' ]
      ]
    }
  ], {
    gpuMem1024: 64
  }, {
    defaults: {
      config_txt: {
        gpu_mem_1024: 32,
        foo: 'bar',
        bar: 'baz'
      }
    }
  }), {
    config_txt: {
      gpu_mem_1024: 64,
      foo: 'bar',
      bar: 'baz'
    }
  });

  /* eslint-enable camelcase */

});

ava('.generate() should override custom default values in choices', (test) => {

  /* eslint-disable camelcase */

  test.deepEqual(configuration.generate([
    {
      property: [ 'network', 'type' ],
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
              name: '{{network.ssid}}',
              passphrase: '{{network.key}}',
              nameservers: '8.8.8.8,8.8.4.4'
            }
          }
        }
      ]
    }
  ], {
    network: {
      type: 'ethernet'
    }
  }, {
    defaults: {
      network_config: {
        service_home_ethernet: {
          type: 'ethernet',
          nameservers: '8.8.8.8,8.8.4.4'
        },
        service_home_wifi: {
          hidden: true,
          type: 'wifi',
          name: 'resin',
          passphrase: 'secret',
          nameservers: '8.8.8.8,8.8.4.4'
        }
      }
    }
  }), {
    network_config: {
      service_home_ethernet: {
        type: 'ethernet',
        nameservers: '8.8.8.8,8.8.4.4'
      }
    }
  });

  /* eslint-enable camelcase */

});
