[@ordjs/runestone](../README.md) / [Exports](../modules.md) / SpacedRune

# Class: SpacedRune

## Table of contents

### Constructors

- [constructor](SpacedRune.md#constructor)

### Properties

- [name](SpacedRune.md#name)
- [rune](SpacedRune.md#rune)
- [runeValue](SpacedRune.md#runevalue)
- [spacers](SpacedRune.md#spacers)

### Methods

- [free](SpacedRune.md#free)
- [toJSON](SpacedRune.md#tojson)
- [toString](SpacedRune.md#tostring)
- [valueOf](SpacedRune.md#valueof)
- [fromString](SpacedRune.md#fromstring)

## Constructors

### constructor

• **new SpacedRune**(`rune`, `spacers`): [`SpacedRune`](SpacedRune.md)

#### Parameters

| Name | Type |
| :------ | :------ |
| `rune` | [`Rune`](Rune.md) |
| `spacers` | `number` |

#### Returns

[`SpacedRune`](SpacedRune.md)

#### Defined in

index.d.ts:343

## Properties

### name

• `Readonly` **name**: `string`

#### Defined in

index.d.ts:355

___

### rune

• **rune**: [`Rune`](Rune.md)

#### Defined in

index.d.ts:358

___

### runeValue

• `Readonly` **runeValue**: `bigint`

#### Defined in

index.d.ts:361

___

### spacers

• **spacers**: `number`

#### Defined in

index.d.ts:364

## Methods

### free

▸ **free**(): `void`

#### Returns

`void`

#### Defined in

index.d.ts:338

___

### toJSON

▸ **toJSON**(): `Object`

* Return copy of self without private attributes.

#### Returns

`Object`

#### Defined in

index.d.ts:333

___

### toString

▸ **toString**(): `string`

Return stringified version of self.

#### Returns

`string`

#### Defined in

index.d.ts:337

___

### valueOf

▸ **valueOf**(): `any`

#### Returns

`any`

#### Defined in

index.d.ts:347

___

### fromString

▸ **fromString**(`s`): [`SpacedRune`](SpacedRune.md)

#### Parameters

| Name | Type |
| :------ | :------ |
| `s` | `string` |

#### Returns

[`SpacedRune`](SpacedRune.md)

#### Defined in

index.d.ts:352
