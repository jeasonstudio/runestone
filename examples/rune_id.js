import { RuneId } from '../runestone/index.js';

const runeId = new RuneId(840000n, 1);
console.log('json:', JSON.stringify(runeId));

runeId.tx = 2;
console.log('json:', JSON.stringify(runeId));

console.log('runeId:', runeId.toString(), runeId.block, runeId.tx);
console.log('delta:', runeId.delta(new RuneId(840001n, 10000)).start);
console.log('next:', runeId.next(1n, 1).toString());

console.log(RuneId.fromString('840000:1').toString());
