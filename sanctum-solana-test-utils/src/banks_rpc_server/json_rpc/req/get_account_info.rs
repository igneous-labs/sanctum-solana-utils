use serde::Deserialize;
use serde_json::Value;
use serde_with::{As, DisplayFromStr};
use solana_program::pubkey::Pubkey;
use solana_rpc_client_api::config::RpcAccountInfoConfig;

#[derive(Deserialize)]
struct GetAccountInfoParams(
    #[serde(with = "As::<DisplayFromStr>")] Pubkey,
    Option<RpcAccountInfoConfig>,
);

pub fn deser_get_account_info_params(
    params: Value,
) -> Result<(Pubkey, Option<RpcAccountInfoConfig>), serde_json::Error> {
    let GetAccountInfoParams(key, cfg) = serde_json::from_value(params)?;
    Ok((key, cfg))
}
