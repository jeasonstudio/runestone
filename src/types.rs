use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize, Default, Copy, Clone)]
#[wasm_bindgen]
pub struct Range {
    pub start: Option<u64>,
    pub end: Option<u64>,
}

pub fn create_range_from_tuple(t: (Option<u64>, Option<u64>)) -> Range {
    Range {
        start: t.0,
        end: t.1,
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
