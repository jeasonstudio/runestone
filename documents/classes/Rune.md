[@ordjs/runestone](../README.md) / [Exports](../modules.md) / Rune

# Class: Rune

## Table of contents

### Constructors

- [constructor](Rune.md#constructor)

### Properties

- [name](Rune.md#name)
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

index.d.ts:195

## Properties

### name

• `Readonly` **name**: `string`

#### Defined in

index.d.ts:232

___

### value

• **value**: `bigint`

#### Defined in

index.d.ts:235

## Methods

### commitment

▸ **commitment**(): `string`

#### Returns

`string`

#### Defined in

index.d.ts:203

___

### free

▸ **free**(): `void`

#### Returns

`void`

#### Defined in

index.d.ts:191

___

### isReserved

▸ **isReserved**(): `boolean`

#### Returns

`boolean`

#### Defined in

index.d.ts:199

___

### toJSON

▸ **toJSON**(): `Object`

* Return copy of self without private attributes.

#### Returns

`Object`

#### Defined in

index.d.ts:186

___

### toString

▸ **toString**(): `string`

Return stringified version of self.

#### Returns

`string`

#### Defined in

index.d.ts:190

___

### valueOf

▸ **valueOf**(): `any`

#### Returns

`any`

#### Defined in

index.d.ts:207

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

index.d.ts:212

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

index.d.ts:229

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

index.d.ts:218

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

index.d.ts:224
