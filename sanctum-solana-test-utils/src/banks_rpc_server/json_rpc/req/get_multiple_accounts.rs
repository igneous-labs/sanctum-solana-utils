use serde::Deserialize;
use serde_json::Value;
use serde_with::{As, DisplayFromStr};
use solana_program::pubkey::Pubkey;
use solana_rpc_client_api::config::RpcAccountInfoConfig;

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
