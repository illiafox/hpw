'use strict';

// In JavaScript, a variable can be declared after it has been used.
const fn = () => {
    /* eslint-disable */
    d++;
    d *= 34;
    console.log(`hoisted variable: ${d}`);
    /* eslint-enable */
    let d = 12;
};

module.exports = {fn};
