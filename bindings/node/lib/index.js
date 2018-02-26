
const native = require('../native');
const Promise = require('bluebird');

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

module.exports = Reconfix;
