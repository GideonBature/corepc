// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core `v0.18` - control.
//!
//! Types for methods found under the `== Control ==` section of the API docs.

use serde::{Deserialize, Serialize};

/// Result of JSON-RPC method `getrpcinfo`.
///
/// > getrpcinfo
/// >
/// > Returns details of the RPC server.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetRpcInfo (
    pub Vec<ActiveCommand>,
);

/// Information about an active command - return as part of `getrpcinfo`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ActiveCommand {
    /// The name of the RPC command.
    pub method: String,
    /// The running time in microseconds.
    pub duration: u64,
}
