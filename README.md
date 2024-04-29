<div align="center"><a name="readme-top"></a>

<h1>@ordjs/runestone</h1>

[Ordinals](https://ordinals.com/) runestone javascript implementation.

This implementation is based on ord `0.18.3` (ordinals runestone `0.0.8`)

[![NPM version][npm-image]][npm-url]
[![NPM downloads][download-image]][download-url]
[![MIT License][license-shield]][license-url]

[![Contributors][contributors-shield]][contributors-url]
[![Issues][issues-shield]][issues-url]
[![Stargazers][stars-shield]][stars-url]
[![Follow Twitter][twitter-image]][twitter-url]

[Report Bug](https://github.com/jeasonstudio/runestone/issues/new) · [Pull Request](https://github.com/jeasonstudio/runestone/compare)

![](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/rainbow.png)

[npm-image]: https://img.shields.io/npm/v/@ordjs/runestone?style=for-the-badge
[npm-url]: http://npmjs.org/package/@ordjs/runestone
[download-image]: https://img.shields.io/npm/dm/@ordjs/runestone.svg?style=for-the-badge
[download-url]: https://npmjs.org/package/@ordjs/runestone

[codecov-image]: https://img.shields.io/codecov/c/github/jeasonstudio/runestone/master.svg?style=for-the-badge
[codecov-url]: https://codecov.io/gh/jeasonstudio/runestone/branch/master
[license-shield]: https://img.shields.io/github/license/jeasonstudio/runestone.svg?style=for-the-badge
[license-url]: https://github.com/jeasonstudio/runestone/blob/master/LICENSE

[contributors-shield]: https://img.shields.io/github/contributors/jeasonstudio/runestone.svg?style=for-the-badge
[contributors-url]: https://github.com/jeasonstudio/runestone/graphs/contributors
[stars-shield]: https://img.shields.io/github/stars/jeasonstudio/runestone.svg?style=for-the-badge
[stars-url]: https://github.com/jeasonstudio/runestone/stargazers
[issues-shield]: https://img.shields.io/github/issues/jeasonstudio/runestone.svg?style=for-the-badge
[issues-url]: https://github.com/jeasonstudio/runestone/issues
[twitter-image]: https://img.shields.io/twitter/follow/jeasonstudio?style=for-the-badge&logo=x
[twitter-url]: https://twitter.com/jeasonstudio

</div>

## Installation

```bash
$ npm install @ordjs/runestone
```

> It will be `pnpm/yarn add @ordjs/runestone` if you use pnpm or yarn.

## Usage

### Runestone

```javascript
import { Runestone } from '@ordjs/runestone';
const fromHexString = (hexString) =>
  Uint8Array.from(hexString.match(/.{1,2}/g).map((byte) => parseInt(byte, 16)));

const toHexString = (bytes) =>
  bytes.reduce((str, byte) => str + byte.toString(16).padStart(2, '0'), '');

// decipher
// See https://mempool.space/tx/2bb85f4b004be6da54f766c17c1e855187327112c231ef2ff35ebad0ea67c69e
const runestone = Runestone.decipher(
  fromHexString(
    // OP_RETURN OP_PUSHNUM_13 ...
    '6a5d21020704b5e1d8e1c8eeb788a30705a02d039f3e01020680dc9afd2808c7e8430a64'
  )
);

console.log('edicts:', runestone.edicts); // []
const etching = runestone.etching;
if (etching) {
  console.log('divisibility:', etching.divisibility); // 2
  console.log('premine:', etching.premine); // 11000000000n
  console.log('rune:', etching.rune.value); // 67090369340599840949n
  console.log('spacers:', etching.spacers); // 7967
  console.log('symbol:', etching.symbol); // ᚠ
  console.log('turbo:', etching.turbo); // true
  console.log('terms.amount:', etching.terms?.amount); // 100n
  console.log('terms.cap:', etching.terms?.cap); // 1111111n
}

// encipher
console.log(toHexString(runestone.encipher()));
// 6a5d21020704b5e1d8e1c8eeb788a3070102039f3e05a02d0680dc9afd280a6408c7e843
```

### Rune ID

```javascript
import { RuneId } from '@ordjs/runestone';

const runeId = new RuneId(840000n, 1);

console.log('runeId:', runeId.toString(), runeId.block, runeId.tx); // 840000:1 840000n 1
console.log('delta:', runeId.delta(new RuneId(840001n, 10000)).start); // 1n
console.log('next:', runeId.next(1n, 1).toString()); // 840001:1
console.log(RuneId.fromString('840000:1').toString()); // 840000:1
```

### Rune

```javascript
import { Rune } from '@ordjs/runestone';

const rune = new Rune(67090369340599840949n);

console.log('value:', rune.value); // 67090369340599840949n
console.log('isReserved:', rune.isReserved()); // false
console.log('name:', rune.toString()); // ZZZZZFEHUZZZZZ
console.log('commitment:', rune.commitment());
// Uint8Array(9) [181,  48, 54, 140, 116, 223, 16, 163, 3]

console.log(Rune.firstRuneHeight('main')); // 840000
console.log(Rune.fromString('ZZZZZFEHUZZZZZ').value); // 67090369340599840949n
```

### Spaced Rune

```javascript
import { SpacedRune, Rune } from '@ordjs/runestone';

const rune = new Rune(67090369340599840949n);
const spacedRune = new SpacedRune(rune, 7967);

console.log('spaced rune:', spacedRune.toString()); // Z•Z•Z•Z•Z•FEHU•Z•Z•Z•Z•Z
console.log(
  'rune value:',
  SpacedRune.fromString('Z•Z•Z•Z•Z•FEHU•Z•Z•Z•Z•Z').rune.value // 67090369340599840949n
);
console.log(
  'rune spacers:',
  SpacedRune.fromString('Z•Z•Z•Z•Z•FEHU•Z•Z•Z•Z•Z').spacers // 7967
);
```

## Use in Browser

```html
<script type="module">
  import { Rune } from 'https://esm.sh/@ordjs/runestone@latest/bundle';
  const rune = new Rune(1n);
  console.log(rune.toString());
</script>
```

## License

[MIT](./LICENSE)