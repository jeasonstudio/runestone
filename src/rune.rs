use super::types::*;
use bitcoin::Network as BitcoinNetwork;
use ordinals::{Height, Rune as RuneOrd};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize, Default, Copy, Clone)]
#[wasm_bindgen]
pub struct Rune {
    #[wasm_bindgen(readonly)]
    pub n: u64,

    #[wasm_bindgen(skip)]
    pub source: RuneOrd,
}

#[wasm_bindgen]
impl Rune {
    #[wasm_bindgen(constructor)]
    pub fn new(n: u64) -> Rune {
        let source = RuneOrd(n as u128);
        Rune { n, source }
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
        Rune {
            n: source.0 as u64,
            source,
        }
    }

    #[wasm_bindgen]
    pub fn reserved(block: u64, tx: u32) -> Self {
        let source: RuneOrd = RuneOrd::reserved(block, tx);
        Rune {
            n: source.0 as u64,
            source,
        }
    }

    #[wasm_bindgen(js_name = "fromString")]
    pub fn from_string(s: &str) -> Result<Rune, JsValue> {
        let source: RuneOrd = RuneOrd::from_str(s).unwrap();
        Ok(Rune {
            n: source.0 as u64,
            source,
        })
    }
}
