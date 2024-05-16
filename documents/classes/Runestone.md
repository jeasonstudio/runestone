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
- [toString](Runestone.md#tostring)
- [valueOf](Runestone.md#valueof)
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

index.d.ts:300

## Properties

### edicts

• **edicts**: [`Edict`](Edict.md)[]

#### Defined in

index.d.ts:316

___

### etching

• `Optional` **etching**: [`Etching`](Etching.md)

#### Defined in

index.d.ts:319

___

### mint

• `Optional` **mint**: [`RuneId`](RuneId.md)

#### Defined in

index.d.ts:322

___

### pointer

• `Optional` **pointer**: `number`

#### Defined in

index.d.ts:325

## Methods

### encipher

▸ **encipher**(): `string`

#### Returns

`string`

#### Defined in

index.d.ts:304

___

### free

▸ **free**(): `void`

#### Returns

`void`

#### Defined in

index.d.ts:296

___

### toJSON

▸ **toJSON**(): `Object`

* Return copy of self without private attributes.

#### Returns

`Object`

#### Defined in

index.d.ts:291

___

### toString

▸ **toString**(): `string`

Return stringified version of self.

#### Returns

`string`

#### Defined in

index.d.ts:295

___

### valueOf

▸ **valueOf**(): `any`

#### Returns

`any`

#### Defined in

index.d.ts:313

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

index.d.ts:309
