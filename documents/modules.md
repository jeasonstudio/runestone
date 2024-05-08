[@ordjs/runestone](README.md) / Exports

# @ordjs/runestone

## Table of contents

### Classes

- [Edict](classes/Edict.md)
- [Etching](classes/Etching.md)
- [Range](classes/Range.md)
- [Rune](classes/Rune.md)
- [RuneId](classes/RuneId.md)
- [Runestone](classes/Runestone.md)
- [SpacedRune](classes/SpacedRune.md)
- [Terms](classes/Terms.md)

### Interfaces

- [OutPoint](interfaces/OutPoint.md)
- [Transaction](interfaces/Transaction.md)
- [TxInput](interfaces/TxInput.md)
- [TxOutput](interfaces/TxOutput.md)
- [Witness](interfaces/Witness.md)

### Type Aliases

- [Network](modules.md#network)

### Functions

- [decodeHex](modules.md#decodehex)
- [encodeHex](modules.md#encodehex)

## Type Aliases

### Network

Ƭ **Network**: ``"main"`` \| ``"test"`` \| ``"signet"`` \| ``"regtest"``

#### Defined in

index.d.ts:28

## Functions

### decodeHex

▸ **decodeHex**(`s`): `Uint8Array`

#### Parameters

| Name | Type |
| :------ | :------ |
| `s` | `string` |

#### Returns

`Uint8Array`

#### Defined in

index.d.ts:7

___

### encodeHex

▸ **encodeHex**(`bytes`): `string`

#### Parameters

| Name | Type |
| :------ | :------ |
| `bytes` | `Uint8Array` |

#### Returns

`string`

#### Defined in

index.d.ts:12
