use super::rune::*;
use gloo_utils::format::JsValueSerdeExt;
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
    #[serde(skip)]
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

    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_json_value(&self) -> JsValue {
        JsValue::from_serde(&self).unwrap()
    }

    #[wasm_bindgen(js_name = "fromString")]
    pub fn from_string(s: &str) -> Result<SpacedRune, JsValue> {
        let spaced_rune = ordinals::SpacedRune::from_str(s).unwrap();
        Ok(Self::from(spaced_rune))
    }
}

impl From<ordinals::SpacedRune> for SpacedRune {
    fn from(source: ordinals::SpacedRune) -> Self {
        Self {
            rune: Rune::from(source.rune),
            spacers: source.spacers,
            source,
        }
    }
}
