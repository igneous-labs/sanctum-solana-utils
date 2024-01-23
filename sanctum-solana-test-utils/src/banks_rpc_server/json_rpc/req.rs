use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::{As, DisplayFromStr};
use solana_program::pubkey::Pubkey;
use solana_rpc_client_api::config::RpcAccountInfoConfig;
use solana_sdk::commitment_config::CommitmentConfig;

use super::JsonRpc2Ident;

// TODO: other methods
/// solana_rpc_client_api::request::RpcRequest doesn't implement Serialize or Deserialize, or TryFromStr to use with #[serde(with = "As::<DisplayFromStr>")],
/// so we're redefining it here
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RpcMethod {
    GetLatestBlockhash,
    GetMultipleAccounts,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JsonRpcReq {
    pub jsonrpc: JsonRpc2Ident,
    pub id: u64,
    pub method: RpcMethod,
    pub params: Value,
}

#[derive(Deserialize)]
struct GetMultipleAccountsParams(
    #[serde(with = "As::<Vec<DisplayFromStr>>")] Vec<Pubkey>,
    Option<RpcAccountInfoConfig>,
);

pub fn deser_get_multiple_accounts_params(
    params: Value,
) -> Result<(Vec<Pubkey>, Option<RpcAccountInfoConfig>), serde_json::Error> {
    let GetMultipleAccountsParams(keys, cfg) = serde_json::from_value(params)?;
    Ok((keys, cfg))
}

pub fn deser_get_latest_blockhash_params(
    params: Value,
) -> Result<Option<CommitmentConfig>, serde_json::Error> {
    let (res,) = serde_json::from_value(params)?;
    Ok(res)
}
