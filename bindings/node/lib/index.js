const native = require('../native');
const stream = require('stream');

class BufferStream extends stream.Duplex {
    constructor(options) {
        super(options);
        this.inner = new native.BufferStream();
    }

    _read(size) {
        this.inner.read(size, this.push);
    }

    _write(chunk, encoding, callback) {
        this.inner.write(chunk, encoding, callback);
    }
}

class Reconfix {
    constructor(options) {
        this._inner = new native.Reconfix(
            (partition, path, callback) => {
                options.read(partition, path, (err, stream) => {
                    let buffer = new BufferStream();
                    stream.on('error', (err) => {
                        callback(err);
                    });
                    stream.on('end', () => {
                        callback(null, buffer.inner);
                    });
                    stream.pipe(buffer);
                });
            },
            (partition, path, data, callback) => {
                options.write(partition, path, (err, stream) => {
                    stream.on('error', (err) => {
                        callback(err);
                    });
                    stream.on('finish', () => {
                        callback();
                    });
                    data.pipe(stream);
                })
            }
        );
    }

    loadSchema(json) {
        return this._inner.loadSchema(json);
    }

    readValues(callback) {
        this._inner.readValues(callback);
    }

    writeValues(json, callback) {
        this._inner.callback(callback);
    }
}

module.exports.Reconfix = Reconfix;
