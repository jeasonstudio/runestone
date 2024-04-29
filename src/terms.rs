use super::types::*;
use ordinals::Terms as TermsOrd;
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
    pub source: TermsOrd,
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
}

pub fn create_terms_from_source(source: TermsOrd) -> Terms {
    let height = create_range_from_tuple(source.height);
    let offset = create_range_from_tuple(source.offset);
    Terms {
        amount: source.amount,
        cap: source.cap,
        height,
        offset,
        source,
    }
}
