use ordinals::Runestone::Flag as FlagOrd;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize)]
#[wasm_bindgen]
pub enum Flag {
    Etching = 0,
    Terms = 1,
    Turbo = 2,
}
