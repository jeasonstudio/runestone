use super::tx_input::TxInput;
use super::tx_output::TxOutput;
use serde::{Deserialize, Serialize};
use tsify::Tsify;

#[derive(Default, Serialize, Deserialize, Tsify)]
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
    pub input: Vec<TxInput>,
    /// List of transaction outputs.
    pub output: Vec<TxOutput>,
}

impl Transaction {
    pub fn to_source(&self) -> bitcoin::Transaction {
        let version = match self.version {
            Some(version) => version,
            None => 2,
        };
        let lock_time = match self.lock_time {
            Some(lock_time) => match bitcoin::absolute::LockTime::from_time(lock_time) {
                Ok(result) => result,
                Err(_) => bitcoin::absolute::LockTime::ZERO,
            },
            None => bitcoin::absolute::LockTime::ZERO,
        };
        let input = self
            .input
            .iter()
            .map(|tx_input| tx_input.to_source())
            .collect();

        let output = self
            .output
            .iter()
            .map(|tx_output| tx_output.to_source())
            .collect();

        bitcoin::Transaction {
            version,
            lock_time,
            input,
            output,
        }
    }
}
