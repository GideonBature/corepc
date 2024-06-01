// SPDX-License-Identifier: CC0-1.0

//! A JSON-RPC client for testing against Bitcoin Core v22.

mod error;
pub mod v17;
pub mod v18;
pub mod v22;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

pub use crate::client_sync::error::Error;

/// Crate-specific Result type.
///
/// Shorthand for `std::result::Result` with our crate-specific [`Error`] type.
pub type Result<T> = std::result::Result<T, Error>;

/// The different authentication methods for the client.
#[derive(Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum Auth {
    None,
    UserPass(String, String),
    CookieFile(PathBuf),
}

impl Auth {
    /// Convert into the arguments that jsonrpc::Client needs.
    pub fn get_user_pass(self) -> Result<(Option<String>, Option<String>)> {
        match self {
            Auth::None => Ok((None, None)),
            Auth::UserPass(u, p) => Ok((Some(u), Some(p))),
            Auth::CookieFile(path) => {
                let line = BufReader::new(File::open(path)?)
                    .lines()
                    .next()
                    .ok_or(Error::InvalidCookieFile)??;
                let colon = line.find(':').ok_or(Error::InvalidCookieFile)?;
                Ok((Some(line[..colon].into()), Some(line[colon + 1..].into())))
            }
        }
    }
}

/// Defines a `jsonrpc::Client` using `minreq`.
///
/// Expects a const `EXPECTED_SERVER_VERSION` to be defined (form is same as returned in the
/// `getnetworkinfo.version` field e.g,. 260000).
#[macro_export]
macro_rules! define_jsonrpc_minreq_client {
    () => {
        use std::fmt;

        use $crate::client_sync::{log_response, Auth, Result};
        use $crate::client_sync::error::{Error, UnexpectedServerVersionError};

        /// Client implements a JSON-RPC client for the Bitcoin Core daemon or compatible APIs.
        pub struct Client {
            inner: jsonrpc::client::Client,
        }

        impl fmt::Debug for Client {
            fn fmt(&self, f: &mut fmt::Formatter) -> core::fmt::Result {
                write!(
                    f,
                    "bitcoind-json-rpc::client_sync::{}::Client({:?})",
                    EXPECTED_SERVER_VERSION, self.inner
                )
            }
        }

        impl Client {
            /// Creates a client to a bitcoind JSON-RPC server without authentication.
            pub fn new(url: &str) -> Self {
                let transport = jsonrpc::http::minreq_http::Builder::new()
                    .url(url)
                    .expect("jsonrpc v0.18, this function does not error")
                    .build();
                let inner = jsonrpc::client::Client::with_transport(transport);

                Self { inner }
            }

            /// Creates a client to a bitcoind JSON-RPC server without authentication.
            pub fn new_with_auth(url: &str, auth: Auth) -> Result<Self> {
                if matches!(auth, Auth::None) {
                    return Err(Error::MissingUserPassword);
                }
                let (user, pass) = auth.get_user_pass()?;

                let transport = jsonrpc::http::minreq_http::Builder::new()
                    .url(url)
                    .expect("jsonrpc v0.18, this function does not error")
                    .basic_auth(user.unwrap(), pass)
                    .build();
                let inner = jsonrpc::client::Client::with_transport(transport);

                Ok(Self { inner })
            }

            /// Call an RPC `method` with given `args` list.
            pub fn call<T: for<'a> serde::de::Deserialize<'a>>(
                &self,
                method: &str,
                args: &[serde_json::Value],
            ) -> Result<T> {
                let raw = serde_json::value::to_raw_value(args)?;
                let req = self.inner.build_request(&method, Some(&*raw));
                if log::log_enabled!(log::Level::Debug) {
                    log::debug!(target: "bitcoind-json-rpc", "request: {} {}", method, serde_json::Value::from(args));
                }

                let resp = self.inner.send_request(req).map_err(Error::from);
                log_response(method, &resp);
                Ok(resp?.result()?)
            }
        }
    }
}

/// Implement a bunch of helper functions.
///
/// Requires the following functions to be implemented:
///
/// - get_blockchain_info
/// - get_block_verbosity_zero
/// - get_new_address
#[macro_export]
macro_rules! impl_client_helpers {
    () => {
        impl Client {
            /// Gets the blockhash of the current chain tip.
            pub fn best_block_hash(&self) -> Result<bitcoin::BlockHash> {
                let json = self.get_blockchain_info()?;
                let concrete: $crate::model::GetBlockchainInfo = json.try_into().unwrap();
                Ok(concrete.best_block_hash)
            }

            /// Gets a block by blockhash.
            pub fn get_block(&self, hash: &bitcoin::BlockHash) -> Result<bitcoin::Block> {
                let json = self.get_block_verbosity_zero(hash)?;
                let concrete: $crate::model::GetBlockVerbosityZero = json.try_into()?;
                Ok(concrete.0)
            }
        }
    };
}

/// Shorthand for converting a variable into a `serde_json::Value`.
fn into_json<T>(val: T) -> Result<serde_json::Value>
where
    T: serde::ser::Serialize,
{
    Ok(serde_json::to_value(val)?)
}

/// Shorthand for converting an `Option` into an `Option<serde_json::Value>`.
#[allow(dead_code)] // TODO: Remove this if unused still when we are done.
fn opt_into_json<T>(opt: Option<T>) -> Result<serde_json::Value>
where
    T: serde::ser::Serialize,
{
    match opt {
        Some(val) => Ok(into_json(val)?),
        None => Ok(serde_json::Value::Null),
    }
}

/// Shorthand for `serde_json::Value::Null`.
#[allow(dead_code)] // TODO: Remove this if unused still when we are done.
fn null() -> serde_json::Value { serde_json::Value::Null }

/// Shorthand for an empty `serde_json::Value` array.
#[allow(dead_code)] // TODO: Remove this if unused still when we are done.
fn empty_arr() -> serde_json::Value { serde_json::Value::Array(vec![]) }

/// Shorthand for an empty `serde_json` object.
#[allow(dead_code)] // TODO: Remove this if unused still when we are done.
fn empty_obj() -> serde_json::Value { serde_json::Value::Object(Default::default()) }

/// Handle default values in the argument list.
///
/// Substitute `Value::Null`s with corresponding values from `defaults` table, except when they are
/// trailing, in which case just skip them altogether in returned list.
///
/// Note, that `defaults` corresponds to the last elements of `args`.
///
/// ```norust
/// arg1 arg2 arg3 arg4
///           def1 def2
/// ```
///
/// Elements of `args` without corresponding `defaults` value, won't be substituted, because they
/// are required.
#[allow(dead_code)] // TODO: Remove this if unused still when we are done.
fn handle_defaults<'a>(
    args: &'a mut [serde_json::Value],
    defaults: &[serde_json::Value],
) -> &'a [serde_json::Value] {
    assert!(args.len() >= defaults.len());

    // Pass over the optional arguments in backwards order, filling in defaults after the first
    // non-null optional argument has been observed.
    let mut first_non_null_optional_idx = None;
    for i in 0..defaults.len() {
        let args_i = args.len() - 1 - i;
        let defaults_i = defaults.len() - 1 - i;
        if args[args_i] == serde_json::Value::Null {
            if first_non_null_optional_idx.is_some() {
                if defaults[defaults_i] == serde_json::Value::Null {
                    panic!("Missing `default` for argument idx {}", args_i);
                }
                args[args_i] = defaults[defaults_i].clone();
            }
        } else if first_non_null_optional_idx.is_none() {
            first_non_null_optional_idx = Some(args_i);
        }
    }

    let required_num = args.len() - defaults.len();

    if let Some(i) = first_non_null_optional_idx {
        &args[..i + 1]
    } else {
        &args[..required_num]
    }
}

/// Convert a possible-null result into an `Option`.
#[allow(dead_code)] // TODO: Remove this if unused still when we are done.
fn opt_result<T: for<'a> serde::de::Deserialize<'a>>(
    result: serde_json::Value,
) -> Result<Option<T>> {
    if result == serde_json::Value::Null {
        Ok(None)
    } else {
        Ok(serde_json::from_value(result)?)
    }
}

/// Helper to log an RPC response.
fn log_response(method: &str, resp: &Result<jsonrpc::Response>) {
    use log::Level::{Debug, Trace, Warn};

    if log::log_enabled!(Warn) || log::log_enabled!(Debug) || log::log_enabled!(Trace) {
        match resp {
            Err(ref e) =>
                if log::log_enabled!(Debug) {
                    log::debug!(target: "bitcoind-json-rpc", "error: {}: {:?}", method, e);
                },
            Ok(ref resp) =>
                if let Some(ref e) = resp.error {
                    if log::log_enabled!(Debug) {
                        log::debug!(target: "bitcoind-json-rpc", "response error for {}: {:?}", method, e);
                    }
                } else if log::log_enabled!(Trace) {
                    let def =
                        serde_json::value::to_raw_value(&serde_json::value::Value::Null).unwrap();
                    let result = resp.result.as_ref().unwrap_or(&def);
                    log::trace!(target: "bitcoind-json-rpc", "response for {}: {}", method, result);
                },
        }
    }
}
