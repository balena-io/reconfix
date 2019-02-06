const rcx = require('reconfix');

test('no errors after instantiation', () => {
    expect(
        new rcx.JellySchema({
            title: 'Foo',
            version: 1
        }).errors()
    ).toEqual([]);
});

test('errors are cleared if validation succeeds', () => {
    js = new rcx.JellySchema({
        type: 'integer'
    });
    expect(js.validate('foo')).toBeFalsy();
    expect(js.errors().length).toBeGreaterThan(0);
    expect(js.validate(10)).toBeTruthy();
    expect(js.errors().length).toBe(0);
});

test('errors are cleared if validation fails', () => {
    js = new rcx.JellySchema({
        type: 'integer'
    });
    expect(js.validate('foo')).toBeFalsy();
    errors_count = js.errors().length;
    expect(errors_count).toBeGreaterThan(0);
    expect(js.validate('foo')).toBeFalsy();
    expect(errors_count).toBe(js.errors().length);
});

test('errors are cleared if validation throws', () => {
    js = new rcx.JellySchema({
        type: 'integer'
    });

    expect(js.validate('foo')).toBeFalsy();
    expect(js.errors().length).toBeGreaterThan(0);

    try {
        js.validate(undefined);
        expect('must throw').toEqual('did not');
    } catch (e) {
        expect(e).toBeDefined();
    }

    expect(js.errors().length).toBe(0);
});
