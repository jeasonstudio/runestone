use super::rune_id::*;
use gloo_utils::format::JsValueSerdeExt;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize, Default, Clone, Copy)]
#[wasm_bindgen]
pub struct Edict {
    #[wasm_bindgen(readonly)]
    pub id: RuneId,

    amount: u128,

    #[wasm_bindgen(readonly)]
    pub output: u32,

    #[wasm_bindgen(skip)]
    #[serde(skip)]
    pub source: ordinals::Edict,
}

#[wasm_bindgen]
impl Edict {
    #[wasm_bindgen(constructor)]
    pub fn new(id: RuneId, amount: js_sys::BigInt, output: u32) -> Self {
        let amount: u128 = amount.try_into().unwrap();
        let source = ordinals::Edict {
            id: id.source,
            amount,
            output,
        };
        Self {
            id,
            output,
            amount,
            source,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn amount(&self) -> js_sys::BigInt {
        js_sys::BigInt::from(self.source.amount)
    }

    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_json_value(&self) -> JsValue {
        JsValue::from_serde(&self).unwrap()
    }
}

impl From<ordinals::Edict> for Edict {
    fn from(source: ordinals::Edict) -> Self {
        Self {
            id: RuneId::from(source.id),
            output: source.output,
            amount: source.amount,
            source,
        }
    }
}
