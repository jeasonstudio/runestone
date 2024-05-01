use super::types::*;
use gloo_utils::format::JsValueSerdeExt;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize, Default, Copy, Clone)]
#[wasm_bindgen]
pub struct Terms {
    amount: Option<u128>,
    cap: Option<u128>,

    #[wasm_bindgen(readonly)]
    pub height: Range,

    #[wasm_bindgen(readonly)]
    pub offset: Range,

    #[wasm_bindgen(skip)]
    #[serde(skip)]
    pub source: ordinals::Terms,
}

#[wasm_bindgen]
impl Terms {
    #[wasm_bindgen(constructor)]
    pub fn new(terms: Option<Terms>) -> Terms {
        terms.unwrap_or_default()
    }

    #[wasm_bindgen(getter)]
    pub fn amount(&self) -> Option<js_sys::BigInt> {
        match self.source.amount {
            Some(amount) => Some(js_sys::BigInt::from(amount)),
            None => None,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn cap(&self) -> Option<js_sys::BigInt> {
        match self.source.cap {
            Some(cap) => Some(js_sys::BigInt::from(cap)),
            None => None,
        }
    }

    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_json_value(&self) -> JsValue {
        JsValue::from_serde(&self).unwrap()
    }
}

impl From<ordinals::Terms> for Terms {
    fn from(source: ordinals::Terms) -> Self {
        let height = create_range_from_tuple(source.height);
        let offset = create_range_from_tuple(source.offset);
        Self {
            amount: source.amount,
            cap: source.cap,
            height,
            offset,
            source,
        }
    }
}
