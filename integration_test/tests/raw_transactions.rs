// SPDX-License-Identifier: CC0-1.0

//! Tests for methods found under the `== Rawtransactions ==` section of the API docs.

#![allow(non_snake_case)] // Test names intentionally use double underscore.

use std::collections::BTreeMap;

use bitcoin::{Amount, Sequence};
#[cfg(feature = "TODO")]
use bitcoin::{absolute, transaction, consensus, TxOut, Transaction};
use integration_test::{Node, NodeExt as _, Wallet};
use node::client::client_sync::Input;

#[test]
fn raw_transacitons__create_raw_transaction() {
    let node = Node::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    // Calls `createrawtransaction`.
    _create_and_send(&node);
}

#[test]
#[cfg(feature = "TODO")]
fn raw_transactions__fund_raw_transaction() {
    let node = Node::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    _create_fund_and_send(&node);
}

#[test]
fn raw_transactions__send_raw_transaction() {
    let node = Node::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    // Calls `sendrawtransaction`.
    _create_and_send(&node);
}

fn _create_and_send(node: &Node) {
    let (_addr, txid) = node.create_mempool_transaction(); // A million sats.
    node.mine_a_block();

    // We don't know what vout the UTXO is in.
    let tx_out = node
        .client
        .get_tx_out(txid, 0)
        .expect("gettxout")
        .into_model()
        .expect("GetTxOut into model")
        .tx_out;
    let spend_amount = Amount::from_sat(100_000);
    let fee = Amount::from_sat(1000);
    let change_amount = tx_out.value - spend_amount - fee;

    let sequence = Sequence::MAX; // Ignore locktime.
    let inputs = vec![Input { txid, vout: 0, sequence: Some(sequence) }];

    let mut outputs = BTreeMap::new();

    // Just send back to ourself.
    let spend_address = node.client.new_address().expect("failed to create new address");
    outputs.insert(spend_address.to_string(), spend_amount.to_btc());

    let json = node.client.get_raw_change_address().expect("getrawchangeaddress");
    let change_address = json.0;
    outputs.insert(change_address.to_string(), change_amount.to_btc());

    let json = node.client.create_raw_transaction(&inputs, &outputs).expect("createrawtransaction");

    // This method is from the wallet section.
    let json = node
        .client
        .sign_raw_transaction_with_wallet(&json.0)
        .expect("signrawtransactionwithwallet");

    // The proves we did everything correctly.
    let model = json.clone().into_model().expect("SignRawTransactionWithWalet into model");
    let _ = node.client.send_raw_transaction(&model.raw_transaction).expect("createrawtransaction");
}

#[cfg(feature = "TODO")]
fn _create_fund_and_send(node: &Node) {
    let (_addr, txid) = node.create_mempool_transaction(); // A million sats.
    node.mine_a_block();

    // We don't know what vout the UTXO is in.
    let tx_out = node
        .client
        .get_tx_out(txid, 0)
        .expect("gettxout")
        .into_model()
        .expect("GetTxOut into model")
        .tx_out;
    let spend_amount = Amount::from_sat(100_000);
    let fee = Amount::from_sat(1000);
    let change_amount = tx_out.value - spend_amount - fee;

    // Just send back to ourself.
    let spend_address = node.client.new_address().expect("failed to get new address");
    let change_address = node
        .client
        .get_raw_change_address()
        .expect("getrawchangeaddress")
        .into_model()
        .expect("GetRawChangeAddress into model")
        .0
        .assume_checked();

    let spend = TxOut { value: spend_amount, script_pubkey: spend_address.script_pubkey() };
    let change = TxOut { value: change_amount, script_pubkey: change_address.script_pubkey() };

    let tx = Transaction {
        version: transaction::Version::TWO,
        lock_time: absolute::LockTime::ZERO,
        input: vec![],
        output: vec![spend, change],
    };
    let _ = consensus::encode::serialize_hex(&tx);

    // TODO: This errors with: RpcError { code: -22, message: "TX decode failed", data: None }
    // let json = node.client.fund_raw_transaction(&tx).expect("fundrawtransaction");
    // let model = json.into_model().expect("FundRawTransaction into model");
    // let funded = consensus::encode::serialize_hex(&model.tx);

    // // This method is from the wallet section.
    // let json = node.client.sign_raw_transaction_with_wallet(&funded).expect("signrawtransactionwithwallet");

    // // The proves we did everything correctly.
    // let model = json.clone().into_model().expect("SignRawTransactionWithWalet into model");
    // let _ = node.client.send_raw_transaction(&model.raw_transaction).expect("createrawtransaction");
}

#[test]
#[cfg(feature = "v28")]
fn raw_transactions__submitpackage() {
    let node = Node::with_wallet(Wallet::Default, &[]);

    // Submitting the empty package should simply fail.
    assert!(node.client.submit_package(&[], None, None).is_err());

    node.fund_wallet();

    let (_, tx_0) = node.create_mined_transaction();
    let (_, tx_1) = node.create_mined_transaction();

    // The call for submitting this package should succeed, but yield an 'already known'
    // error for all transactions.
    let res = node
        .client
        .submit_package(&[tx_0, tx_1], None, None)
        .expect("failed to submit package")
        .into_model()
        .expect("failed to submit package");
    for (_, tx_result) in &res.tx_results {
        assert!(tx_result.error.is_some());
    }
    assert!(res.replaced_transactions.is_empty());
}
