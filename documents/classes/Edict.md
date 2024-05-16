[@ordjs/runestone](../README.md) / [Exports](../modules.md) / Edict

# Class: Edict

## Table of contents

### Constructors

- [constructor](Edict.md#constructor)

### Properties

- [amount](Edict.md#amount)
- [id](Edict.md#id)
- [output](Edict.md#output)

### Methods

- [free](Edict.md#free)
- [toJSON](Edict.md#tojson)
- [toString](Edict.md#tostring)
- [valueOf](Edict.md#valueof)

## Constructors

### constructor

• **new Edict**(`id`, `amount`, `output?`): [`Edict`](Edict.md)

#### Parameters

| Name | Type |
| :------ | :------ |
| `id` | [`RuneId`](RuneId.md) |
| `amount` | `bigint` |
| `output?` | `number` |

#### Returns

[`Edict`](Edict.md)

#### Defined in

index.d.ts:91

## Properties

### amount

• **amount**: `bigint`

#### Defined in

index.d.ts:98

___

### id

• **id**: [`RuneId`](RuneId.md)

#### Defined in

index.d.ts:101

___

### output

• **output**: `number`

#### Defined in

index.d.ts:104

## Methods

### free

▸ **free**(): `void`

#### Returns

`void`

#### Defined in

index.d.ts:85

___

### toJSON

▸ **toJSON**(): `Object`

* Return copy of self without private attributes.

#### Returns

`Object`

#### Defined in

index.d.ts:80

___

### toString

▸ **toString**(): `string`

Return stringified version of self.

#### Returns

`string`

#### Defined in

index.d.ts:84

___

### valueOf

▸ **valueOf**(): `any`

#### Returns

`any`

#### Defined in

index.d.ts:95
