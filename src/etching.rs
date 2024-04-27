use super::rune::*;
use super::terms::*;
use ordinals::Etching as EtchingOrd;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Default, Serialize, Deserialize, Copy, Clone)]
#[wasm_bindgen]
pub struct Etching {
    #[wasm_bindgen(readonly)]
    pub divisibility: Option<u8>,

    #[wasm_bindgen(readonly)]
    pub premine: Option<u64>,

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
    pub source: EtchingOrd,
}

#[wasm_bindgen]
impl Etching {
    #[wasm_bindgen(constructor)]
    pub fn new(etching: Option<Etching>) -> Etching {
        etching.unwrap_or_default()
    }

    pub fn supply(&self) -> Option<u64> {
        let value = self.source.supply().unwrap();
        Some(value as u64)
    }
}
