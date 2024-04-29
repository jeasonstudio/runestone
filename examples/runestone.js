import { Runestone } from '../runestone/index.js';
const fromHexString = (hexString) =>
  Uint8Array.from(hexString.match(/.{1,2}/g).map((byte) => parseInt(byte, 16)));

const toHexString = (bytes) =>
  bytes.reduce((str, byte) => str + byte.toString(16).padStart(2, '0'), '');

const runestone = Runestone.decipher(
  fromHexString(
    '6a5d21020704b5e1d8e1c8eeb788a30705a02d039f3e01020680dc9afd2808c7e8430a64'
  )
);

console.log('edicts:', runestone.edicts);
const etching = runestone.etching;
if (etching) {
  console.log('divisibility:', etching.divisibility);
  console.log('premine:', etching.premine);
  console.log('rune:', etching.rune.value);
  console.log('spacers:', etching.spacers);
  console.log('symbol:', etching.symbol);
  console.log('turbo:', etching.turbo);
  console.log('terms.amount:', etching.terms?.amount);
  console.log('terms.cap:', etching.terms?.cap);
  console.log(
    'terms.height:',
    `(${etching.terms?.height?.start}, ${etching.terms?.height?.end})`
  );
  console.log(
    'terms.offset:',
    `(${etching.terms?.offset?.start}, ${etching.terms?.offset?.end})`
  );
}
console.log('mint:', runestone.mint?.toString());
console.log('pointer:', runestone.pointer);

console.log(toHexString(runestone.encipher()));