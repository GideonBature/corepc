// SPDX-License-Identifier: CC0-1.0

//! Tests for methods found under the `== Wallet ==` section of the API docs.

#![allow(non_snake_case)] // Test names intentionally use double underscore.

#[cfg(feature = "TODO")]
use bitcoin::address::{Address, NetworkChecked};
use bitcoin::Amount;
use integration_test::{Node, NodeExt as _, Wallet};
use node::AddressType;
use node::vtype::*;             // All the version specific types.
use node::mtype;

#[test]
#[cfg(feature = "TODO")]
fn wallet__add_multisig_address__modelled() {
    let nrequired = 1; // 1-of-2 multisig.

    let add1: Address<NetworkChecked> =
        "32iVBEu4dxkUQk9dJbZUiBiQdmypcEyJRf".parse::<Address<_>>().unwrap().assume_checked();
    let add2: Address<NetworkChecked> =
        "132F25rTsvBdp9JzLLBHP5mvGY66i1xdiM".parse::<Address<_>>().unwrap().assume_checked();

    let node = Node::with_wallet(Wallet::Default, &[]);
    let json: AddMultisigAddress = node
        .client
        .add_multisig_address_with_addresses(nrequired, vec![add1, add2])
        .expect("addmultisigaddress");
    let model: Result<AddMultisigAddress, AddMultisigAddressError> = json.into_model();
    model.unwrap();
}

#[test]
fn wallet__bump_fee__modelled() {
    let node = Node::with_wallet(Wallet::Default, &[]);
    let address = node.client.new_address().expect("failed to create new address");
    let _ = node.client.generate_to_address(101, &address).expect("generatetoaddress");

    let txid = node
        .client
        .send_to_address_rbf(&address, Amount::from_sat(10_000))
        .expect("sendtoaddress")
        .txid()
        .unwrap();

    let json: BumpFee = node.client.bump_fee(txid).expect("bumpfee");
    let model: Result<mtype::BumpFee, BumpFeeError> = json.into_model();
    model.unwrap();
}

#[test]
fn wallet__create_wallet__modelled() {
    // Implicitly tests `createwallet` because we create the default wallet.
    let _ = Node::with_wallet(Wallet::Default, &[]);
}

#[test]
fn wallet__dump_priv_key__modelled() {
    // As of Core v23 the default wallet is an native descriptor wallet which does not
    // support dumping private keys. Legacy wallets are supported upto v25 it seems.
    #[cfg(any(
        feature = "v23",
        feature = "v24",
        feature = "v25",
    ))]
    {
        let node = Node::with_wallet(Wallet::None, &[]);

        node.client.create_legacy_wallet("legacy_wallet").expect("legacy create_wallet");
        let address = node.client.get_new_address(Some("label"), Some(AddressType::Legacy)).expect("legacy get_new_address");
        let address = address.into_model().unwrap().0.assume_checked();

        let json: DumpPrivKey = node.client.dump_priv_key(&address).expect("dumpprivkey");
        let model: Result<mtype::DumpPrivKey, _> = json.into_model();
        model.unwrap();
    }

    #[cfg(any(
        feature = "v17",
        feature = "v18",
        feature = "v19",
        feature = "v20",
        feature = "v21",
        feature = "v22",
    ))]
    {
        let node = Node::with_wallet(Wallet::Default, &[]);
        let address = node.client.new_address().expect("failed to get new address");

        let json: DumpPrivKey = node.client.dump_priv_key(&address).expect("dumpprivkey");
        let model: Result<mtype::DumpPrivKey, _> = json.into_model();
        model.unwrap();
    }
}

#[test]
fn wallet__dump_wallet__modelled() {
    // As of Core v23 the default wallet is an native descriptor wallet which does not
    // support dumping private keys. Legacy wallets are supported upto v25 it seems.
    #[cfg(any(
        feature = "v23",
        feature = "v24",
        feature = "v25",
    ))]
    {
        let node = Node::with_wallet(Wallet::None, &[]);

        node.client.create_legacy_wallet("legacy_wallet").expect("legacy create_wallet");
        let out = integration_test::random_tmp_file();

        let json: DumpWallet = node.client.dump_wallet(&out).expect("dumpwallet");
        let _: mtype::DumpWallet = json.into_model();
    }

    #[cfg(any(
        feature = "v17",
        feature = "v18",
        feature = "v19",
        feature = "v20",
        feature = "v21",
        feature = "v22",
    ))]
    {
        let node = Node::with_wallet(Wallet::Default, &[]);
        let out = integration_test::random_tmp_file();

        let json: DumpWallet = node.client.dump_wallet(&out).expect("dumpwallet");
        let _: mtype::DumpWallet = json.into_model();
    }
}

#[test]
fn wallet__get_addresses_by_label__modelled() {
    let node = Node::with_wallet(Wallet::Default, &[]);
    let label = "some-label";
    let addr = node.client.new_address_with_label(label).expect("failed to get new address");

    let json: GetAddressesByLabel = node.client.get_addresses_by_label(label).expect("getaddressesbylabel");
    let model: Result<mtype::GetAddressesByLabel, _> = json.into_model();
    let map = model.unwrap();

    // sanity checks.
    assert!(!map.0.is_empty());
    assert!(map.0.get(&addr).is_some());
}

#[test]
#[cfg(feature = "TODO")]        // FIXME: The types are broken.
// TODO: Consider testing a few different address types.
fn wallet__get_address_info__modelled() {
    let node = Node::with_wallet(Wallet::Default, &[]);
    let address = node.client.new_address().expect("failed to create new address");

    let json: GetAddressInfo = node.client.get_address_info(&address).expect("getaddressinfo");
    let model: Result<mtype::GetAddressInfo, GetAddressInfoError> = json.into_model();
    model.unwrap();
}

#[test]
fn wallet__get_balance__modelled() {
    let node = Node::with_wallet(Wallet::Default, &[]);

    let json: GetBalance = node.client.get_balance().expect("getbalance");
    let model: Result<mtype::GetBalance, _> = json.into_model();
    model.unwrap();

    // Check non-zero balance just for giggles.
    node.fund_wallet();
    let json = node.client.get_balance().expect("getbalance");
    json.into_model().unwrap();
}


#[test]
#[cfg(all(not(feature = "v17"), not(feature = "v18")))]
fn wallet__get_balances() {
    let node = Node::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();

    let json: GetBalances = node.client.get_balances().expect("getbalances");
    let model: Result<mtype::GetBalances, _> = json.into_model();
    model.unwrap();
}

#[test]
fn wallet__get_new_address__modelled() {
    let node = Node::with_wallet(Wallet::Default, &[]);

    // Implicitly tests `getnewaddress`.
    let _ = node.client.new_address().unwrap();

    // Exhaustively test address types with helper.
    let _ = node.client.new_address_with_type(AddressType::Legacy).unwrap();
    let _ = node.client.new_address_with_type(AddressType::P2shSegwit).unwrap();
    let _ = node.client.new_address_with_type(AddressType::Bech32).unwrap();
}

#[test]
fn wallet__get_raw_change_address__modelled() {
    let node = Node::with_wallet(Wallet::Default, &[]);
    let json: GetRawChangeAddress = node.client.get_raw_change_address().expect("getrawchangeaddress");
    let model: Result<mtype::GetRawChangeAddress, _> = json.into_model();
    model.unwrap();
}

#[test]
fn wallet__get_received_by_address__modelled() {
    let amount = Amount::from_sat(10_000);

    let node = Node::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    let address = node.client.new_address().expect("failed to create new address");

    let _txid =
        node.client.send_to_address(&address, amount).expect("sendtoaddress").txid().unwrap();
    node.mine_a_block();

    let json: GetReceivedByAddress = node.client.get_received_by_address(&address).expect("getreceivedbyaddress");
    let model: Result<mtype::GetReceivedByAddress, _> = json.into_model();
    let model = model.unwrap();

    assert_eq!(model.0, amount);
}

#[test]
fn wallet__get_transaction__modelled() {
    let node = Node::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    let address = node.client.new_address().expect("failed to create new address");

    let txid = node
        .client
        .send_to_address(&address, Amount::from_sat(10_000))
        .expect("sendtoaddress")
        .txid()
        .unwrap();

    let json: GetTransaction = node.client.get_transaction(txid).expect("gettransaction");
    let model: Result<mtype::GetTransaction, GetTransactionError> = json.into_model();
    model.unwrap();
}

#[test]
fn wallet__load_wallet__modelled() {
    create_load_unload_wallet();
}

#[test]
fn wallet__unload_wallet() {
    create_load_unload_wallet();
}

#[test]
fn wallet__send_to_address__modelled() {
    let node = Node::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    let address = node.client.new_address().expect("failed to create new address");

    let json: SendToAddress =
        node.client.send_to_address(&address, Amount::from_sat(10_000)).expect("sendtddress");
    let model: Result<mtype::SendToAddress, _> = json.into_model();
    model.unwrap();
}

fn create_load_unload_wallet() {
    let node = Node::with_wallet(Wallet::None, &[]);

    let wallet = format!("wallet-{}", rand::random::<u32>()).to_string();
    node.client.create_wallet(&wallet).expect("failed to create wallet");

    // Upto version 20 Core returns null for `unloadwallet`.
    #[cfg(any(feature = "v17", feature = "v18", feature = "v19", feature = "v20"))]
    let _ = node.client.unload_wallet(&wallet).expect("unloadwallet");

    // From version 21 Core returns warnings for `unloadwallet`.
    #[cfg(all(not(feature = "v17"), not(feature = "v18"), not(feature = "v19"), not(feature = "v20")))]
    {
        let json: UnloadWallet = node.client.unload_wallet(&wallet).expect("unloadwallet");
        let _: mtype::UnloadWallet = json.into_model();
    }

    let _: LoadWallet = node.client.load_wallet(&wallet).expect("loadwallet");
}
