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

index.d.ts:179

## Properties

### block

• `Readonly` **block**: `bigint`

#### Defined in

index.d.ts:206

___

### tx

• `Readonly` **tx**: `number`

#### Defined in

index.d.ts:209

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

index.d.ts:184

___

### free

▸ **free**(): `void`

#### Returns

`void`

#### Defined in

index.d.ts:174

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

index.d.ts:190

___

### toJSON

▸ **toJSON**(): `any`

#### Returns

`any`

#### Defined in

index.d.ts:198

___

### toString

▸ **toString**(): `string`

#### Returns

`string`

#### Defined in

index.d.ts:194

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

index.d.ts:203
