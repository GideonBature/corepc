// SPDX-License-Identifier: CC0-1.0

//! Tests for methods found under the `== Network ==` section of the API docs.

#![allow(non_snake_case)] // Test names intentionally use double underscore.

use integration_test::{AddNodeCommand, SetBanCommand};
use integration_test::{Node, NodeExt as _, Wallet};
use node::vtype::*;             // All the version specific types.
use node::mtype;

#[test]
fn network__get_added_node_info() {
    let node = Node::with_wallet(Wallet::None, &[]);
    let _: GetAddedNodeInfo = node.client.get_added_node_info().expect("getaddednodeinfo");
}

#[test]
fn network__get_net_totals() {
    let node = Node::with_wallet(Wallet::None, &[]);
    let _: GetNetTotals = node.client.get_net_totals().expect("getnettotals");
}

#[test]
fn network__get_network_info() {
    let node = Node::with_wallet(Wallet::None, &[]);
    let json: GetNetworkInfo = node.client.get_network_info().expect("getnetworkinfo");
    let model: Result<mtype::GetNetworkInfo, GetNetworkInfoError> = json.into_model();
    model.unwrap();

    // Server version is returned as part of the getnetworkinfo method.
    node.client.check_expected_server_version().expect("unexpected version");
}

#[test]
fn network__get_peer_info() {
    get_peer_info_one_node_network();
    get_peer_info_three_node_network();
}

fn get_peer_info_one_node_network() {
    let node = Node::with_wallet(Wallet::None, &[]);
    let json: GetPeerInfo = node.client.get_peer_info().expect("getpeerinfo");
    assert_eq!(json.0.len(), 0);
}

fn get_peer_info_three_node_network() {
    let (node1, node2, node3) = integration_test::three_node_network();

    // Just for good measure.
    node1.mine_a_block();
    node2.mine_a_block();
    node3.mine_a_block();

    // FIXME: Fails if we use equal to 2 ???
    assert!(node1.peers_connected() >= 1);
    assert!(node2.peers_connected() >= 1);
    assert!(node3.peers_connected() >= 1);
}

#[test]
fn network__add_node() {
    let node = Node::with_wallet(Wallet::None, &[]);

    let dummy_peer = "192.0.2.1:8333";

    #[cfg(any(
        feature = "v17",
        feature = "v18",
        feature = "v19",
        feature = "v20",
        feature = "v21",
        feature = "v22",
        feature = "v23",
        feature = "v24",
        feature = "v25"
    ))] {
        node.client.add_node(dummy_peer, AddNodeCommand::OneTry).expect("addnode onetry failed (v17-v25)");

        node.client.add_node(dummy_peer, AddNodeCommand::Add).expect("addnode add failed (v17-v25");

        node.client.add_node(dummy_peer, AddNodeCommand::Remove).expect("addnode remove failed (v17-v25");
    }

    #[cfg(any(
        feature = "v26",
        feature = "v27",
        feature = "v28"
    ))] {
        node.client.add_node(dummy_peer, AddNodeCommand::OneTry, None).expect("addnode onetry failed (v26+, v2transport=None)");

        node.client.add_node(dummy_peer, AddNodeCommand::Add, Some(false)).expect("addone add failed (v26+, v2transport=false)");

        node.client.add_node(dummy_peer, AddNodeCommand::Remove, Some(true)).expect("addnode remove failed (v26+, v2transport=true)");
    }
}

#[test]
fn network__clear_banned() {
    let node = Node::with_wallet(Wallet::Node, &[]);

    let dummy_ip = "192.0.2.2";

    match node.client.set_ban(dummy_ip, "add", Some(60), None) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("set_ban failed: {:?}", e))
    }

    node.client.clear_banned().expect("clearbanned RPC call failed");

    match node.client.list_banned() {
        Ok(banned_list) => {
            assert!(banned_list.o.is_empty(), "Banned list should be empty after clearbanned");
        },
        Err(e) => Err(format!("list_banned failed during verification: {:?}", e)),
    }
}

#[test]
fn network__set_ban() {
    let node = Node::with_wallet(Wallet::None, &[]);

    let subnet1 = "192.0.2.3";
    let subnet2 = "192.0.2.0/24";

    node.client.set_ban(subnet1, SetBanCommand::Add, None, None).expect("setban add default time failed");

    node.client.set_ban(subnet2, SetBanCommand::Add, Some(600), None).expect("setban add specific duration failed");

    let future_timestamp = (std::time::SystemTime::now() + std::time::Duration::from_secs(3600)).duration_since(std::time::UNIX_EPOCH).expect("Time went backwards").as_secs();

    node.client.set_ban(subnet1, SetBanCommand::Add, Some(future_timestamp as i64), Some(true)).expect("setban add absolute time failed");

    node.client.set_ban(subnet1, SetBanCommand::Add, None, Some(true)).expect("setban add absolute default time failed");

    node.client.set_ban(subnet1, SetBanCommand::Remove, None, None).expect("setban remove failed for subnet1");

    node.client.set_ban(subnet2, SetBanCommand::Remove, None, None).expect("setban remove failed for subnet2")
}
