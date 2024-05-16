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

index.d.ts:159

## Properties

### name

• `Readonly` **name**: `string`

#### Defined in

index.d.ts:196

___

### value

• **value**: `bigint`

#### Defined in

index.d.ts:199

## Methods

### commitment

▸ **commitment**(): `string`

#### Returns

`string`

#### Defined in

index.d.ts:167

___

### free

▸ **free**(): `void`

#### Returns

`void`

#### Defined in

index.d.ts:155

___

### isReserved

▸ **isReserved**(): `boolean`

#### Returns

`boolean`

#### Defined in

index.d.ts:163

___

### toJSON

▸ **toJSON**(): `any`

#### Returns

`any`

#### Defined in

index.d.ts:171

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

index.d.ts:176

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

index.d.ts:193

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

index.d.ts:182

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

index.d.ts:188
