use super::rune::*;
use ordinals::SpacedRune as SpacedRuneOrd;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize, Default, Clone, Copy)]
#[wasm_bindgen]
pub struct SpacedRune {
    #[wasm_bindgen(readonly)]
    pub rune: Rune,

    #[wasm_bindgen(readonly)]
    pub spacers: u32,

    #[wasm_bindgen(skip)]
    pub source: SpacedRuneOrd,
}

#[wasm_bindgen]
impl SpacedRune {
    #[wasm_bindgen(constructor)]
    pub fn new(rune: Rune, spacers: u32) -> SpacedRune {
        let source = SpacedRuneOrd {
            rune: rune.source,
            spacers,
        };
        SpacedRune {
            rune,
            spacers,
            source,
        }
    }

    #[wasm_bindgen(js_name = "toString")]
    pub fn to_string(&self) -> String {
        self.source.to_string()
    }

    #[wasm_bindgen(js_name = "fromString")]
    pub fn from_string(s: &str) -> Result<SpacedRune, JsValue> {
        let spaced_rune = SpacedRuneOrd::from_str(s).unwrap();
        Ok(SpacedRune {
            rune: Rune::new(spaced_rune.rune.0 as u64),
            spacers: spaced_rune.spacers,
            source: spaced_rune,
        })
    }
}
