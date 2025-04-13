// SPDX-License-Identifier: CC0-1.0

//! Macros for implementing JSON-RPC methods on a client.
//!
//! Requires `Client` to be in scope.
//!
//! Specifically this is methods found under the `== Network ==` section of the
//! API docs of Bitcoin Core `v0.17`.
//!
//! See, or use the `define_jsonrpc_minreq_client!` macro to define a `Client`.

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

/// Implements Bitcoin Core JSON-RPC API method `addnode`
#[macro_export]
macro_rules! impl_client_v17__addnode {
    () => {
        impl Client {
            pub fn add_node(&self, node: &str, command: AddNodeCommand) -> Result<()> {
                let params = &[node.into(), serde_json::to_value(command)?];

                match self.call("addnode", params) {
                    Ok(serde_json::Value::Null) => Ok(()),
                    Ok(ref val) if val.is_null() => Ok(()),
                    Ok(other) => {
                        Err(crate::client_sync::Error::Returned(format!(
                            "addnode expected null, got: {}", other
                        )))
                    },
                    Err(e) => Err(e.into()),
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
                    Ok(ref val) if val.is_null() => Ok(()),
                    Ok(other) => Err(crate::client_sync::Error::Returned(format!("clearbanned expected null, got: {}", other))),
                    Err(e) => Err(e.into()),
                }
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `setban`
#[macro_export]
macro_rules! impl_client_v17__setban {
    () => {
        impl Client {
            pub fn set_ban(
                &self,
                subnet: &str,
                command: SetBanCommand,
                bantime: Option<i64>,
                absolute: Option<bool>,
            ) -> Result<()> {
                let mut params: Vec<serde_json::Value> = vec![subnet.into(), serde_json::to_value(command)?,];

                if bantime.is_some() || absolute.is_some() {
                    params.push(bantime.map_or(serde_json::Value::Null, |t| t.into()));

                    if let Some(abs) = absolute {
                        params.push(abs.into());
                    }
                }

                match self.call("setban", &params) {
                    Ok(serde_json::Value::Null) => Ok(()),
                    Ok(ref val) if val.is_null() => Ok(()),
                    Ok(other) => {
                        Err(crate::client_sync::Error::Returned(format!("setban expected null, got: {}", other)))
                    },
                    Err(e) => Err(e.into()),
                }
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `listbanned`
#[macro_export]
macro_rules! impl_client_v17__listbanned {
    () => {
        impl Client {
            pub fn list_banned(&self) -> Result<ListBanned> {
                self.call("listbanned", &[])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `disconnectnode`
#[macro_export]
macro_rules! impl_client_v17__disconnectnode {
    () => {
        impl Client {
            pub fn disconnect_node(
                &self,
                address: Option<&str>,
                nodeid: Option<u64>,
            ) -> Result<()> {
                let params: Vec<serde_json::Value> = match (address, nodeid) {
                    (Some(addr), None) => {
                        vec![addr.into()]
                    }
                    (None, Some(id)) => {
                        vec![serde_json::Value::String(String::new()), id.into()]
                    }
                    (Some(_), Some(_)) => {
                        return Err(crate::client_sync::Error::DisconnectNodeArgsBoth);
                    }
                    (None, None) => {
                        return Err(crate::client_sync::Error::DisconnectNodeArgsNone);
                    }
                };

                match self.call("disconnectnode", &params) {
                    Ok(serde_json::Value::Null) => Ok(()),
                    Ok(ref val) if val.is_null() => Ok(()),
                    Ok(other) => {
                        Err(crate::client_sync::Error::Returned(format!("disconnectnode expected null, got: {}", other)))
                    }
                    Err(e) => Err(e.into()),
                }
            }
        }
    };
}
