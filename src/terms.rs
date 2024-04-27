use super::types::*;
use ordinals::Terms as TermsOrd;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize, Default, Copy, Clone)]
#[wasm_bindgen]
pub struct Terms {
    #[wasm_bindgen(readonly)]
    pub amount: Option<u64>,

    #[wasm_bindgen(readonly)]
    pub cap: Option<u64>,

    #[wasm_bindgen(readonly)]
    pub height: Option<Range>,

    #[wasm_bindgen(readonly)]
    pub offset: Option<Range>,

    #[wasm_bindgen(skip)]
    pub source: TermsOrd,
}

#[wasm_bindgen]
impl Terms {
    #[wasm_bindgen(constructor)]
    pub fn new(terms: Option<Terms>) -> Terms {
        terms.unwrap_or_default()
    }
}
