use super::edict::*;
use super::etching::*;
use super::rune_id::*;
use ordinals::Runestone as RunestoneOrd;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Default, Deserialize, Serialize)]
pub struct Runestone {
    pub edicts: Vec<Edict>,
    pub etching: Option<Etching>,
    pub mint: Option<RuneId>,
    pub pointer: Option<u32>,
}

// #[wasm_bindgen]
// impl Runestone {
#[wasm_bindgen]
pub fn create_ronestone() -> JsValue {
    serde_wasm_bindgen::to_value(&Runestone::default()).unwrap()
}
// }
