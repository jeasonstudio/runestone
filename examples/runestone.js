import {
  Etching,
  Rune,
  Runestone,
  SpacedRune,
  Terms,
} from '../runestone/index.js';

const tx = {
  output: [
    {
      script_pubkey:
        '51203b8b3ab1453eb47e2d4903b963776680e30863df3625d3e74292338ae7928da1',
      value: 1797928002,
    },
    {
      script_pubkey:
        '6a5d21020704b5e1d8e1c8eeb788a30705a02d039f3e01020680dc9afd2808c7e8430a64',
      value: 0,
    },
  ],
  // input: [],
};

const runestone = Runestone.decipher(tx);

console.log('runestone etching:', JSON.stringify(runestone, null, 2));
console.log(runestone.encipher());

console.log(
  'runestone mint:',
  JSON.stringify(
    Runestone.decipher({
      output: [
        {
          script_pubkey: '51206da59bfd6b08756375d62e8e38e55db6edaaf20a12b3e6e7f3aa36afcfc3c931',
          value: 0,
        },
        {
          script_pubkey: '6a5d0914f8a83314f0031600',
          value: 0,
        },
      ],
    }),
    null,
    2
  )
);

console.log(
  'runestone edicts:',
  JSON.stringify(
    Runestone.decipher({
      output: [
        {
          script_pubkey: '6a5d0800c0a23301d00f00',
          value: 0,
        },
      ],
    }),
    null,
    2
  )
);

// const spacedRune = SpacedRune.fromString('Z•Z•Z•Z•Z•FEHU•Z•Z•Z•Z•Z');
// const terms = new Terms({
//   amount: 420n,
//   cap: 69n,
//   offset: {
//     end: 9001n,
//   },
// });
// Terms.fromJSON({
//   amount: 420n,
//   cap: 69n,
//   offset: {
//     end: 9001n,
//   },
// });

// const etching = new Etching({
//   rune: spacedRune.rune,
//   spacers: spacedRune.spacers,
//   divisibility: 2,
//   premine: 0,
//   symbol: '',
//   terms,
//   turbo: true,
// });
// console.log(etching);
