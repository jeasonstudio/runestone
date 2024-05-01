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
- [valueOf](Runestone.md#valueof)
- [decipher](Runestone.md#decipher)

## Constructors

### constructor

• **new Runestone**(`edicts`, `etching?`, `mint?`, `pointer?`): [`Runestone`](Runestone.md)

#### Parameters

| Name | Type |
| :------ | :------ |
| `edicts` | [`Edict`](Edict.md)[] |
| `etching?` | [`Etching`](Etching.md) |
| `mint?` | [`RuneId`](RuneId.md) |
| `pointer?` | `number` |

#### Returns

[`Runestone`](Runestone.md)

#### Defined in

index.d.ts:221

## Properties

### edicts

• **edicts**: [`Edict`](Edict.md)[]

#### Defined in

index.d.ts:38

index.d.ts:241

___

### etching

• **etching**: ``null`` \| [`Etching`](Etching.md)

#### Defined in

index.d.ts:39

index.d.ts:244

___

### mint

• **mint**: ``null`` \| [`RuneId`](RuneId.md)

#### Defined in

index.d.ts:40

index.d.ts:247

___

### pointer

• **pointer**: ``null`` \| `number`

#### Defined in

index.d.ts:41

index.d.ts:250

## Methods

### encipher

▸ **encipher**(): `Uint8Array`

#### Returns

`Uint8Array`

#### Defined in

index.d.ts:225

___

### free

▸ **free**(): `void`

#### Returns

`void`

#### Defined in

index.d.ts:214

___

### toJSON

▸ **toJSON**(): `any`

#### Returns

`any`

#### Defined in

index.d.ts:234

___

### valueOf

▸ **valueOf**(): `any`

#### Returns

`any`

#### Defined in

index.d.ts:238

___

### decipher

▸ **decipher**(`tx`): [`Runestone`](Runestone.md)

#### Parameters

| Name | Type |
| :------ | :------ |
| `tx` | `any` |

#### Returns

[`Runestone`](Runestone.md)

#### Defined in

index.d.ts:230
