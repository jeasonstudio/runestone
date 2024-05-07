import { SpacedRune, Terms, Etching } from '../runestone/index.js';

const spacedRune = SpacedRune.fromString('Z•Z•Z•Z•Z•FEHU•Z•Z•Z•Z•Z');
const etching = new Etching(spacedRune);
console.log(JSON.stringify(etching));

etching.terms = new Terms(420n, 2n);
etching.divisibility = 2;
etching.premine = 100n;
etching.symbol = '?';
etching.turbo = false;
console.log(JSON.stringify(etching));
