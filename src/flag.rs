// use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

// #[derive(Serialize, Deserialize, Default, Clone, Copy)]
// #[wasm_bindgen]
// pub struct Flag {
//     pub foo: u128,
// }

#[wasm_bindgen]
pub fn test() -> u64 {
    u64::MAX
}
