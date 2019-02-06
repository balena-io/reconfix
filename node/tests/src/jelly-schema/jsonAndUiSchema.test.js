const rcx = require('reconfix');

test('string validation', () => {
    js = new rcx.JellySchema({
        title: "Foo",
        properties: [
            {
                name: {
                    type: "string",
                    title: "Name",
                    description: "Your full name"
                }
            }
        ]
    });

    expected = {
        jsonSchema: {
            "$$order": [
                "name"
            ],
            "$$version": 1,
            "$schema": "http://json-schema.org/draft-04/schema#",
            additionalProperties: false,
            properties: {
                name: {
                    title: "Name",
                    type: "string"
                }
            },
            required: [
                "name"
            ],
            title: "Foo",
            type: "object"
        },
        uiSchema: {
            name: {
                "ui:description": "Your full name"
            },
            "ui:order": [
                "name"
            ]
        }
    };

    expect(js.jsonAndUiSchema()).toEqual(expected);
});
