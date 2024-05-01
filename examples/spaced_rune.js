import { SpacedRune, Rune } from '../runestone/index.js';

const rune = new Rune(67090369340599840949n);
const spacedRune = new SpacedRune(rune, 7967);

console.log('spaced rune:', spacedRune.toString());
console.log('spaced rune:', JSON.stringify(spacedRune));
console.log(
  'rune value:',
  SpacedRune.fromString('Z•Z•Z•Z•Z•FEHU•Z•Z•Z•Z•Z').rune.value
);
console.log(
  'rune spacers:',
  SpacedRune.fromString('Z•Z•Z•Z•Z•FEHU•Z•Z•Z•Z•Z').spacers
);
