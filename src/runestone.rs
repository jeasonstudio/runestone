use std::fmt::Debug;
use std::fmt::Formatter;

use super::edict::*;
use super::etching::*;
use super::rune::*;
use super::rune_id::*;
use bitcoin::absolute::LockTime;
use bitcoin::ScriptBuf;
use bitcoin::Transaction;
use bitcoin::TxOut;
use ordinals::{Artifact, Flaw, Runestone as RunestoneOrd};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::throw_str;

#[derive(Default, Deserialize, Serialize)]
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
    pub source: RunestoneOrd,
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
        let source = RunestoneOrd {
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
    pub fn decipher(script_pubkey: js_sys::Uint8Array) -> Runestone {
        let script_pubkey = ScriptBuf::from(script_pubkey.to_vec());
        let transaction = Transaction {
            input: Vec::new(),
            output: vec![TxOut {
                script_pubkey,
                value: 0,
            }],
            lock_time: LockTime::ZERO,
            version: 2,
        };
        let artifact = RunestoneOrd::decipher(&transaction).unwrap();
        match artifact {
            Artifact::Cenotaph(cenotaph) => {
                let flaw = cenotaph.flaw.unwrap();
                throw_str(&format!("Cenotaph: {flaw}"));
            }
            Artifact::Runestone(runestone) => {
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
