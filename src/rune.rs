use super::types::*;
use bitcoin::Network as BitcoinNetwork;
use ordinals::{Height, Rune as RuneOrd};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize, Default, Copy, Clone)]
#[wasm_bindgen]
pub struct Rune {
    value: u128,

    #[wasm_bindgen(skip)]
    pub source: RuneOrd,
}

#[wasm_bindgen]
impl Rune {
    #[wasm_bindgen(constructor)]
    pub fn new(value: js_sys::BigInt) -> Rune {
        let rune_value: u128 = value.try_into().unwrap();
        let source = RuneOrd(rune_value);
        Rune {
            value: rune_value,
            source,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn value(&self) -> js_sys::BigInt {
        self.source.0.into()
    }

    #[wasm_bindgen(js_name = "isReserved")]
    pub fn is_reserved(&self) -> bool {
        self.source.is_reserved()
    }

    #[wasm_bindgen]
    pub fn commitment(&self) -> Vec<u8> {
        self.source.commitment()
    }

    #[wasm_bindgen(js_name = "toString")]
    pub fn to_string(&self) -> String {
        self.source.to_string()
    }

    #[wasm_bindgen(js_name = "valueOf")]
    pub fn value_of(&self) -> js_sys::BigInt {
        self.source.0.into()
    }

    // static
    #[wasm_bindgen(js_name = "firstRuneHeight")]
    pub fn first_rune_height(network: Network) -> u32 {
        let network: BitcoinNetwork =
            BitcoinNetwork::from_core_arg(&network.as_string().unwrap()).unwrap();
        RuneOrd::first_rune_height(network)
    }

    #[wasm_bindgen(js_name = "minimumAtHeight")]
    pub fn minimum_at_height(network: Network, height: u32) -> Self {
        let network: BitcoinNetwork =
            BitcoinNetwork::from_core_arg(&network.as_string().unwrap()).unwrap();
        let source: RuneOrd = RuneOrd::minimum_at_height(network, Height(height));
        create_rune_from_source(source)
    }

    #[wasm_bindgen]
    pub fn reserved(block: u64, tx: u32) -> Self {
        let source: RuneOrd = RuneOrd::reserved(block, tx);
        create_rune_from_source(source)
    }

    #[wasm_bindgen(js_name = "fromString")]
    pub fn from_string(s: &str) -> Result<Rune, JsValue> {
        let source: RuneOrd = RuneOrd::from_str(s).unwrap();
        Ok(create_rune_from_source(source))
    }
}

pub fn create_rune_from_source(source: RuneOrd) -> Rune {
    Rune { source, value: source.0 }
}
