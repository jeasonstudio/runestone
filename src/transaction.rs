use super::utils::*;
use serde::{Deserialize, Serialize};
use std::fmt;
use tsify::Tsify;
use wasm_bindgen::throw_str;

#[derive(Default, Serialize, Deserialize, Tsify, Clone)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct TxOutput {
    /// The value of the output, in satoshis.
    pub value: Option<u64>,
    /// The script which must be satisfied for the output to be spent.
    pub script_pubkey: String,
}

impl From<bitcoin::TxOut> for TxOutput {
    fn from(source: bitcoin::TxOut) -> Self {
        Self {
            script_pubkey: source.script_pubkey.to_hex_string(),
            value: Some(source.value),
        }
    }
}

impl From<TxOutput> for bitcoin::TxOut {
    fn from(source: TxOutput) -> Self {
        let script_pubkey = bitcoin::ScriptBuf::from_hex(&source.script_pubkey);
        let value = match source.value {
            Some(value) => value,
            None => 0,
        };
        match script_pubkey {
            Ok(script_pubkey) => bitcoin::TxOut {
                value,
                script_pubkey,
            },
            Err(err) => throw_str(err.to_string().as_str()),
        }
    }
}

impl fmt::Display for TxOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "TxOutput({} satoshis, {})",
            match self.value {
                Some(value) => value,
                None => 0,
            },
            self.script_pubkey
        )
    }
}

#[derive(Default, Serialize, Deserialize, Tsify, Clone)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct TxInput {
    /// The reference to the previous output that is being used an an input.
    pub previous_output: String, // bitcoin::OutPoint,
    /// The script which pushes values on the stack which will cause
    /// the referenced output's script to be accepted.
    pub script_sig: Option<String>,
    /// The sequence number, which suggests to miners which of two
    /// conflicting transactions should be preferred, or 0xFFFFFFFF
    /// to ignore this feature. This is generally never used since
    /// the miner behaviour cannot be enforced.
    pub sequence: Option<u32>,
    /// Witness data: an array of byte-arrays.
    /// Note that this field is *not* (de)serialized with the rest of the TxIn in
    /// Encodable/Decodable, as it is (de)serialized at the end of the full
    /// Transaction. It *is* (de)serialized with the rest of the TxIn in other
    /// (de)serialization routines.
    pub witness: Option<Vec<String>>,
}

impl From<bitcoin::TxIn> for TxInput {
    fn from(source: bitcoin::TxIn) -> Self {
        let previous_output = format!("{}", source.previous_output);
        let script_sig = Some(format!("{:x}", source.script_sig));
        let sequence = Some(source.sequence.to_consensus_u32());

        let witness = source
            .witness
            .iter()
            .map(|w| encode_bytes_to_hex(w.to_vec()))
            .collect();

        Self {
            previous_output,
            script_sig,
            sequence,
            witness: Some(witness),
        }
    }
}

impl From<TxInput> for bitcoin::TxIn {
    fn from(source: TxInput) -> bitcoin::TxIn {
        let previous_output = match source.previous_output.parse::<bitcoin::OutPoint>() {
            Ok(result) => result,
            Err(err) => throw_str(err.to_string().as_str()),
        };
        let script_sig: bitcoin::ScriptBuf = match source.script_sig {
            Some(script_sig) => match bitcoin::ScriptBuf::from_hex(&script_sig) {
                Ok(result) => result,
                Err(err) => throw_str(err.to_string().as_str()),
            },
            None => bitcoin::ScriptBuf::new(),
        };
        let sequence = match source.sequence {
            Some(sequence) => bitcoin::Sequence::from_consensus(sequence),
            None => bitcoin::Sequence::ZERO,
        };
        let witness: Vec<Vec<u8>> = match source.witness {
            Some(witness) => witness
                .iter()
                .map(|w| decode_hex_to_bytes(w.clone()).to_vec())
                .collect(),
            None => vec![],
        };
        let witness: bitcoin::Witness = bitcoin::Witness::from_slice(witness.as_slice());

        bitcoin::TxIn {
            previous_output,
            script_sig,
            sequence,
            witness,
        }
    }
}

impl fmt::Display for TxInput {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TxInput({})", self.previous_output)
    }
}

#[derive(Default, Serialize, Deserialize, Tsify, Clone)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Transaction {
    /// The protocol version, is currently expected to be 1 or 2 (BIP 68).
    pub version: Option<i32>,
    /// Block height or timestamp. Transaction cannot be included in a block until this height/time.
    ///
    /// ### Relevant BIPs
    ///
    /// * [BIP-65 OP_CHECKLOCKTIMEVERIFY](https://github.com/bitcoin/bips/blob/master/bip-0065.mediawiki)
    /// * [BIP-113 Median time-past as endpoint for lock-time calculations](https://github.com/bitcoin/bips/blob/master/bip-0113.mediawiki)
    pub lock_time: Option<u32>, // bitcoin::absolute::LockTime

    /// List of transaction inputs.
    pub input: Option<Vec<TxInput>>,
    /// List of transaction outputs.
    pub output: Option<Vec<TxOutput>>,
}

impl From<Transaction> for bitcoin::Transaction {
    fn from(source: Transaction) -> bitcoin::Transaction {
        let version = match source.version {
            Some(version) => version,
            None => 2,
        };
        let lock_time = match source.lock_time {
            Some(lock_time) => match bitcoin::absolute::LockTime::from_time(lock_time) {
                Ok(result) => result,
                Err(_) => bitcoin::absolute::LockTime::ZERO,
            },
            None => bitcoin::absolute::LockTime::ZERO,
        };

        let input: Vec<bitcoin::TxIn> = match source.input {
            Some(input) => input
                .iter()
                .map(|tx_input| bitcoin::TxIn::from(tx_input.clone()))
                .collect(),
            None => vec![],
        };

        let output: Vec<bitcoin::TxOut> = match source.output {
            Some(output) => output
                .iter()
                .map(|tx_output| bitcoin::TxOut::from(tx_output.clone()))
                .collect(),
            None => vec![],
        };

        bitcoin::Transaction {
            version,
            lock_time,
            input,
            output,
        }
    }
}

impl From<bitcoin::Transaction> for Transaction {
    fn from(source: bitcoin::Transaction) -> Transaction {
        let lock_time = source.lock_time.to_consensus_u32();

        let input: Vec<TxInput> = source
            .input
            .iter()
            .map(|tx_in| TxInput::from(tx_in.clone()))
            .collect();

        let output: Vec<TxOutput> = source
            .output
            .iter()
            .map(|tx_out| TxOutput::from(tx_out.clone()))
            .collect();

        Transaction {
            version: Some(source.version),
            lock_time: Some(lock_time),
            input: Some(input),
            output: Some(output),
        }
    }
}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let input = match self.input.clone() {
            Some(input) => input
                .iter()
                .map(|tx_input| format!("  {}", tx_input))
                .collect::<Vec<String>>()
                .join("\n"),
            None => "".to_string(),
        };
        let output = match self.output.clone() {
            Some(output) => output
                .iter()
                .map(|tx_output| format!("  {}", tx_output))
                .collect::<Vec<String>>()
                .join("\n"),
            None => "".to_string(),
        };

        write!(
            f,
            "Transaction\nversion: {}\nlock_time: {}\ninputs:\n{}\noutputs:\n{})",
            self.version.unwrap_or_default(),
            self.lock_time.unwrap_or_default(),
            input,
            output
        )
    }
}
