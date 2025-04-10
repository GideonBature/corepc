// SPDX-License-Identifier: CC0-1.0

//! Tests for methods found under the `== Control ==` section of the API docs.

#![allow(non_snake_case)] // Test names intentionally use double underscore.

use integration_test::{Node, NodeExt as _, Wallet};
use node::vtype::*;             // All the version specific types.

#[test]
fn control__get_memory_info() {
    let node = Node::with_wallet(Wallet::None, &[]);
    let _: GetMemoryInfoStats = node.client.get_memory_info().unwrap();
}

#[cfg(not(feature = "v17"))]
#[test]
fn control__get_rpc_info() {
    let node = Node::with_wallet(Wallet::None, &[]);
    let _ = node.client.get_rpc_info().unwrap();
}

#[test]
fn control__help() {
    let node = Node::with_wallet(Wallet::None, &[]);
    let _ = node.client.help().unwrap();
}

#[test]
fn control__logging() {
    let node = Node::with_wallet(Wallet::None, &[]);
    let _: Logging = node.client.logging().unwrap();
}

#[test]
fn control__stop() {
    let node = Node::with_wallet(Wallet::None, &[]);
    let _ = node.client.stop().unwrap();
}

#[test]
fn control__uptime() {
    let node = Node::with_wallet(Wallet::None, &[]);
    let _ = node.client.uptime().unwrap();
}
