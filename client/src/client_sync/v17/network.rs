// SPDX-License-Identifier: CC0-1.0

//! Macros for implementing JSON-RPC methods on a client.
//!
//! Requires `Client` to be in scope.
//!
//! Specifically this is methods found under the `== Network ==` section of the
//! API docs of Bitcoin Core `v0.17`.
//!
//! See, or use the `define_jsonrpc_minreq_client!` macro to define a `Client`.

/// Implements Bitcoin Core JSON-RPC API method `addnode`
#[macro_export]
macro_rules! impl_client_v17__addnode {
    () => {
        impl Client {
            pub fn add_node(&self, node: &str, command: AddNodeCommand) -> Result<()> {
                match self.call("addnode", &[into_json(node)?, into_json(command)?]) {
                    Ok(serde_json::Value::Null) => Ok(()),
                    Ok(res) => Err(Error::Returned(res.to_string())),
                    Err(err) => Err(err.into()),
                }
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `clearbanned`
#[macro_export]
macro_rules! impl_client_v17__clearbanned {
    () => {
        impl Client {
            pub fn clear_banned(&self) -> Result<()> {
                match self.call("clearbanned", &[]) {
                    Ok(serde_json::Value::Null) => Ok(()),
                    Ok(res) => Err(Error::Returned(res.to_string())),
                    Err(err) => Err(err.into()),
                }
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getaddednodeinfo`
#[macro_export]
macro_rules! impl_client_v17__getaddednodeinfo {
    () => {
        impl Client {
            pub fn get_added_node_info(&self) -> Result<GetAddedNodeInfo> {
                self.call("getaddednodeinfo", &[])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getnettotals`
#[macro_export]
macro_rules! impl_client_v17__getnettotals {
    () => {
        impl Client {
            pub fn get_net_totals(&self) -> Result<GetNetTotals> { self.call("getnettotals", &[]) }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getnetworkinfo`
#[macro_export]
macro_rules! impl_client_v17__getnetworkinfo {
    () => {
        impl Client {
            /// Returns the server version field of `GetNetworkInfo`.
            pub fn server_version(&self) -> Result<usize> {
                let info = self.get_network_info()?;
                Ok(info.version)
            }

            pub fn get_network_info(&self) -> Result<GetNetworkInfo> {
                self.call("getnetworkinfo", &[])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getpeerinfo`
#[macro_export]
macro_rules! impl_client_v17__getpeerinfo {
    () => {
        impl Client {
            pub fn get_peer_info(&self) -> Result<GetPeerInfo> { self.call("getpeerinfo", &[]) }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `setban`
#[macro_export]
macro_rules! impl_client_v17__setban {
    () => {
        impl Client {
            pub fn set_ban(&self, subnet: &str, command: SetBanCommand) -> Result<()> {
                match self.call("setban", &[into_json(subnet)?, into_json(command)?]) {
                    Ok(serde_json::Value::Null) => Ok(()),
                    Ok(res) => Err(Error::Returned(res.to_string())),
                    Err(err) => Err(err.into()),
                }
            }
        }
    };
}
