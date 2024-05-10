use {
    self::{fee_rate::FeeRate, inscription_id::InscriptionId},
    anyhow::{bail, Error},
    bitcoin::{
        address::Address, blockdata::locktime::absolute::LockTime, hashes::Hash, Amount, OutPoint,
        ScriptBuf, Sequence, Transaction, TxIn, TxOut, Txid, Witness,
    },
    ordinals::SatPoint,
    serde::{Deserialize, Deserializer},
    serde_with::{DeserializeFromStr, SerializeDisplay},
    std::{
        cmp::{max, min},
        collections::{BTreeMap, BTreeSet},
        fmt::{self, Display, Formatter},
        str::FromStr,
    },
};

const TARGET_POSTAGE: Amount = Amount::from_sat(10_000);

#[macro_use]
pub mod macros;

pub mod fee_rate;
pub mod inscription_id;
pub mod transaction_builder;
