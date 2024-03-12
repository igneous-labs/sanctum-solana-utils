use std::cmp::min;

use medians::Medianf64;
use solana_client::{
    nonblocking::rpc_client::RpcClient as NonblockingRpcClient,
    rpc_client::{RpcClient, SerializableTransaction},
    rpc_response::RpcPrioritizationFee,
};
use solana_rpc_client_api::{
    client_error::Error as ClientError, config::RpcSimulateTransactionConfig,
};
use solana_sdk::{
    compute_budget::ComputeBudgetInstruction, instruction::Instruction, pubkey::Pubkey,
};

const WEIGHTED_MEDIAN_EPSILON: f64 = 0.0001;

pub fn get_compute_budget_ixs(unit_limit: u32, unit_price_micro_lamports: u64) -> [Instruction; 2] {
    [
        ComputeBudgetInstruction::set_compute_unit_limit(unit_limit),
        ComputeBudgetInstruction::set_compute_unit_price(unit_price_micro_lamports),
    ]
}

pub fn get_writable_account_keys(ixs: &[Instruction]) -> Vec<Pubkey> {
    let mut res = ixs
        .iter()
        .map(|ix| {
            ix.accounts
                .iter()
                .filter_map(|acc_meta| {
                    if acc_meta.is_writable {
                        Some(acc_meta.pubkey)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
        .concat();
    res.sort();
    res.dedup();
    res
}

// NOTE: assumes unreported slots has `prioritization_fee = 0`
pub fn calc_slot_weighted_median_prioritization_fees(
    rpc_prio_fees: &[RpcPrioritizationFee],
) -> Option<u64> {
    let max_slot = rpc_prio_fees.iter().max_by_key(|fee| fee.slot)?.slot;
    let min_slot = rpc_prio_fees.iter().min_by_key(|fee| fee.slot)?.slot;
    let slot_interval = max_slot - min_slot + 1;

    let (values, weights): (Vec<f64>, Vec<f64>) = rpc_prio_fees
        .iter()
        .filter_map(|fee| {
            if fee.prioritization_fee == 0 {
                None
            } else {
                Some((
                    fee.prioritization_fee as f64,
                    (1 + fee.slot - min_slot) as f64 / slot_interval as f64,
                ))
            }
        })
        .unzip();

    // Unwrap safty: the length of v and w are always the same
    let median = values
        .medf_weighted(&weights, WEIGHTED_MEDIAN_EPSILON)
        .unwrap();
    log::debug!("Calculated slot weighted median for prioritization fee: {median}");
    Some(median.floor() as u64)
}

/// Runs simulation and returns consumed compute unit
pub fn estimate_compute_unit_limit(
    client: RpcClient,
    tx: &impl SerializableTransaction,
) -> Result<u64, ClientError> {
    client
        .simulate_transaction_with_config(
            tx,
            RpcSimulateTransactionConfig {
                sig_verify: false,
                ..Default::default()
            },
        )?
        .value
        .units_consumed
        .ok_or(ClientError::new_with_request(
            solana_rpc_client_api::client_error::ErrorKind::Custom(
                "Could not retrieve consumed compute units from simulation".to_owned(),
            ),
            solana_rpc_client_api::request::RpcRequest::SimulateTransaction,
        ))
}

/// Calculates slot weighted median prioritiziation fee and generate compute
/// budget ixs
///
/// NOTE: assumes <= `MAX_SLOT_DISPLACEMENT` slots of sample size for `rpc_prio_fees`
pub fn get_compute_budget_ixs_with_rpc_prio_fees(
    rpc_prio_fees: &[RpcPrioritizationFee],
    unit_limit: u32,
    max_unit_price_micro_lamports: u64,
) -> Result<[Instruction; 2], ClientError> {
    let unit_price_micro_lamports = calc_slot_weighted_median_prioritization_fees(rpc_prio_fees)
        .ok_or(ClientError::new_with_request(
            solana_rpc_client_api::client_error::ErrorKind::Custom(
                "Could not retrieve samples for prioritization fees".to_owned(),
            ),
            solana_rpc_client_api::request::RpcRequest::GetRecentPrioritizationFees,
        ))?;
    Ok(get_compute_budget_ixs(
        unit_limit,
        min(unit_price_micro_lamports, max_unit_price_micro_lamports),
    ))
}

/// Fetches recent prioritization fees and generate compute budget ixs by taking
/// slot weighted median prioritization fee
pub fn get_slot_weighted_median_compute_budget_ixs(
    client: RpcClient,
    addresses: &[Pubkey],
    unit_limit: u32,
    max_unit_price_micro_lamports: u64,
) -> Result<[Instruction; 2], ClientError> {
    let rpc_prio_fees = client.get_recent_prioritization_fees(addresses)?;
    get_compute_budget_ixs_with_rpc_prio_fees(
        &rpc_prio_fees,
        unit_limit,
        max_unit_price_micro_lamports,
    )
}

/// Fetches recent prioritization fees and generate compute budget ixs by taking
/// slot weighted median prioritization fee (nonblocking)
pub async fn get_slot_weighted_median_compute_budget_ixs_nonblocking(
    client: NonblockingRpcClient,
    addresses: &[Pubkey],
    unit_limit: u32,
    max_unit_price_micro_lamports: u64,
) -> Result<[Instruction; 2], ClientError> {
    let rpc_prio_fees = client.get_recent_prioritization_fees(addresses).await?;
    get_compute_budget_ixs_with_rpc_prio_fees(
        &rpc_prio_fees,
        unit_limit,
        max_unit_price_micro_lamports,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let rpc = RpcClient::new_with_commitment(
            "https://api.mainnet-beta.solana.com".to_owned(),
            solana_sdk::commitment_config::CommitmentConfig::processed(),
        );

        let res = get_slot_weighted_median_compute_budget_ixs(rpc, &[], 200_000, 4_200).unwrap();
        println!("priority ixs: {:?}", res);
    }
}
