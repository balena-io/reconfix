
const native = require('../native');
const stream = require('stream');
const Promise = require('bluebird');

class BufferStream extends stream.Duplex {
    constructor(options) {
        super(options);
        this.inner = options.inner;
    }

    _read(size) {
        let result = this.inner.read(size);
        this.push(result);
    }

    _write(chunk, encoding, callback) {
        this.inner.write(chunk, encoding);
        callback(null);
    }
}

function bufferRead(stream) {
    return new Promise((resolve, reject) => {
        let buffer = new BufferStream({ 
            inner: new native.BufferStream() 
        });
        stream.on('error', reject);
        stream.on('end', () => resolve(buffer.inner));

        stream.pipe(buffer);
    });
}

function bufferWrite(data, stream) {
    return new Promise((resolve, reject) => {
        stream.on('error', reject);
        stream.on('finish', () => resolve());

        let buffer = new BufferStream({ inner: data });
        buffer.pipe(stream);
    })
}

class Reconfix {
    constructor(options) {
        let readAsync = Promise.promisify(options.read);
        let writeAsync = Promise.promisify(options.write);
        this._inner = new native.Reconfix(
            (partition, path, callback) => {
                readAsync(partition, path).then((disposer) => {
                    return Promise.using(disposer, bufferRead);
                }).asCallback(callback);
            },
            (partition, path, data, callback) => {
                writeAsync(partition, path).then((disposer) => {
                    return Promise.using(disposer, (stream) => bufferWrite(data, stream));
                }).asCallback(callback);
            }
        );
        this._readValues = Promise.promisify(this._inner.readValues);
        this._writeValues= Promise.promisify(this._inner.writeValues);
    }

    loadSchema(json) {
        let input = JSON.stringify(json);
        return this._inner.loadSchema(input);
    }

    readValues() {
        return this._readValues();
    }

    writeValues(json, callback) {
        return this._writeValues(json);
    }
}

module.exports.Reconfix = Reconfix;
