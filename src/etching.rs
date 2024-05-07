use super::rune::*;
use super::spaced_rune::*;
use super::terms::*;
use gloo_utils::format::JsValueSerdeExt;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Default, Serialize, Deserialize, Copy, Clone)]
#[wasm_bindgen]
pub struct Etching {
    #[wasm_bindgen(readonly)]
    pub divisibility: Option<u8>,

    premine: Option<u128>,

    #[wasm_bindgen(readonly)]
    pub rune: Option<Rune>,

    #[wasm_bindgen(readonly)]
    pub spacers: Option<u32>,

    #[wasm_bindgen(readonly)]
    pub symbol: Option<char>,

    #[wasm_bindgen(readonly)]
    pub terms: Option<Terms>,

    #[wasm_bindgen(readonly)]
    pub turbo: bool,

    #[wasm_bindgen(skip)]
    #[serde(skip)]
    pub source: ordinals::Etching,
}

#[wasm_bindgen]
impl Etching {
    #[wasm_bindgen(constructor)]
    pub fn new(spaced_rune: SpacedRune) -> Etching {
        let source = ordinals::Etching {
            divisibility: None,
            premine: None,
            rune: Some(spaced_rune.rune.source),
            spacers: Some(spaced_rune.spacers),
            symbol: None,
            terms: None,
            turbo: true,
        };
        Etching::from(source)
    }

    #[wasm_bindgen(getter)]
    pub fn premine(&self) -> Option<js_sys::BigInt> {
        match self.source.premine {
            Some(premine) => Some(js_sys::BigInt::from(premine)),
            None => None,
        }
    }

    #[wasm_bindgen(js_name = "divisibility", setter)]
    pub fn set_divisibility(&mut self, values: u8) {
        self.source.divisibility = Some(values);
        self.divisibility = Some(values);
    }

    #[wasm_bindgen(js_name = "premine", setter)]
    pub fn set_premine(&mut self, values: js_sys::BigInt) {
        let value: u128 = values.try_into().unwrap();
        self.source.premine = Some(value);
        self.premine = Some(value);
    }

    #[wasm_bindgen(js_name = "symbol", setter)]
    pub fn set_symbol(&mut self, values: char) {
        self.source.symbol = Some(values);
        self.symbol = Some(values);
    }

    #[wasm_bindgen(js_name = "terms", setter)]
    pub fn set_terms(&mut self, values: Terms) {
        self.source.terms = Some(values.source);
        self.terms = Some(values);
    }

    #[wasm_bindgen(js_name = "turbo", setter)]
    pub fn set_turbo(&mut self, values: bool) {
        self.source.turbo = values;
        self.turbo = values;
    }

    #[wasm_bindgen]
    pub fn supply(&self) -> Option<u64> {
        match self.source.supply() {
            Some(value) => Some(value as u64),
            None => None,
        }
    }

    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_json_value(&self) -> JsValue {
        JsValue::from_serde(&self).unwrap()
    }
}

impl From<ordinals::Etching> for Etching {
    fn from(source: ordinals::Etching) -> Self {
        let divisibility = match source.divisibility {
            Some(divisibility) => Some(divisibility),
            None => None,
        };
        let premine = match source.premine {
            Some(premine) => Some(premine),
            None => None,
        };
        let rune = match source.rune {
            Some(rune) => Some(Rune::from(rune)),
            None => None,
        };
        let spacers = match source.spacers {
            Some(spacers) => Some(spacers),
            None => None,
        };
        let symbol = match source.symbol {
            Some(symbol) => Some(symbol),
            None => None,
        };
        let terms = match source.terms {
            Some(terms) => Some(Terms::from(terms)),
            None => None,
        };
        Self {
            divisibility,
            premine,
            rune,
            spacers,
            symbol,
            terms,
            turbo: source.turbo,
            source,
        }
    }
}
