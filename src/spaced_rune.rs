use super::rune::*;
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
    pub source: ordinals::SpacedRune,
}

#[wasm_bindgen]
impl SpacedRune {
    #[wasm_bindgen(constructor)]
    pub fn new(rune: Rune, spacers: u32) -> SpacedRune {
        let source = ordinals::SpacedRune {
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
        let spaced_rune = ordinals::SpacedRune::from_str(s).unwrap();
        Ok(create_spaced_rune_from_source(spaced_rune))
    }
}

pub fn create_spaced_rune_from_source(source: ordinals::SpacedRune) -> SpacedRune {
    SpacedRune {
        rune: create_rune_from_source(source.rune),
        spacers: source.spacers,
        source,
    }
}
