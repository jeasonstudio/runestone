const {
  Edict,
  Etching,
  Range,
  Rune,
  RuneId,
  Runestone,
  SpacedRune,
  Terms,
} = require('../runestone/cjs');

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
};

const runestone = Runestone.decipher(tx);

console.log('runestone etching:', runestone);
console.log(runestone.encipher());

console.log(
  'runestone mint:',
  Runestone.decipher({
    output: [
      {
        script_pubkey:
          '51206da59bfd6b08756375d62e8e38e55db6edaaf20a12b3e6e7f3aa36afcfc3c931',
        value: 0,
      },
      {
        script_pubkey: '6a5d0914f8a83314f0031600',
        value: 0,
      },
    ],
  })
);

console.log(
  'runestone edicts:',
  Runestone.decipher({
    output: [
      {
        script_pubkey: '6a5d0800c0a23301d00f00',
        value: 0,
      },
    ],
  })
);

const spacedRune = SpacedRune.fromString('HI•JEASON');
const etching = new Etching({
  rune: 1n,
  spacers: 2,
  premine: 100n,
  terms: new Terms({ amount: 69n, cap: 420n }),
});
const rs = new Runestone({
  etching,
});
console.log(rs);

const edict = new Edict(new RuneId(840000n, 1), 10000n, 0);
rs.edicts = [edict];
console.log(rs);
console.log(rs.encipher());
