use serde_json::Value;
use solana_sdk::commitment_config::CommitmentConfig;

pub fn deser_get_latest_blockhash_params(
    params: Value,
) -> Result<Option<CommitmentConfig>, serde_json::Error> {
    let (res,) = serde_json::from_value(params)?;
    Ok(res)
}
