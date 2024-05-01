use super::types::*;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use wasm_bindgen::prelude::*;
use gloo_utils::format::JsValueSerdeExt;

#[derive(Serialize, Deserialize, Default, Clone, Copy)]
#[wasm_bindgen]
pub struct RuneId {
    #[wasm_bindgen(readonly)]
    pub block: u64,

    #[wasm_bindgen(readonly)]
    pub tx: u32,

    #[wasm_bindgen(skip)]
    #[serde(skip)]
    pub source: ordinals::RuneId,
}

#[wasm_bindgen]
impl RuneId {
    #[wasm_bindgen(constructor)]
    pub fn new(block: u64, tx: u32) -> Self {
        let source = ordinals::RuneId { block, tx };
        Self { block, tx, source }
    }

    #[wasm_bindgen]
    pub fn delta(&self, next: Self) -> Option<Range> {
        let next_rune_id = ordinals::RuneId {
            block: next.block,
            tx: next.tx,
        };
        let tuple = self.source.delta(next_rune_id);
        match tuple {
            Some((start, end)) => Some(Range {
                start: Some(start as u64),
                end: Some(end as u64),
            }),
            None => None,
        }
    }

    #[wasm_bindgen]
    pub fn next(&self, block: u64, tx: u32) -> Option<RuneId> {
        let next_rune_id = self.source.next(block as u128, tx as u128);
        match next_rune_id {
            Some(value) => Some(Self::from(value)),
            None => None,
        }
    }

    #[wasm_bindgen(js_name = "toString")]
    pub fn to_string(&self) -> String {
        self.source.to_string()
    }

    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_json_value(&self) -> JsValue {
        JsValue::from_serde(&self).unwrap()
    }

    #[wasm_bindgen(js_name = "fromString")]
    pub fn from_string(s: &str) -> Result<RuneId, JsValue> {
        let source = ordinals::RuneId::from_str(s).unwrap();
        Ok(Self::from(source))
    }
}

impl From<ordinals::RuneId> for RuneId {
    fn from(source: ordinals::RuneId) -> Self {
        Self {
            block: source.block,
            tx: source.tx,
            source,
        }
    }
}
