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

const _ = require('lodash');
const ava = require('ava');
const Bluebird = require('bluebird');
const tmp = Bluebird.promisifyAll(require('tmp'));
const path = require('path');
const fs = require('fs');
const rindle = require('rindle');
const filesystem = require('../../lib/engine/filesystem');
const reconfix = require('../../lib');
const lkl = Bluebird.promisifyAll(require('lkl'));
lkl.fs = Bluebird.promisifyAll(lkl.fs);

const createTemporaryFileFromFile = (file) => {
  return tmp.fileAsync().tap((temporaryFilePath) => {
    const stream = fs.createReadStream(file)
      .pipe(fs.createWriteStream(temporaryFilePath));
    return rindle.wait(stream);
  });
};

ava.test('should switch an ethernet resin image into a wifi one', (test) => {
  const fixturesPath = path.join(__dirname, 'fixtures');

  const files = {
    ethernet: {
      image: path.join(fixturesPath, 'resinos-v1-ethernet', 'image.img'),
      schema: require(path.join(fixturesPath, 'resinos-v1-ethernet', 'schema.json')),
      data: require(path.join(fixturesPath, 'resinos-v1-ethernet', 'data.json'))
    },
    wifi: {
      data: require(path.join(fixturesPath, 'resinos-v1-wifi', 'data.json'))
    }
  };

  return createTemporaryFileFromFile(files.ethernet.image).then((imagePath) => {
    return reconfix.readConfiguration(files.ethernet.schema, imagePath).then((settings) => {
      test.deepEqual(settings, files.ethernet.data);
      return reconfix.writeConfiguration(files.ethernet.schema, files.wifi.data, imagePath);
    }).then(() => {
      return reconfix.readConfiguration(files.ethernet.schema, imagePath);
    }).then((settings) => {
      test.deepEqual(settings, files.wifi.data);
    });
  });
});

ava.test('should switch a wifi resin image into an ethernet one', (test) => {
  const fixturesPath = path.join(__dirname, 'fixtures');

  const files = {
    wifi: {
      image: path.join(fixturesPath, 'resinos-v1-wifi', 'image.img'),
      schema: require(path.join(fixturesPath, 'resinos-v1-wifi', 'schema.json')),
      data: require(path.join(fixturesPath, 'resinos-v1-wifi', 'data.json'))
    },
    ethernet: {
      data: require(path.join(fixturesPath, 'resinos-v1-ethernet', 'data.json'))
    }
  };

  return createTemporaryFileFromFile(files.wifi.image).then((imagePath) => {
    return reconfix.readConfiguration(files.wifi.schema, imagePath).then((settings) => {
      test.deepEqual(settings, files.wifi.data);
      return reconfix.writeConfiguration(files.wifi.schema, files.ethernet.data, imagePath);
    }).then(() => {
      return reconfix.readConfiguration(files.wifi.schema, imagePath);
    }).then((settings) => {
      test.deepEqual(settings, files.ethernet.data);
    });
  });
});

ava.test('should extend the current values instead of overwriting', (test) => {
  const fixturesPath = path.join(__dirname, 'fixtures');
  const schema = require(path.join(fixturesPath, 'resinos-v1-ethernet', 'schema.json'));
  const fixtureImage = path.join(fixturesPath, 'resinos-v1-ethernet', 'image.img');

  /* eslint-disable camelcase */

  return createTemporaryFileFromFile(fixtureImage).then((imagePath) => {
    return filesystem.readImageConfiguration(schema.files, imagePath).then((data) => {
      test.deepEqual(data.config_txt, {
        gpu_mem: 16,
        dtparam: 'spi=on',
        device_tree_overlay: 'w1-gpio-pullup-overlay.dtb'
      });

      return reconfix.writeConfiguration(schema, {
        gpuMem: 64,
        appUpdatePollInterval: '60000',
        networkType: 'ethernet'
      }, imagePath);
    }).then(() => {
      return filesystem.readImageConfiguration(schema.files, imagePath);
    }).then((data) => {
      test.deepEqual(data.config_txt, {
        gpu_mem: 64,
        dtparam: 'spi=on',
        device_tree_overlay: 'w1-gpio-pullup-overlay.dtb'
      });
    });
  });

  /* eslint-enable camelcase */

});

const readFiles = (image, schema) => {
  const disk = new lkl.disk.FileDisk(image);
  const options = {
    filesystem: 'vfat',
    readOnly: true,
    partition: filesystem.getPartitionNumber(schema.files.system_connections.location.partition)
  };
  return Bluebird.using(filesystem.mountPartition(disk, options), (partitionAndMountpoint) => {
    const mpoint = partitionAndMountpoint[1];
    const folder = path.join(mpoint, schema.files.system_connections.location.path);
    const result = _.fromPairs([ 'cellular', 'ethernet', 'wifi' ].map((v) => {
      return [ v, lkl.fs.readFileAsync(path.join(folder, v), 'utf8') ];
    }));
    return Bluebird.props(result);
  });
};

ava.test('should be able to modify a fileset', (test) => {
  const fixturesPath = path.join(__dirname, 'fixtures');
  const schema = require(path.join(fixturesPath, 'resinos-v2', 'schema.json'));
  const fixtureImage = path.join(fixturesPath, 'resinos-v2', 'image.img');

  return createTemporaryFileFromFile(fixtureImage).then((imagePath) => {
    return readFiles(imagePath, schema)
    .then((files) => {
      test.deepEqual(files, {
        cellular: '[connection]\nname=cellular\n',
        ethernet: '[connection]\nname=ethernet\n',
        wifi: '[connection]\nname=wifi\n'
      });

      return reconfix.writeConfiguration(schema, {
        cellularConnectionName: 'newcellular',
        ethernetConnectionName: 'newethernet',
        wifiConnectionName: 'newwifi'
      }, imagePath);
    })
    .then(() => {
      return readFiles(imagePath, schema);
    })
    .then((files) => {
      test.deepEqual(files, {
        cellular: '[connection]\nname=newcellular',
        ethernet: '[connection]\nname=newethernet',
        wifi: '[connection]\nname=newwifi'
      });
    });
  });
});

ava.test('should not override custom properties inside a fileset', (test) => {
  const fixturesPath = path.join(__dirname, 'fixtures');
  const schema = require(path.join(fixturesPath, 'resinos-v2', 'schema.json'));
  const fixtureImage = path.join(fixturesPath, 'resinos-v2', 'image.img');

  return createTemporaryFileFromFile(fixtureImage)
  .then((imagePath) => {
    const disk = new lkl.disk.FileDisk(imagePath);
    const options = {
      filesystem: 'vfat',
      partition: filesystem.getPartitionNumber(schema.files.system_connections.location.partition)
    };
    return Bluebird.using(filesystem.mountPartition(disk, options), (partitionAndMountpoint) => {
      const mpoint = partitionAndMountpoint[1];
      const fpath = path.join(
        mpoint,
        schema.files.system_connections.location.path,
        'cellular'
      );
      return lkl.fs.writeFileAsync(fpath, '[connection]\nname=cellular\nfoo=bar\nbar=baz');
    })
    .then(() => {
      return reconfix.writeConfiguration(schema, {
        cellularConnectionName: 'newcellular',
        ethernetConnectionName: 'newethernet',
        wifiConnectionName: 'newwifi'
      }, imagePath);
    })
    .then(() => {
      return readFiles(imagePath, schema);
    })
    .then((files) => {
      test.deepEqual(files, {
        cellular: '[connection]\nname=newcellular\nfoo=bar\nbar=baz',
        ethernet: '[connection]\nname=newethernet',
        wifi: '[connection]\nname=newwifi'
      });
    });
  });
});
