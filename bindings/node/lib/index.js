
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
        const buffer = new BufferStream({ 
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
        this._inner = new native.Reconfix(
            (partition, path, callback) => {
                options.read(partition, path)
                    .asCallback(callback);
            },
            (partition, path, data, callback) => {
                options.write(partition, path, data)
                    .asCallback(callback);
            }
        );
    }

    loadSchema(json) {
        const input = JSON.stringify(json);
        return this._inner.loadSchema(input);
    }

    readValues() {
        const readPromise = Promise.promisify(this._inner.readValues);
        return readPromise.bind(this._inner)();
    }

    writeValues(json) {
        const writePromise = Promise.promisify(this._inner.writeValues);
        return writePromise.bind(this._inner)(json);
    }
}

module.exports.Reconfix = Reconfix;
