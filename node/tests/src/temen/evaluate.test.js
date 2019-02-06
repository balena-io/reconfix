const rcx = require('reconfix');

test('validate throws if the input is invalid', () => {
    expect(
        () => {
            rcx.evaluate(undefined)
        }
    ).toThrow();
})

test('same object is returned if there is nothing to evaluate', () => {
    data = {
        foo: 'bar',
        bar: 'baz',
        baz: 10
    };

    expect(rcx.evaluate(data)).toEqual(data);
})

test('math expression', () => {
    data = {
        '$$formula': '1+1'
    };

    expect(rcx.evaluate(data)).toEqual(2);
})

test('math expression with references', () => {
    data = {
        x: 10,
        y: 20,
        sum: {
            '$$formula': 'x + y'
        }
    };

    expect(rcx.evaluate(data).sum).toEqual(30);
})
