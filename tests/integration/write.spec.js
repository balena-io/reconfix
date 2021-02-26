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
const Bluebird = require('bluebird');
const tmp = Bluebird.promisifyAll(require('tmp'));
const path = require('path');
const imagefs = require('balena-image-fs');
const fs = require('fs');
const rindle = require('rindle');
const filesystem = require('../../lib/engine/filesystem');
const reconfix = require('../../lib');

const createTemporaryFileFromFile = (file) => {
  return tmp.fileAsync().tap((temporaryFilePath) => {
    const stream = fs.createReadStream(file)
      .pipe(fs.createWriteStream(temporaryFilePath));
    return rindle.wait(stream);
  });
};

ava('should switch an ethernet resin image into a wifi one', (test) => {
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

ava('should switch a wifi resin image into an ethernet one', (test) => {
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

ava('should extend the current values instead of overwriting', (test) => {
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

ava('should be able to modify a fileset', (test) => {
  const fixturesPath = path.join(__dirname, 'fixtures');
  const schema = require(path.join(fixturesPath, 'resinos-v2', 'schema.json'));
  const fixtureImage = path.join(fixturesPath, 'resinos-v2', 'image.img');

  const readFiles = (image) => {
    return imagefs.interact(
      image,
      schema.files.system_connections.location.partition,
      (_fs) => {
        const readFileAsync = Bluebird.promisify(_fs.readFile);
        return Bluebird.props({
          cellular: readFileAsync(path.posix.join(schema.files.system_connections.location.path, 'cellular')).then((b) => {
            return b.toString();
          }),
          ethernet: readFileAsync(path.posix.join(schema.files.system_connections.location.path, 'ethernet')).then((b) => {
            return b.toString();
          }),
          wifi: readFileAsync(path.posix.join(schema.files.system_connections.location.path, 'wifi')).then((b) => {
            return b.toString();
          })
        });
      }
    );
  };

  return createTemporaryFileFromFile(fixtureImage).then((imagePath) => {
    return readFiles(imagePath).then((files) => {
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
    }).then(() => {
      return readFiles(imagePath);
    }).then((files) => {
      test.deepEqual(files, {
        cellular: '[connection]\nname=newcellular',
        ethernet: '[connection]\nname=newethernet',
        wifi: '[connection]\nname=newwifi'
      });
    });
  });
});

ava('should not override custom properties inside a fileset', (test) => {
  const fixturesPath = path.join(__dirname, 'fixtures');
  const schema = require(path.join(fixturesPath, 'resinos-v2', 'schema.json'));
  const fixtureImage = path.join(fixturesPath, 'resinos-v2', 'image.img');

  const readFiles = (image) => {
    return imagefs.interact(
      image,
      schema.files.system_connections.location.partition,
      (_fs) => {
        const readFileAsync = Bluebird.promisify(_fs.readFile);
        return Bluebird.props({
          cellular: readFileAsync(path.posix.join(schema.files.system_connections.location.path, 'cellular')).then((b) => {
            return b.toString();
          }),
          ethernet: readFileAsync(path.posix.join(schema.files.system_connections.location.path, 'ethernet')).then((b) => {
            return b.toString();
          }),
          wifi: readFileAsync(path.posix.join(schema.files.system_connections.location.path, 'wifi')).then((b) => {
            return b.toString();
          })
        });
      }
    );
  };

  return createTemporaryFileFromFile(fixtureImage).then((imagePath) => {
    return imagefs.interact(
      imagePath,
      schema.files.system_connections.location.partition,
      (_fs) => {
        const writeFileAsync = Bluebird.promisify(_fs.writeFile);
        const filePath = path.posix.join(schema.files.system_connections.location.path, 'cellular');
        return writeFileAsync(filePath, '[connection]\nname=cellular\nfoo=bar\nbar=baz');
      }
    )
      .then(() => {
        return reconfix.writeConfiguration(schema, {
          cellularConnectionName: 'newcellular',
          ethernetConnectionName: 'newethernet',
          wifiConnectionName: 'newwifi'
        }, imagePath);
      }).then(() => {
        return readFiles(imagePath);
      }).then((files) => {
        test.deepEqual(files, {
          cellular: '[connection]\nname=newcellular\nfoo=bar\nbar=baz',
          ethernet: '[connection]\nname=newethernet',
          wifi: '[connection]\nname=newwifi'
        });
      });
  });
});
