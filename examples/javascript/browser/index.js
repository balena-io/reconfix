import * as bt from "reconfix";

const initialValue = {
    "version": 1
};
const stringify = (value) => JSON.stringify(value, null, 2);
const parse = (value) => JSON.parse(value);
const $source = document.getElementById('source');
const $result = document.getElementById('result');

$source.value = stringify(initialValue);

const evaluate = () => {
    try {
        const value = parse($source.value);
        const result = value;
        console.log('result', $source.value);
        $result.innerText = stringify(result)
    } catch (error) {
        console.error(error)
    }
};

evaluate();

$source.addEventListener('input', evaluate, false);
