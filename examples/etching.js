import { SpacedRune, Terms, Etching, Rune } from '../runestone/index.js';

const spacedRune = SpacedRune.fromString('Z•Z•Z•Z•Z•FEHU•Z•Z•Z•Z•Z');
const terms = new Terms({ amount: 420n, cap: 2n });
console.log(spacedRune.rune);
const etching = new Etching({
  rune: spacedRune.rune,
  spacers: spacedRune.spacers,
  premine: 100n,
  terms: new Terms({ amount: 420n, cap: 2n }),
  divisibility: 2,
  symbol: '?',
  turbo: true,
});
console.log(etching.toJSON());
