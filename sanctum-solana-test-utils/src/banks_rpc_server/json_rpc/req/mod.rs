use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::JsonRpc2Ident;

mod get_account_info;
mod get_latest_blockhash;
mod get_multiple_accounts;

pub use get_account_info::*;
pub use get_latest_blockhash::*;
pub use get_multiple_accounts::*;

// TODO: other methods
/// solana_rpc_client_api::request::RpcRequest doesn't implement Serialize or Deserialize, or TryFromStr to use with #[serde(with = "As::<DisplayFromStr>")],
/// so we're redefining it here
#[allow(clippy::enum_variant_names)] // common "Get" prefix
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RpcMethod {
    GetAccountInfo,
    GetLatestBlockhash,
    GetMultipleAccounts,
    GetVersion, // many RpcClient methods call this method before calling the actual method
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JsonRpcReq {
    pub jsonrpc: JsonRpc2Ident,
    pub id: u64,
    pub method: RpcMethod,
    pub params: Value,
}
