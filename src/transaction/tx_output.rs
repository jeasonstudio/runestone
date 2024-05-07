use bitcoin::{ScriptBuf, TxOut};
use serde::{Deserialize, Serialize};
use tsify::Tsify;

use crate::utils::decode_hex_to_bytes;

#[derive(Default, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct TxOutput {
    /// The value of the output, in satoshis.
    pub value: u64,
    /// The script which must be satisfied for the output to be spent.
    pub script_pubkey: String,
}

impl TxOutput {
    pub fn to_source(&self) -> TxOut {
        let script_pubkey = ScriptBuf::from_bytes(decode_hex_to_bytes(self.script_pubkey.to_owned()));
        TxOut {
            value: self.value,
            script_pubkey,
        }
    }
}
