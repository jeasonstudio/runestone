<div align="center"><a name="readme-top"></a>

<h1>@ordjs/runestone</h1>

[Ordinals](https://ordinals.com/) runestone javascript implementation.

This implementation is based on ord `0.18.3` (ordinals runestone `0.0.8`)

[![NPM version][npm-image]][npm-url]
[![MIT License][license-shield]][license-url]
[![Stargazers][stars-shield]][stars-url]

[![Contributors][contributors-shield]][contributors-url]
[![Issues][issues-shield]][issues-url]
[![Follow Twitter][twitter-image]][twitter-url]

[Report Bug](https://github.com/jeasonstudio/runestone/issues/new) · [Pull Request](https://github.com/jeasonstudio/runestone/compare)

![](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/rainbow.png)

[npm-image]: https://img.shields.io/npm/v/@ordjs/runestone?style=for-the-badge
[npm-url]: http://npmjs.org/package/@ordjs/runestone

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

Rune protocol messages, called runestones, are stored in Bitcoin transaction outputs.

#### Decipher Runestones

```typescript
import { Runestone, Transaction } from '@ordjs/runestone';

// See https://mempool.space/tx/2bb85f4b004be6da54f766c17c1e855187327112c231ef2ff35ebad0ea67c69e
const tx: Transaction = {
  output: [{
    // // OP_RETURN OP_PUSHNUM_13 ...
    script_pubkey: '6a5d21020704b5e1d8e1c8eeb788a30705a02d039f3e01020680dc9afd2808c7e8430a64',
    value: 0,
  }],
};

const runestone = Runestone.decipher(tx);
// runestone.divisibility => 2
// runestone.premine => 11000000000
// runestone.symbol => ᚠ
// runestone.terms.amount => 100
```

#### Encipher Runestones

To deploy a new rune ticker, this will require a commitment in an input script.

```typescript
import { Runestone, Etching, SpacedRune, Terms } from '@ordjs/runestone';

const etching = new Etching(SpacedRune.fromString('HI•JEASON'));
etching.terms = new Terms(69n, 420n);
etching.divisibility = 0;
etching.premine = 0n;
etching.symbol = '$';

const runestone = new Runestone();
runestone.etching = etching;

console.log(runestone.encipher());
// 6a5d16020704b7fcb396fa0101000302052406000a4508a403
// send runestone.encipher() to the blockchain
```

To mint `UNCOMMON•GOODS`:

```typescript
import { Runestone, RuneId } from '@ordjs/runestone';

const runestone = new Runestone();
runestone.mint = new RuneId(1n, 0);

console.log(runestone.encipher());
// 6a5d0414011400
// send runestone.encipher() to the blockchain
```

Transfer 10 `UNCOMMON•GOODS` to output 1:

```typescript
import { Runestone, Edict, RuneId } from '@ordjs/runestone';

const edict = new Edict(new RuneId(1n, 0), 10n, 1);
const runestone = new Runestone();
runestone.edicts = [edict];

console.log(runestone.encipher());
// 6a5d050001000a01
// send runestone.encipher() to the blockchain
```

## Use in Browser

We provide the output format of ESM bundles for easy use in browsers directly:

```html
<script type="module">
  import { Runestone } from 'https://esm.sh/@ordjs/runestone/bundle';

  const rs = Runestone.decipher({...});
  console.log(JSON.stringify(rs));
</script>
```

For more usage, please refer to the [examples](/examples/) directory.

## License

[MIT](./LICENSE)