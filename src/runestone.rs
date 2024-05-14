use super::*;
use wasm_bindgen::throw_str;

#[derive(Default, Serialize, Deserialize, Tsify, Clone)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct RunestoneParams {
    pub edicts: Option<Vec<Edict>>,
    pub etching: Option<Etching>,
    pub mint: Option<RuneId>,
    pub pointer: Option<u32>,
}

#[derive(Default, Deserialize, Serialize, Clone)]
#[wasm_bindgen]
pub struct Runestone {
    edicts: Vec<Edict>,

    #[wasm_bindgen]
    pub etching: Option<Etching>,

    #[wasm_bindgen]
    pub mint: Option<RuneId>,

    #[wasm_bindgen]
    pub pointer: Option<u32>,
}

#[wasm_bindgen]
impl Runestone {
    #[wasm_bindgen(constructor)]
    pub fn new(params: Option<RunestoneParams>) -> Self {
        match params {
            None => Self::default(),
            Some(params) => Self::from(params),
        }
    }

    fn source(&self) -> ordinals::Runestone {
        ordinals::Runestone::from(self.clone())
    }

    #[wasm_bindgen(js_name = "edicts", getter)]
    pub fn to_edicts(&self) -> js_sys::Array {
        let result = js_sys::Array::new();
        for edict in self.edicts.iter() {
            result.push(&serde_wasm_bindgen::to_value(edict).unwrap());
        }
        result
    }

    #[wasm_bindgen(js_name = "edicts", setter)]
    pub fn set_edicts(&mut self, values: Vec<Edict>) {
        self.edicts = values;
    }

    #[wasm_bindgen]
    pub fn encipher(&self) -> String {
        let script_buf = self.source().encipher();
        script_buf.to_hex_string()
    }

    #[wasm_bindgen]
    pub fn decipher(tx: JsValue) -> Self {
        let transaction: Transaction = match serde_wasm_bindgen::from_value(tx) {
            Ok(transaction) => transaction,
            Err(_) => throw_str("Cenotaph: cannot parse transaction from JS value."),
        };
        let bitcoin_tx = bitcoin::Transaction::from(transaction);
        let artifact = ordinals::Runestone::decipher(&bitcoin_tx).unwrap();
        match artifact {
            ordinals::Artifact::Cenotaph(cenotaph) => {
                let flaw = cenotaph.flaw.unwrap();
                throw_str(&format!("Cenotaph: {flaw}"));
            }
            ordinals::Artifact::Runestone(runestone) => {
                let edicts = runestone
                    .edicts
                    .iter()
                    .map(|&edict| Edict::from(edict))
                    .collect();

                let etching = match runestone.etching {
                    Some(etching) => Some(Etching::from(etching)),
                    None => None,
                };

                let mint = match runestone.mint {
                    Some(mint) => Some(RuneId::from(mint)),
                    None => None,
                };

                let pointer = match runestone.pointer {
                    Some(pointer) => Some(pointer),
                    None => None,
                };

                Runestone {
                    edicts,
                    etching,
                    mint,
                    pointer,
                }
            }
        }
    }

    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_json_value(&self) -> Result<JsValue, Error> {
        serde_wasm_bindgen::to_value(&self)
    }

    #[wasm_bindgen(js_name = "valueOf")]
    pub fn value_of(&self) -> Result<JsValue, Error> {
        serde_wasm_bindgen::to_value(&self)
    }
}

impl From<ordinals::Runestone> for Runestone {
    fn from(source: ordinals::Runestone) -> Self {
        Self {
            edicts: source
                .edicts
                .iter()
                .map(|&edict| Edict::from(edict))
                .collect(),
            etching: match source.etching {
                Some(etching) => Some(Etching::from(etching)),
                None => None,
            },
            mint: match source.mint {
                Some(mint) => Some(RuneId::from(mint)),
                None => None,
            },
            pointer: source.pointer,
        }
    }
}

impl From<Runestone> for ordinals::Runestone {
    fn from(source: Runestone) -> Self {
        ordinals::Runestone {
            edicts: source
                .edicts
                .iter()
                .map(|&edict| ordinals::Edict::from(edict))
                .collect(),
            etching: match source.etching {
                Some(etching) => Some(ordinals::Etching::from(etching)),
                None => None,
            },
            mint: match source.mint {
                Some(mint) => Some(ordinals::RuneId::from(mint)),
                None => None,
            },
            pointer: source.pointer,
        }
    }
}

impl From<RunestoneParams> for Runestone {
    fn from(params: RunestoneParams) -> Self {
        let edicts = match params.edicts {
            Some(edicts) => edicts,
            None => vec![],
        };

        Self {
            edicts,
            etching: params.etching,
            mint: params.mint,
            pointer: params.pointer,
        }
    }
}
