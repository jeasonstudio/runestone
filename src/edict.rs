use super::rune_id::*;
use ordinals::Edict as EdictOrd;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize, Default, Clone, Copy)]
#[wasm_bindgen]
pub struct Edict {
    #[wasm_bindgen(readonly)]
    pub id: RuneId,

    #[wasm_bindgen(readonly)]
    pub amount: u64,

    #[wasm_bindgen(readonly)]
    pub output: u32,

    #[wasm_bindgen(skip)]
    pub source: EdictOrd,
}

#[wasm_bindgen]
impl Edict {
    #[wasm_bindgen(constructor)]
    pub fn new(id: RuneId, amount: u64, output: u32) -> Edict {
        let source = EdictOrd {
            id: id.source,
            amount: amount as u128,
            output,
        };
        Edict { id, amount, output, source }
    }
}
