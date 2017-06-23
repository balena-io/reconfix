'use strict';

const Bluebird = require('bluebird');
const filedisk = require('file-disk');
const fs = require('fs');
const rindle = require('rindle');
const tmp = Bluebird.promisifyAll(require('tmp'));

const createTemporaryFileFromFile = (file) => {
  return tmp.fileAsync()
  .tap((temporaryFilePath) => {
    const input = fs.createReadStream(file);
    const output = fs.createWriteStream(temporaryFilePath);
    return rindle.wait(input.pipe(output));
  });
};

const testWithPath = (path_, fn) => {
  return createTemporaryFileFromFile(path_)
  .then((imagePath) => {
    return fn(imagePath);
  });
};

const testWithDisk = (path_, fn) => {
  return Bluebird.using(filedisk.openFile(path_, 'r'), (fd) => {
    const disk = new filedisk.FileDisk(fd, true, true);
    return fn(disk);
  });
};

exports.testBoth = (path_, fn) => {
  return testWithPath(path_, fn)
  .then(() => {
    return testWithDisk(path_, fn);
  });
};
