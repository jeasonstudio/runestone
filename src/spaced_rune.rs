use super::*;
use std::str::FromStr;

#[derive(Serialize, Deserialize, Default, Clone, Copy)]
#[wasm_bindgen]
pub struct SpacedRune {
    #[wasm_bindgen]
    pub rune: Rune,

    #[wasm_bindgen]
    pub spacers: u32,
}

#[wasm_bindgen]
impl SpacedRune {
    #[wasm_bindgen(constructor)]
    pub fn new(rune: Rune, spacers: u32) -> Self {
        Self { rune, spacers }
    }

    fn source(&self) -> ordinals::SpacedRune {
        ordinals::SpacedRune::from(self.clone())
    }

    #[wasm_bindgen(getter, js_name = "name")]
    pub fn name(&self) -> String {
        self.source().to_string()
    }

    #[wasm_bindgen(getter, js_name = "runeValue")]
    pub fn rune_value(&self) -> js_sys::BigInt {
        self.rune.get_value()
    }

    #[wasm_bindgen(js_name = "toString")]
    pub fn to_string(&self) -> String {
        self.source().to_string()
    }

    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_json_value(&self) -> Result<JsValue, Error> {
        serde_wasm_bindgen::to_value(&self)
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
        }
    }
}

impl From<SpacedRune> for ordinals::SpacedRune {
    fn from(source: SpacedRune) -> Self {
        ordinals::SpacedRune {
            rune: ordinals::Rune::from(source.rune),
            spacers: source.spacers,
        }
    }
}
