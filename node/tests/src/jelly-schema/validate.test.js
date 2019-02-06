const rcx = require('reconfix');

test('validate throws if the input is invalid', () => {
    js = new rcx.JellySchema({
        type: "string",
        pattern: "^[0-9]+$"
    });

    expect(
        () => {
            js.validate(undefined)
        }
    ).toThrow();
})

test('string validation', () => {
    js = new rcx.JellySchema({
        type: "string",
        pattern: "^[0-9]+$",
        minLength: 3
    });
    expect(js.validate("123")).toBeTruthy();
    expect(js.validate("12")).toBeFalsy();
    expect(js.validate("foo")).toBeFalsy();
});
