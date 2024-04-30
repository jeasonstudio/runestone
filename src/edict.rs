use super::rune_id::*;
use ordinals::Edict as EdictOrd;
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
    pub source: EdictOrd,
}

#[wasm_bindgen]
impl Edict {
    #[wasm_bindgen(constructor)]
    pub fn new(id: RuneId, amount: js_sys::BigInt, output: u32) -> Edict {
        let amount: u128 = amount.try_into().unwrap();
        let source = EdictOrd {
            id: id.source,
            amount,
            output,
        };
        Edict {
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
}

pub fn create_edict_from_source(source: EdictOrd) -> Edict {
    Edict {
        id: create_rune_id_from_source(source.id),
        output: source.output,
        amount: source.amount,
        source,
    }
}
