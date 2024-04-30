use super::rune::*;
use super::terms::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Default, Serialize, Deserialize, Copy, Clone)]
#[wasm_bindgen]
pub struct Etching {
    #[wasm_bindgen(readonly)]
    pub divisibility: Option<u8>,

    premine: Option<u128>,

    #[wasm_bindgen(readonly)]
    pub rune: Option<Rune>,

    #[wasm_bindgen(readonly)]
    pub spacers: Option<u32>,

    #[wasm_bindgen(readonly)]
    pub symbol: Option<char>,

    #[wasm_bindgen(readonly)]
    pub terms: Option<Terms>,

    #[wasm_bindgen(readonly)]
    pub turbo: bool,

    #[wasm_bindgen(skip)]
    pub source: ordinals::Etching,
}

#[wasm_bindgen]
impl Etching {
    #[wasm_bindgen(constructor)]
    pub fn new(etching: Option<Etching>) -> Etching {
        etching.unwrap_or_default()
    }

    #[wasm_bindgen(getter)]
    pub fn premine(&self) -> Option<js_sys::BigInt> {
        match self.source.premine {
            Some(premine) => Some(js_sys::BigInt::from(premine)),
            None => None,
        }
    }

    #[wasm_bindgen]
    pub fn supply(&self) -> Option<u64> {
        let value = self.source.supply().unwrap();
        Some(value as u64)
    }
}

pub fn create_etching_from_source(source: ordinals::Etching) -> Etching {
    let divisibility = match source.divisibility {
        Some(divisibility) => Some(divisibility),
        None => None,
    };
    let premine = match source.premine {
        Some(premine) => Some(premine),
        None => None,
    };
    let rune = match source.rune {
        Some(rune) => Some(create_rune_from_source(rune)),
        None => None,
    };
    let spacers = match source.spacers {
        Some(spacers) => Some(spacers),
        None => None,
    };
    let symbol = match source.symbol {
        Some(symbol) => Some(symbol),
        None => None,
    };
    let terms = match source.terms {
        Some(terms) => Some(create_terms_from_source(terms)),
        None => None,
    };
    Etching {
        divisibility,
        premine,
        rune,
        spacers,
        symbol,
        terms,
        turbo: source.turbo,
        source,
    }
}
