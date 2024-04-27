use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize, Default, Copy, Clone)]
#[wasm_bindgen]
pub struct Range {
    pub start: Option<u64>,
    pub end: Option<u64>,
}

#[wasm_bindgen(typescript_custom_section)]
const Network: &'static str = r#"
export type Network = "main" | "test" | "signet" | "regtest";
"#;

#[wasm_bindgen(typescript_custom_section)]
const Uint128: &'static str = r#"
export type Uint128 = bigint;
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "Network")]
    pub type Network;

    #[wasm_bindgen(typescript_type = "Uint128")]
    pub type Uint128;
}
