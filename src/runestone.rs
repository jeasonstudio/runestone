use super::edict::*;
use super::etching::*;
use super::rune_id::*;
use super::transaction::tx::Transaction;
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
    pub source: ordinals::Runestone,
}

#[wasm_bindgen]
impl Runestone {
    #[wasm_bindgen(constructor)]
    pub fn new(
        edicts: Vec<Edict>,
        etching: Option<Etching>,
        mint: Option<RuneId>,
        pointer: Option<u32>,
    ) -> Runestone {
        let edicts_source = edicts.iter().map(|&edict| edict.source).collect();
        let rune_id = mint.unwrap();
        let etching_source = etching.unwrap();
        let source = ordinals::Runestone {
            edicts: edicts_source,
            etching: Some(etching_source.source),
            mint: Some(rune_id.source),
            pointer,
        };
        Runestone {
            edicts,
            etching,
            mint,
            pointer,
            source,
        }
    }

    #[wasm_bindgen(js_name = "edicts", getter)]
    pub fn to_edicts(&self) -> js_sys::Array {
        let result = js_sys::Array::new();
        for edict in self.edicts.iter() {
            result.push(&serde_wasm_bindgen::to_value(edict).unwrap());
        }
        result
    }

    #[wasm_bindgen]
    pub fn encipher(&self) -> js_sys::Uint8Array {
        let script_buf = self.source.encipher();
        js_sys::Uint8Array::from(script_buf.to_bytes().as_slice())
    }

    #[wasm_bindgen]
    pub fn decipher(tx: JsValue) -> Runestone {
        let transaction: Transaction = match serde_wasm_bindgen::from_value(tx) {
            Ok(transaction) => transaction,
            Err(_) => throw_str("Cenotaph: cannot parse transaction from JS value."),
        };
        let bitcoin_tx = transaction.to_source();
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
                    .map(|&edict| create_edict_from_source(edict))
                    .collect();

                let etching = match runestone.etching {
                    Some(etching) => Some(create_etching_from_source(etching)),
                    None => None,
                };

                let mint = match runestone.mint {
                    Some(mint) => Some(create_rune_id_from_source(mint)),
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
}
