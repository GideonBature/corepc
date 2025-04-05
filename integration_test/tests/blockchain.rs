// SPDX-License-Identifier: CC0-1.0

//! Tests for methods found under the `== Blockchain ==` section of the API docs.

use integration_test::{Node, NodeExt as _, Wallet};

#[test]
fn get_blockchain_info() {
    let node = Node::with_wallet(Wallet::None, &[]);
    let json = node.client.get_blockchain_info().expect("getblockchaininfo");
    assert!(json.into_model().is_ok());
}

#[test]
fn get_best_block_hash() {
    let node = Node::with_wallet(Wallet::None, &[]);
    let json = node.client.get_best_block_hash().expect("getbestblockhash");
    assert!(json.into_model().is_ok());
}

#[test]
fn get_block() {
    let node = Node::with_wallet(Wallet::None, &[]);
    let block_hash = node.client.best_block_hash().expect("best_block_hash failed");

    let json = node.client.get_block_verbose_zero(block_hash).expect("getblock verbose=0");
    assert!(json.into_model().is_ok());

    let json = node.client.get_block_verbose_one(block_hash).expect("getblock verbose=1");
    assert!(json.into_model().is_ok());

    // TODO: Test getblock 2
    // let json = node.client.get_block_with_verbosity(block_hash, 2).expect("getblock verbosity 2");
    // assert!(json.into_model().is_ok());
}

#[test]
fn get_block_count() {
    let node = Node::with_wallet(Wallet::None, &[]);
    let json = node.client.get_block_count().expect("getblockcount");
    let _ = json.into_model();
}

#[test]
#[cfg(not(feature = "v17"))]
#[cfg(not(feature = "v18"))]
fn get_block_filter() {
    let node = Node::with_wallet(Wallet::Default, &["-blockfilterindex"]);
    node.mine_a_block();
    let hash = node.client.best_block_hash().expect("best_block_hash failed");

    let json = node.client.get_block_filter(hash).expect("getblockfilter");
    let _ = json.into_model();
}

#[test]
fn get_block_hash() {
    let node = Node::with_wallet(Wallet::None, &[]);
    let json = node.client.get_block_hash(0).expect("getblockhash");
    assert!(json.into_model().is_ok());
}

#[test]
fn get_block_header() {
    // verbose = false
    let node = Node::with_wallet(Wallet::None, &[]);
    let block_hash = node.client.best_block_hash().expect("best_block_hash failed");
    let json = node.client.get_block_header(&block_hash).expect("getblockheader");
    assert!(json.into_model().is_ok());
}

#[test]
fn get_block_header_verbose() {
    // verbose = true
    let node = Node::with_wallet(Wallet::None, &[]);
    let block_hash = node.client.best_block_hash().expect("best_block_hash failed");
    let json = node.client.get_block_header_verbose(&block_hash).expect("getblockheader");
    assert!(json.into_model().is_ok());
}

#[test]
fn get_block_stats() {
    // Version 18 cannot getblockstats if -txindex is not enabled.
    #[cfg(not(feature = "v18"))]
    getblockstats();

    // All non-feature gated versions including 18 can getblockstats if -txindex is enabled.
    getblockstats_txindex();
}

#[cfg(not(feature = "v18"))]
fn getblockstats() {
    let node = Node::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();

    let json = node.client.get_block_stats_by_height(1).expect("getblockstats");
    assert!(json.into_model().is_ok());

    let block_hash = node.client.best_block_hash().expect("best_block_hash failed");
    let json = node.client.get_block_stats_by_block_hash(&block_hash).expect("getblockstats");
    assert!(json.into_model().is_ok());
}

fn getblockstats_txindex() {
    let node = Node::with_wallet(Wallet::Default, &["-txindex"]);
    node.fund_wallet();

    let json = node.client.get_block_stats_by_height(101).expect("getblockstats");
    assert!(json.into_model().is_ok());

    let block_hash = node.client.best_block_hash().expect("best_block_hash failed");
    let json = node.client.get_block_stats_by_block_hash(&block_hash).expect("getblockstats");
    assert!(json.into_model().is_ok());
}

#[test]
fn get_chain_tips() {
    let node = Node::with_wallet(Wallet::None, &[]);
    let json = node.client.get_chain_tips().expect("getchaintips");
    assert!(json.into_model().is_ok());
}

#[test]
fn get_chain_tx_stats() {
    let node = Node::with_wallet(Wallet::None, &[]);
    let json = node.client.get_chain_tx_stats().expect("getchaintxstats");
    assert!(json.into_model().is_ok());
}

#[test]
fn get_difficulty() {
    let node = Node::with_wallet(Wallet::None, &[]);
    let json = node.client.get_difficulty().expect("getdifficulty");
    let _ = json.into_model();
}

#[test]
#[cfg(feature = "TODO")]
fn get_mempool_ancestors() {
    // We can probably get away with not testing this because it returns the
    // same type as `getmempoolentry` which is tested below.
}

#[test]
#[cfg(feature = "TODO")]
fn get_mempool_descendants() {
    // We can probably get away with not testing this because it returns the
    // same type as `getmempoolentry` which is tested below.
}

#[test]
fn get_mempool_entry() {
    let node = Node::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    let (_address, txid) = node.create_mempool_transaction();

    let json = node.client.get_mempool_entry(txid).expect("getmempoolentry");
    assert!(json.into_model().is_ok());
}

#[test]
fn get_mempool_info() {
    let node = Node::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    let (_address, _txid) = node.create_mempool_transaction();

    // Test the type and into model conversion code.
    let json = node.client.get_mempool_info().expect("getmempoolinfo");
    let info = json.into_model().expect("into_model");
    // Sanity check.
    assert_eq!(info.size, 1);
}

#[test]
fn get_raw_mempool() {
    let node = Node::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    let (_address, _txid) = node.create_mempool_transaction();

    // Test the type and into model conversion code.
    let json = node.client.get_raw_mempool().expect("getrawmempool");
    let mempool = json.into_model().expect("into_model");
    // Sanity check.
    assert_eq!(mempool.0.len(), 1);
}

#[test]
// FIXME: Fails with getrawmempool verbose: JsonRpc(Json(Error("invalid type: map, expected a sequence", line: 1, column: 0)))
#[cfg(feature = "TODO")]
fn get_raw_mempool_verbose() {
    let node = Node::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    let (_address, _txid) = node.create_mempool_transaction();

    // Test the type and into model conversion code.
    let json = node.client.get_raw_mempool_verbose().expect("getrawmempool verbose");
    let mempool = json.into_model().expect("into_model");
    // Sanity check.
    assert_eq!(mempool.0.len(), 1);
}

#[test]
fn get_tx_out() {
    let node = Node::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    let (_address, tx) = node.create_mined_transaction();
    let txid = tx.compute_txid();

    // Test the type and into model conversion code.
    let json = node.client.get_tx_out(txid, 1).expect("gettxout");
    let _ = json.into_model().expect("into_model");
}

#[test]
fn get_tx_out_proof() {
    let node = Node::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    let (_address, tx) = node.create_mined_transaction();
    let txid = tx.compute_txid();

    let _ = node.client.get_tx_out_proof(&[txid]).expect("gettxoutproof");
}

#[test]
fn get_tx_out_set_info() {
    let node = Node::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    let (_address, _tx) = node.create_mined_transaction();

    // Test the type and into model conversion code.
    let json = node.client.get_tx_out_set_info().expect("gettxoutsetinfo");
    let _ = json.into_model().expect("into_model");
}

#[test]
fn precious_block() {
    let node = Node::with_wallet(Wallet::Default, &[]);
    node.mine_a_block();
    let hash = node.client.best_block_hash().expect("best_block_hash failed");
    node.mine_a_block();

    let _ = node.client.precious_block(hash).expect("preciousblock");
}

// Implicitly tests the omitted method `gettxoutproof` as well.
#[test]
fn verify_tx_out_proof() {
    let node = Node::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    let (_address, tx) = node.create_mined_transaction();
    let txid = tx.compute_txid();

    let proof = node.client.get_tx_out_proof(&[txid]).expect("gettxoutproof");

    let txids = node.client.verify_tx_out_proof(&proof).expect("verifytxoutproof");
    assert_eq!(txids.0.len(), 1);
}

// Implicitly tests the omitted method `pruneblockchain`
#[test]
fn prune_blockchain() {
    let node = Node::with_wallet(Wallet::Default, &["-prune=550"]);
    // Generate 1001 blocks to exceed regtest's nPruneAfterHeight (1000)
    let num_blocks_to_mine = 1001;
    // (Using a loop with mine_a_block is necessary as generate() is deprecated in v28)
    for _ in 0..num_blocks_to_mine {
        node.mine_a_block();
    }

    // Verify block count
    let count = node.client.get_block_count().expect("getblockcount").0;
    assert!(
        count >= num_blocks_to_mine as u64,
        "Failed to mine enough blocks. Expected: {}, Got: {}",
        num_blocks_to_mine,
        count
    );

    let target_height: u64 = 500;

    let _ = node.client
        .prune_blockchain(target_height)
        .expect("pruneblockchain RPC call failed");
}