import { Runestone } from '../runestone/index.js';

const fromHexString = (hexString) =>
  Uint8Array.from(hexString.match(/.{1,2}/g).map((byte) => parseInt(byte, 16)));

const toHexString = (bytes) =>
  bytes.reduce((str, byte) => str + byte.toString(16).padStart(2, '0'), '');

const tx = {
  output: [
    {
      script_pubkey: fromHexString(
        '6a5d1f02010480bbb180c5ddf4ede90303a40805b5e9070680809dd085bedd031601'
      ),
      value: 0,
    },
    {
      script_pubkey: fromHexString(
        '5120f74ffbe050dae50a5564d8c9ff57e5bc2fc931225402e4fa751bb21c7da53560'
      ),
      value: 546,
    },
    {
      script_pubkey: fromHexString(
        '0014e08b4212e2a63e1cce78e1ccfbe326a0b8380968'
      ),
      value: 2158,
    },
  ],
  input: [],
};

const runestone = Runestone.decipher(tx);

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
