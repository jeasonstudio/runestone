[@ordjs/runestone](../README.md) / [Exports](../modules.md) / Etching

# Class: Etching

## Table of contents

### Constructors

- [constructor](Etching.md#constructor)

### Properties

- [divisibility](Etching.md#divisibility)
- [premine](Etching.md#premine)
- [rune](Etching.md#rune)
- [spacers](Etching.md#spacers)
- [symbol](Etching.md#symbol)
- [terms](Etching.md#terms)
- [turbo](Etching.md#turbo)

### Methods

- [free](Etching.md#free)
- [supply](Etching.md#supply)
- [toJSON](Etching.md#tojson)
- [toString](Etching.md#tostring)
- [valueOf](Etching.md#valueof)

## Constructors

### constructor

• **new Etching**(`params?`): [`Etching`](Etching.md)

#### Parameters

| Name | Type |
| :------ | :------ |
| `params?` | [`EtchingParams`](../interfaces/EtchingParams.md) |

#### Returns

[`Etching`](Etching.md)

#### Defined in

index.d.ts:121

## Properties

### divisibility

• `Optional` **divisibility**: `number`

#### Defined in

index.d.ts:132

___

### premine

• `Optional` **premine**: `bigint`

#### Defined in

index.d.ts:135

___

### rune

• `Optional` **rune**: [`Rune`](Rune.md)

#### Defined in

index.d.ts:138

___

### spacers

• `Optional` **spacers**: `number`

#### Defined in

index.d.ts:141

___

### symbol

• `Optional` **symbol**: `string`

#### Defined in

index.d.ts:144

___

### terms

• `Optional` **terms**: [`Terms`](Terms.md)

#### Defined in

index.d.ts:147

___

### turbo

• **turbo**: `boolean`

#### Defined in

index.d.ts:150

## Methods

### free

▸ **free**(): `void`

#### Returns

`void`

#### Defined in

index.d.ts:117

___

### supply

▸ **supply**(): `undefined` \| `bigint`

#### Returns

`undefined` \| `bigint`

#### Defined in

index.d.ts:125

___

### toJSON

▸ **toJSON**(): `Object`

* Return copy of self without private attributes.

#### Returns

`Object`

#### Defined in

index.d.ts:112

___

### toString

▸ **toString**(): `string`

Return stringified version of self.

#### Returns

`string`

#### Defined in

index.d.ts:116

___

### valueOf

▸ **valueOf**(): `any`

#### Returns

`any`

#### Defined in

index.d.ts:129
