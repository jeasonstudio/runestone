use bitcoin::hashes::Hash;
use serde::{Deserialize, Serialize};
use tsify::Tsify;

#[derive(Default, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct OutPoint {
    /// The referenced transaction's txid.
    pub txid: Vec<u8>,
    /// The index of the referenced output in its transaction's vout.
    pub vout: u32,
}

impl OutPoint {
    pub fn to_source(&self) -> bitcoin::OutPoint {
        let hash = match bitcoin::hashes::sha256d::Hash::from_slice(&self.txid) {
            Ok(hash) => hash,
            Err(_) => bitcoin::hashes::sha256d::Hash::from_byte_array([0; 32]),
        };
        let txid = bitcoin::Txid::from_raw_hash(hash);
        bitcoin::OutPoint {
            txid,
            vout: self.vout,
        }
    }
}

#[derive(Default, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Witness {
    /// Contains the witness `Vec<Vec<u8>>` serialization without the initial varint indicating the
    /// number of elements (which is stored in `witness_elements`).
    pub content: Vec<u8>,

    /// The number of elements in the witness.
    ///
    /// Stored separately (instead of as a VarInt in the initial part of content) so that methods
    /// like [`Witness::push`] don't have to shift the entire array.
    pub witness_elements: usize,

    /// This is the valid index pointing to the beginning of the index area. This area is 4 *
    /// stack_size bytes at the end of the content vector which stores the indices of each item.
    pub indices_start: usize,
}

#[derive(Default, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct TxInput {
    /// The reference to the previous output that is being used an an input.
    pub previous_output: OutPoint,
    /// The script which pushes values on the stack which will cause
    /// the referenced output's script to be accepted.
    pub script_sig: Vec<u8>,
    /// The sequence number, which suggests to miners which of two
    /// conflicting transactions should be preferred, or 0xFFFFFFFF
    /// to ignore this feature. This is generally never used since
    /// the miner behaviour cannot be enforced.
    pub sequence: u32,
    /// Witness data: an array of byte-arrays.
    /// Note that this field is *not* (de)serialized with the rest of the TxIn in
    /// Encodable/Decodable, as it is (de)serialized at the end of the full
    /// Transaction. It *is* (de)serialized with the rest of the TxIn in other
    /// (de)serialization routines.
    pub witness: Witness,
}

impl TxInput {
    pub fn to_source(&self) -> bitcoin::TxIn {
        let previous_output = self.previous_output.to_source();
        let script_sig = bitcoin::ScriptBuf::from_bytes(self.script_sig.to_vec());
        let sequence = bitcoin::Sequence::from_consensus(self.sequence);
        let witness = bitcoin::Witness::new(); // TODO: ignored
        bitcoin::TxIn {
            previous_output,
            script_sig,
            sequence,
            witness,
        }
    }
}
