use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize, Default, Copy, Clone)]
#[wasm_bindgen]
pub struct Range {
    pub start: Option<u64>,
    pub end: Option<u64>,
}

#[wasm_bindgen]
impl Range {
    #[wasm_bindgen(constructor)]
    pub fn new(start: Option<u64>, end: Option<u64>) -> Self {
        Self { start, end }
    }
}

impl From<(Option<u64>, Option<u64>)> for Range {
    fn from(source: (Option<u64>, Option<u64>)) -> Self {
        Self {
            start: source.0,
            end: source.1,
        }
    }
}

#[wasm_bindgen(typescript_custom_section)]
const Network: &'static str = r#"
export type Network = "main" | "test" | "signet" | "regtest";
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "Network")]
    pub type Network;
}
