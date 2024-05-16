use super::ord::{
    fee_rate::FeeRate, inscription_id::InscriptionId, transaction_builder::Target,
    transaction_builder::TransactionBuilder,
};
use super::*;
use bitcoin::{
    address::{Address, NetworkUnchecked},
    Amount, OutPoint, TxOut,
};
use ordinals::SatPoint;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Default, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Input {
    pub output_value: Option<u64>,
    pub fee_rate: f64,
}

#[wasm_bindgen]
pub fn create_tx(input: Input) -> String {
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

    let recipient = "bc1pdqrcrxa8vx6gy75mfdfj84puhxffh4fq46h3gkp6jxdd0vjcsdyspfxcv6"
        .parse::<Address<NetworkUnchecked>>()
        .unwrap()
        .assume_checked();

    let change = [
        "bc1pxwww0ct9ue7e8tdnlmug5m2tamfn7q06sahstg39ys4c9f3340qqxrdu9k"
            .parse::<Address<NetworkUnchecked>>()
            .unwrap()
            .assume_checked(),
        "bc1prfc9mzpjsty087387x82fjmjpxzcvkap24f2w93y0dj8tr53g68s5ramkv"
            .parse::<Address<NetworkUnchecked>>()
            .unwrap()
            .assume_checked(),
    ];

    let fee_rate = FeeRate::try_from(input.fee_rate).unwrap();

    let target = match input.output_value {
        Some(output_value) => Target::Value(Amount::from_sat(output_value)),
        None => Target::Postage,
    };

    let locked_utxos: BTreeSet<OutPoint> = BTreeSet::new();
    let runic_utxos: BTreeSet<OutPoint> = BTreeSet::new();

    let builder = TransactionBuilder::new(
        satpoint,
        inscriptions,
        amounts,
        locked_utxos,
        runic_utxos,
        recipient,
        change,
        fee_rate,
        target,
    );

    match builder.build_transaction() {
        Ok(tx) => format!("{}", Transaction::from(tx)),
        Err(err) => throw_str(err.to_string().as_str()),
    }
}
