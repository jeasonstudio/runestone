[@ordjs/runestone](../README.md) / [Exports](../modules.md) / RuneId

# Class: RuneId

## Table of contents

### Constructors

- [constructor](RuneId.md#constructor)

### Properties

- [block](RuneId.md#block)
- [tx](RuneId.md#tx)

### Methods

- [delta](RuneId.md#delta)
- [free](RuneId.md#free)
- [next](RuneId.md#next)
- [toJSON](RuneId.md#tojson)
- [toString](RuneId.md#tostring)
- [valueOf](RuneId.md#valueof)
- [fromString](RuneId.md#fromstring)

## Constructors

### constructor

• **new RuneId**(`block`, `tx`): [`RuneId`](RuneId.md)

#### Parameters

| Name | Type |
| :------ | :------ |
| `block` | `bigint` |
| `tx` | `number` |

#### Returns

[`RuneId`](RuneId.md)

#### Defined in

index.d.ts:253

## Properties

### block

• **block**: `bigint`

#### Defined in

index.d.ts:280

___

### tx

• **tx**: `number`

#### Defined in

index.d.ts:283

## Methods

### delta

▸ **delta**(`next`): `undefined` \| [`Range`](Range.md)

#### Parameters

| Name | Type |
| :------ | :------ |
| `next` | [`RuneId`](RuneId.md) |

#### Returns

`undefined` \| [`Range`](Range.md)

#### Defined in

index.d.ts:258

___

### free

▸ **free**(): `void`

#### Returns

`void`

#### Defined in

index.d.ts:248

___

### next

▸ **next**(`block`, `tx`): `undefined` \| [`RuneId`](RuneId.md)

#### Parameters

| Name | Type |
| :------ | :------ |
| `block` | `bigint` |
| `tx` | `number` |

#### Returns

`undefined` \| [`RuneId`](RuneId.md)

#### Defined in

index.d.ts:264

___

### toJSON

▸ **toJSON**(): `Object`

* Return copy of self without private attributes.

#### Returns

`Object`

#### Defined in

index.d.ts:243

___

### toString

▸ **toString**(): `string`

Return stringified version of self.

#### Returns

`string`

#### Defined in

index.d.ts:247

▸ **toString**(): `string`

#### Returns

`string`

#### Defined in

index.d.ts:268

___

### valueOf

▸ **valueOf**(): `any`

#### Returns

`any`

#### Defined in

index.d.ts:277

___

### fromString

▸ **fromString**(`s`): [`RuneId`](RuneId.md)

#### Parameters

| Name | Type |
| :------ | :------ |
| `s` | `string` |

#### Returns

[`RuneId`](RuneId.md)

#### Defined in

index.d.ts:273
