import { Range, Terms } from '../runestone/index.js';

console.log(JSON.stringify(new Terms()));
const terms = new Terms({
  amount: 420n,
  cap: 2n,
  offset: { start: 1n },
});
console.log(JSON.stringify(terms));

terms.amount = 20202n;
console.log(JSON.stringify(terms));

terms.height = new Range(1n, 2n);
console.log(JSON.stringify(terms));

terms.offset = new Range(1n, 2n);
console.log(JSON.stringify(terms));
