use serde::{Deserialize, Serialize};
use std::{fmt::Write, num::ParseIntError};
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

pub fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}

pub fn encode_hex(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        write!(&mut s, "{:02x}", b).unwrap();
    }
    s
}
