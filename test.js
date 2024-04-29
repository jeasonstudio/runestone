const pkg = require('./pkg/ordjsordinals');

// const runeId = new pkg.RuneId(840000n, 1);
// console.log(runeId.block, runeId.tx, runeId.toString());

// console.log(runeId.delta(new pkg.RuneId(840001n, 10000)));
// console.log(runeId.next(1n, 1n));

// // const rune = pkg.create_rune(18278n);
// // console.log(rune);

// // const spacedRune = pkg.create_spaced_rune(rune, 0b111);
// // console.log(spacedRune);

// const spacedRune = new pkg.SpacedRune(18278n, 0b111);
// console.log(
//   spacedRune,
//   spacedRune.rune,
//   spacedRune.spacers,
//   spacedRune.toString()
// );

console.log(pkg.SpacedRune.fromString('Z•Z•Z•Z•Z•FEHU•Z•Z•Z•Z•Z').spacers);

console.log(pkg.SpacedRune.fromString('Z•Z•Z•Z•Z•FEHU•Z•Z•Z•Z•Z').rune.value);
console.log(
  new pkg.SpacedRune(new pkg.Rune(67090369340599840949n), 7967).toString()
);

// const fromHexString = (hexString) =>
//   Uint8Array.from(hexString.match(/.{1,2}/g).map((byte) => parseInt(byte, 16)));

// const toHexString = (bytes) =>
//   bytes.reduce((str, byte) => str + byte.toString(16).padStart(2, '0'), '');

// const res = pkg.Runestone.decipher(
//   fromHexString(
//     '6a5d0614c0a2331401'
//   )
// );
// console.log(res);

console.log(new pkg.Rune(100n).value);
