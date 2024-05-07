import { Range, Terms } from '../runestone/index.js';

const terms = new Terms(420n, 2n);
console.log(JSON.stringify(terms));

terms.height = new Range(1n, 2n);
console.log(JSON.stringify(terms));

terms.offset = new Range(1n, 2n);
console.log(JSON.stringify(terms));