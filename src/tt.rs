use arbitrary::Arbitrary;
use bitcoin::{
    address::{Address, NetworkUnchecked},
    Amount, OutPoint, TxOut,
};
use ord::{FeeRate, InscriptionId, Target, TransactionBuilder};
use ordinals::SatPoint;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use tsify::Tsify;
use wasm_bindgen::prelude::*;

#[derive(Default, Serialize, Deserialize, Tsify, Arbitrary)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Input {
    pub output_value: Option<u64>,
    pub fee_rate: f64,
    pub utxos: Vec<u64>,
}

#[wasm_bindgen]
pub fn create_tx(input: Input) {
    let outpoint = "1111111111111111111111111111111111111111111111111111111111111111:1"
        .parse::<OutPoint>()
        .unwrap();

    let satpoint = "1111111111111111111111111111111111111111111111111111111111111111:1:0"
        .parse::<SatPoint>()
        .unwrap();

    let inscription_id = "1111111111111111111111111111111111111111111111111111111111111111i1"
        .parse::<InscriptionId>()
        .unwrap();

    let mut inscriptions = BTreeMap::new();
    inscriptions.insert(satpoint, vec![inscription_id]);

    let address = "bc1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4"
        .parse::<Address<NetworkUnchecked>>()
        .unwrap()
        .assume_checked();
    let mut amounts = BTreeMap::new();
    amounts.insert(
        outpoint,
        TxOut {
            value: 50_000,
            script_pubkey: address.script_pubkey(),
        },
    );

    // let mut amounts = BTreeMap::new();
    // amounts.insert(outpoint, bitcoin::Amount::from_sat(1_000_000));
    // for (i, value) in input.utxos.into_iter().enumerate() {
    //     amounts.insert(
    //         format!("0000000000000000000000000000000000000000000000000000000000000000:{i}",)
    //             .parse()
    //             .unwrap(),
    //         bitcoin::Amount::from_sat(value),
    //     );
    // }

    let recipient = "bc1pdqrcrxa8vx6gy75mfdfj84puhxffh4fq46h3gkp6jxdd0vjcsdyspfxcv6"
        .parse::<Address<NetworkUnchecked>>()
        .unwrap()
        .assume_checked();

    let change = [
        "bc1pxwww0ct9ue7e8tdnlmug5m2tamfn7q06sahstg39ys4c9f3340qqxrdu9k"
            .parse::<Address<NetworkUnchecked>>()
            .unwrap()
            .assume_checked(),
        "bc1pxwww0ct9ue7e8tdnlmug5m2tamfn7q06sahstg39ys4c9f3340qqxrdu9k"
            .parse::<Address<NetworkUnchecked>>()
            .unwrap()
            .assume_checked(),
    ];

    let Ok(fee_rate) = FeeRate::try_from(input.fee_rate) else {
        return;
    };

    let target = match input.output_value {
        Some(output_value) => Target::Value(Amount::from_sat(output_value)),
        None => Target::Postage,
    };

    let locked_utxos: BTreeSet<OutPoint> = BTreeSet::new();
    let runic_utxos: BTreeSet<OutPoint> = BTreeSet::new();

    let _ = TransactionBuilder::new(
        satpoint,
        inscriptions,
        amounts,
        locked_utxos,
        runic_utxos,
        recipient,
        change,
        fee_rate,
        target,
    )
    .build_transaction();
}
