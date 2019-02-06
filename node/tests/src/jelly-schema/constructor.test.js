const rcx = require('reconfix');

test('can be instantiated from a string', () => {
    expect(
        new rcx.JellySchema('title: Foo\nversion: 1\n')
    ).toBeDefined();
});

test('can be instantiated from an object', () => {
    expect(
        new rcx.JellySchema({
            title: 'Foo',
            version: 1
        })
    ).toBeDefined();
});

test('throws in case of invalid schema (string)', () => {
    expect(
        () => {
            js = new rcx.JellySchema('foo');
        }
    ).toThrow();
});

test('throws in case of invalid schema (boolean)', () => {
    expect(
        () => {
            js = new rcx.JellySchema(true);
        }
    ).toThrow();
});

test('throws in case of invalid schema (undefined)', () => {
    expect(
        () => {
            js = new rcx.JellySchema(undefined);
        }
    ).toThrow();
});

test('throws in case of invalid schema (null)', () => {
    expect(
        () => {
            js = new rcx.JellySchema(null);
        }
    ).toThrow();
});

test('throws in case of invalid schema (number)', () => {
    expect(
        () => {
            js = new rcx.JellySchema(10);
        }
    ).toThrow();
});
