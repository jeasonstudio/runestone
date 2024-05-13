use super::edict::*;
use super::etching::*;
use super::rune_id::*;
use super::transaction::Transaction;
use super::utils::*;
use gloo_utils::format::JsValueSerdeExt;
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::*;
use wasm_bindgen::throw_str;

#[derive(Default, Deserialize, Serialize, Tsify)]
#[wasm_bindgen]
pub struct Runestone {
    edicts: Vec<Edict>,

    #[wasm_bindgen(readonly)]
    pub etching: Option<Etching>,

    #[wasm_bindgen(readonly)]
    pub mint: Option<RuneId>,

    #[wasm_bindgen(readonly)]
    pub pointer: Option<u32>,

    #[wasm_bindgen(skip)]
    #[serde(skip)]
    pub source: ordinals::Runestone,
}

#[wasm_bindgen]
impl Runestone {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let default = ordinals::Runestone::default();
        Self::from(default)
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
        let edicts_source = values.iter().map(|&edict| edict.source).collect();
        self.source.edicts = edicts_source;
        self.edicts = values;
    }

    #[wasm_bindgen(js_name = "etching", setter)]
    pub fn set_etching(&mut self, values: Etching) {
        self.source.etching = Some(values.source);
        self.etching = Some(values);
    }

    #[wasm_bindgen(js_name = "mint", setter)]
    pub fn set_mint(&mut self, values: RuneId) {
        self.source.mint = Some(values.source);
        self.mint = Some(values);
    }

    #[wasm_bindgen(js_name = "pointer", setter)]
    pub fn set_pointer(&mut self, values: u32) {
        self.source.pointer = Some(values);
        self.pointer = Some(values);
    }

    #[wasm_bindgen]
    pub fn encipher(&self) -> String {
        let script_buf = self.source.encipher();
        let buffer = js_sys::Uint8Array::from(script_buf.to_bytes().as_slice());
        encode_hex(buffer)
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
                    source: runestone,
                }
            }
        }
    }

    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_json_value(&self) -> JsValue {
        JsValue::from_serde(&self).unwrap()
    }

    #[wasm_bindgen(js_name = "valueOf")]
    pub fn value_of(&self) -> JsValue {
        JsValue::from_serde(&self).unwrap()
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
            source,
        }
    }
}
