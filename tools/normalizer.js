"use strict";
//import {expect} from "chai";

const average = array => {
    return array.reduce((a, b) => a + b, 0) / array.length;
};

const closestIn = array => value => {
    let result = array[0];
    for (let v of array) {
        if (Math.abs(v - value) < Math.abs(result - value)) {
            result = v;
        }
    }
    return result;
};

const normalize = array => {
    let norms = [];
    let a = [...array];
    a.sort();
    for (let value of a) {
        let i = 0;
        let values;
        do {
            if (i >= norms.length) {
                norms.push([]);
            }
            values = norms[i];
            i++;
        } while (!(values.length === 0 || Math.abs(average(values) - value) < value * 0.15));

        values.push(value);
    }
    norms = norms.map(average);
    return array.map(closestIn(norms)).map(Math.round);
};

let numbers = process.argv[2].split(",").map(s=>parseInt(s.trim()));

console.log(JSON.stringify(normalize(numbers)));
/*
describe('Normalize', () => {
    it('normalize 2', () => {
        expect(normalize([10, 11, 12])).to.deep.equal([11, 11, 11]);
    });
    it('normalize 3', () => {
        expect(normalize([10, 11, 12, 40, 41, 42])).to.deep.equal([11, 11, 11, 41, 41, 41]);
    });
});
*/
