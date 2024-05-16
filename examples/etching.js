import { SpacedRune, Terms, Etching, Rune } from '../runestone/index.js';

const etching = new Etching({
  spacedRune: SpacedRune.fromString('Z•Z•Z•Z•Z•FEHU•Z•Z•Z•Z•Z'),
  premine: 100n,
  terms: new Terms({ amount: 420n, cap: 2n }),
  divisibility: 2,
  symbol: '?',
  turbo: true,
});
console.log(etching.valueOf());

etching.rune = new Rune(100n);
etching.spacers = 1;
etching.divisibility = 100;
console.dir(etching);
