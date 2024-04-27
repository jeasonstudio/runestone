use super::types::*;
use ordinals::RuneId as RuneIdOrd;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize, Default, Clone, Copy)]
#[wasm_bindgen]
pub struct RuneId {
    #[wasm_bindgen(readonly)]
    pub block: u64,

    #[wasm_bindgen(readonly)]
    pub tx: u32,

    #[wasm_bindgen(skip)]
    pub source: RuneIdOrd,
}

#[wasm_bindgen]
impl RuneId {
    #[wasm_bindgen(constructor)]
    pub fn new(block: u64, tx: u32) -> RuneId {
        let source = RuneIdOrd { block, tx };
        RuneId { block, tx, source }
    }

    #[wasm_bindgen]
    pub fn delta(&self, next: RuneId) -> Range {
        let next_rune_id = RuneIdOrd {
            block: next.block,
            tx: next.tx,
        };
        let (start, end) = self.source.delta(next_rune_id).unwrap_throw();
        Range {
            start: Some(start as u64),
            end: Some(end as u64),
        }
    }

    #[wasm_bindgen]
    pub fn next(&self, block: u64, tx: u32) -> RuneId {
        let next_rune_id = self.source.next(block as u128, tx as u128).unwrap_throw();
        RuneId {
            block: next_rune_id.block,
            tx: next_rune_id.tx,
            source: next_rune_id,
        }
    }

    #[wasm_bindgen(js_name = "toString")]
    pub fn to_string(&self) -> String {
        self.source.to_string()
    }

    #[wasm_bindgen(js_name = "fromString")]
    pub fn from_string(s: &str) -> Result<RuneId, JsValue> {
        let rune_id = RuneIdOrd::from_str(s).unwrap();
        Ok(RuneId {
            block: rune_id.block,
            tx: rune_id.tx,
            source: rune_id,
        })
    }
}
