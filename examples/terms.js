import { Range, Terms } from '../runestone/index.js';

console.log(new Terms().valueOf());
const terms = new Terms({
  amount: 420n,
  cap: 2n,
  offset: { start: 1n },
});
console.log(terms.valueOf());

terms.amount = 20202n;
console.log(terms.valueOf());

terms.height = new Range(1n, 2n);
console.log(terms.valueOf());

terms.offset = new Range(1n, 2n);
console.log(terms.valueOf());
