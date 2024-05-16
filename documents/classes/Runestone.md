[@ordjs/runestone](../README.md) / [Exports](../modules.md) / Runestone

# Class: Runestone

## Table of contents

### Constructors

- [constructor](Runestone.md#constructor)

### Properties

- [edicts](Runestone.md#edicts)
- [etching](Runestone.md#etching)
- [mint](Runestone.md#mint)
- [pointer](Runestone.md#pointer)

### Methods

- [encipher](Runestone.md#encipher)
- [free](Runestone.md#free)
- [toJSON](Runestone.md#tojson)
- [decipher](Runestone.md#decipher)

## Constructors

### constructor

• **new Runestone**(`params?`): [`Runestone`](Runestone.md)

#### Parameters

| Name | Type |
| :------ | :------ |
| `params?` | [`RunestoneParams`](../interfaces/RunestoneParams.md) |

#### Returns

[`Runestone`](Runestone.md)

#### Defined in

index.d.ts:248

## Properties

### edicts

• **edicts**: [`Edict`](Edict.md)[]

#### Defined in

index.d.ts:264

___

### etching

• `Optional` **etching**: [`Etching`](Etching.md)

#### Defined in

index.d.ts:267

___

### mint

• `Optional` **mint**: [`RuneId`](RuneId.md)

#### Defined in

index.d.ts:270

___

### pointer

• `Optional` **pointer**: `number`

#### Defined in

index.d.ts:273

## Methods

### encipher

▸ **encipher**(): `string`

#### Returns

`string`

#### Defined in

index.d.ts:252

___

### free

▸ **free**(): `void`

#### Returns

`void`

#### Defined in

index.d.ts:244

___

### toJSON

▸ **toJSON**(): `any`

#### Returns

`any`

#### Defined in

index.d.ts:261

___

### decipher

▸ **decipher**(`transaction`): [`Runestone`](Runestone.md)

#### Parameters

| Name | Type |
| :------ | :------ |
| `transaction` | [`Transaction`](../interfaces/Transaction.md) |

#### Returns

[`Runestone`](Runestone.md)

#### Defined in

index.d.ts:257
