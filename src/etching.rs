use super::*;

#[derive(Default, Serialize, Deserialize, Tsify, Clone)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[serde(rename_all = "camelCase")]
pub struct EtchingParams {
    pub spaced_rune: Option<SpacedRune>,
    pub premine: Option<u128>,
    pub divisibility: Option<u8>,
    pub symbol: Option<char>,
    pub terms: Option<Terms>,
    pub turbo: Option<bool>,
}

#[derive(Default, Serialize, Deserialize, Copy, Clone)]
#[wasm_bindgen(inspectable)]
pub struct Etching {
    premine: Option<u128>,

    #[wasm_bindgen]
    pub divisibility: Option<u8>,

    #[wasm_bindgen]
    pub rune: Option<Rune>,

    #[wasm_bindgen]
    pub spacers: Option<u32>,

    #[wasm_bindgen]
    pub symbol: Option<char>,

    #[wasm_bindgen]
    pub terms: Option<Terms>,

    #[wasm_bindgen]
    pub turbo: bool,
}

#[wasm_bindgen]
impl Etching {
    #[wasm_bindgen(constructor)]
    pub fn new(params: Option<EtchingParams>) -> Self {
        match params {
            None => Self::default(),
            Some(params) => Self::from(params),
        }
    }

    fn source(&self) -> ordinals::Etching {
        ordinals::Etching::from(self.clone())
    }

    #[wasm_bindgen(getter, js_name = "premine")]
    pub fn get_premine(&self) -> Option<js_sys::BigInt> {
        match self.premine {
            Some(premine) => Some(js_sys::BigInt::from(premine)),
            None => None,
        }
    }

    #[wasm_bindgen(setter, js_name = "premine")]
    pub fn set_premine(&mut self, new_premine: Option<js_sys::BigInt>) {
        self.premine = match new_premine {
            Some(premine) => Some(premine.try_into().unwrap()),
            None => None,
        };
    }

    #[wasm_bindgen]
    pub fn supply(&self) -> Option<js_sys::BigInt> {
        match self.source().supply() {
            Some(value) => Some(js_sys::BigInt::from(value)),
            None => None,
        }
    }

    #[wasm_bindgen(js_name = "valueOf")]
    pub fn value_of(&self) -> Result<JsValue, Error> {
        let serializer = serde_wasm_bindgen::Serializer::json_compatible();
        self.serialize(&serializer)
    }
}

impl From<ordinals::Etching> for Etching {
    fn from(source: ordinals::Etching) -> Self {
        let rune = match source.rune {
            Some(rune) => Some(Rune::from(rune)),
            None => None,
        };
        let terms = match source.terms {
            Some(terms) => Some(Terms::from(terms)),
            None => None,
        };
        Self {
            divisibility: source.divisibility,
            premine: source.premine,
            rune,
            spacers: source.spacers,
            symbol: source.symbol,
            terms,
            turbo: source.turbo,
        }
    }
}

impl From<EtchingParams> for Etching {
    fn from(params: EtchingParams) -> Self {
        let rune = match params.spaced_rune {
            Some(spaced_rune) => Some(Rune::from(spaced_rune)),
            None => None,
        };
        let spacers = match params.spaced_rune {
            Some(spaced_rune) => Some(spaced_rune.spacers),
            None => None,
        };
        let turbo = match params.turbo {
            Some(turbo) => turbo,
            None => true, // TODO
        };
        Self {
            divisibility: params.divisibility,
            premine: params.premine,
            rune,
            spacers,
            symbol: params.symbol,
            terms: params.terms,
            turbo,
        }
    }
}

impl From<Etching> for ordinals::Etching {
    fn from(source: Etching) -> Self {
        ordinals::Etching {
            divisibility: source.divisibility,
            premine: source.premine,
            rune: match source.rune {
                Some(rune) => Some(ordinals::Rune::from(rune)),
                None => None,
            },
            spacers: source.spacers,
            symbol: source.symbol,
            terms: match source.terms {
                Some(terms) => Some(ordinals::Terms::from(terms)),
                None => None,
            },
            turbo: source.turbo,
        }
    }
}
