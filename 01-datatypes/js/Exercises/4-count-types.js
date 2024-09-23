'use strict';

const countTypesInArray = (arr) => {
    const types = {};
    for (const field of arr) {
        types[typeof field] = ++types[typeof field] || 1;
    }
    return types;
};

module.exports = {countTypesInArray};