[@ordjs/runestone](../README.md) / [Exports](../modules.md) / Rune

# Class: Rune

## Table of contents

### Constructors

- [constructor](Rune.md#constructor)

### Properties

- [value](Rune.md#value)

### Methods

- [commitment](Rune.md#commitment)
- [free](Rune.md#free)
- [isReserved](Rune.md#isreserved)
- [toJSON](Rune.md#tojson)
- [toString](Rune.md#tostring)
- [valueOf](Rune.md#valueof)
- [firstRuneHeight](Rune.md#firstruneheight)
- [fromString](Rune.md#fromstring)
- [minimumAtHeight](Rune.md#minimumatheight)
- [reserved](Rune.md#reserved)

## Constructors

### constructor

• **new Rune**(`value`): [`Rune`](Rune.md)

#### Parameters

| Name | Type |
| :------ | :------ |
| `value` | `bigint` |

#### Returns

[`Rune`](Rune.md)

#### Defined in

index.d.ts:139

## Properties

### value

• `Readonly` **value**: `bigint`

#### Defined in

index.d.ts:184

## Methods

### commitment

▸ **commitment**(): `Uint8Array`

#### Returns

`Uint8Array`

#### Defined in

index.d.ts:147

___

### free

▸ **free**(): `void`

#### Returns

`void`

#### Defined in

index.d.ts:135

___

### isReserved

▸ **isReserved**(): `boolean`

#### Returns

`boolean`

#### Defined in

index.d.ts:143

___

### toJSON

▸ **toJSON**(): `any`

#### Returns

`any`

#### Defined in

index.d.ts:159

___

### toString

▸ **toString**(): `string`

#### Returns

`string`

#### Defined in

index.d.ts:151

___

### valueOf

▸ **valueOf**(): `bigint`

#### Returns

`bigint`

#### Defined in

index.d.ts:155

___

### firstRuneHeight

▸ **firstRuneHeight**(`network`): `number`

#### Parameters

| Name | Type |
| :------ | :------ |
| `network` | [`Network`](../modules.md#network) |

#### Returns

`number`

#### Defined in

index.d.ts:164

___

### fromString

▸ **fromString**(`s`): [`Rune`](Rune.md)

#### Parameters

| Name | Type |
| :------ | :------ |
| `s` | `string` |

#### Returns

[`Rune`](Rune.md)

#### Defined in

index.d.ts:181

___

### minimumAtHeight

▸ **minimumAtHeight**(`network`, `height`): [`Rune`](Rune.md)

#### Parameters

| Name | Type |
| :------ | :------ |
| `network` | [`Network`](../modules.md#network) |
| `height` | `number` |

#### Returns

[`Rune`](Rune.md)

#### Defined in

index.d.ts:170

___

### reserved

▸ **reserved**(`block`, `tx`): [`Rune`](Rune.md)

#### Parameters

| Name | Type |
| :------ | :------ |
| `block` | `bigint` |
| `tx` | `number` |

#### Returns

[`Rune`](Rune.md)

#### Defined in

index.d.ts:176
