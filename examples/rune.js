import { Rune } from '../runestone/runestone.js';

const rune = new Rune(67090369340599840949n);

console.log('value:', rune.value);
console.log('isReserved:', rune.isReserved());
console.log('name:', rune.toString());
console.log('commitment:', rune.commitment());

console.log(Rune.firstRuneHeight('main'));
console.log(Rune.fromString('ZZZZZFEHUZZZZZ').value);
