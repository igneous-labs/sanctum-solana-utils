use std::cmp::min;

use medians::Medianf64;

use solana_client::nonblocking::rpc_client::RpcClient as NonblockingRpcClient;
use solana_client::rpc_client::RpcClient;
use solana_client::rpc_response::RpcPrioritizationFee;
use solana_rpc_client_api::client_error::Error as ClientError;
use solana_rpc_client_api::config::RpcSimulateTransactionConfig;
use solana_rpc_client_api::response::RpcSimulateTransactionResult;
use solana_sdk::compute_budget::ComputeBudgetInstruction;
use solana_sdk::instruction::Instruction;
use solana_sdk::message::Message;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::transaction::Transaction;

const MAX_SLOT_DISPLACEMENT: u64 = 150;
const WEIGHTED_MEDIAN_EPSILON: f64 = 0.0001;

pub fn get_compute_budget_ixs(unit_limit: u32, unit_price_micro_lamports: u64) -> [Instruction; 2] {
    [
        ComputeBudgetInstruction::set_compute_unit_limit(unit_limit),
        ComputeBudgetInstruction::set_compute_unit_price(unit_price_micro_lamports),
    ]
}

// assumes <= `MAX_SLOT_DISPLACEMENT` slots of sample size
pub fn calc_slot_weighted_median_prioritization_fees(
    rpc_prio_fees: &[RpcPrioritizationFee],
) -> u64 {
    if rpc_prio_fees.is_empty() {
        log::warn!("Given sample for the recent prioritization fee was empty");
        return 0u64;
    }

    // min_slot (the most recent slot) is assumed to be max - MAX_SLOT_DISPLACEMENT
    // NOTE: data for the slot at max - MAX_SLOT_DISPLACEMENT might be missing but we
    // assume that it exists and the fee was 0 for that slot
    let min_slot = rpc_prio_fees
        .iter()
        .max_by_key(|fee| fee.slot)
        .unwrap()
        .slot
        - MAX_SLOT_DISPLACEMENT;

    let (values, weights): (Vec<f64>, Vec<f64>) = rpc_prio_fees
        .iter()
        .filter_map(|fee| {
            if fee.prioritization_fee == 0 {
                None
            } else {
                Some((
                    fee.prioritization_fee as f64,
                    (1 + fee.slot - min_slot) as f64 / MAX_SLOT_DISPLACEMENT as f64,
                ))
            }
        })
        .unzip();

    // Unwrap safty: the length of v and w are always the same
    let median = values
        .medf_weighted(&weights, WEIGHTED_MEDIAN_EPSILON)
        .unwrap();
    log::debug!("Calculated slot weighted median for prioritization fee: {median}");
    median.floor() as u64
}

// /// Runs simulation and returns consumed compute unit
// pub fn estimate_compute_unit_limit(
//     client: RpcClient,
//     ixs: &[Instruction],
// ) -> Result<u64, ClientError> {
//     let tx = Transaction::new_unsigned(Message::new(ixs, None));
//     client
//         .simulate_transaction_with_config(
//             &tx,
//             RpcSimulateTransactionConfig {
//                 sig_verify: false,
//                 ..Default::default()
//             },
//         )?
//         .value
//         .units_consumed
//         .ok_or(ClientError::new_with_request(
//             solana_rpc_client_api::client_error::ErrorKind::Custom(
//                 "Could not retrieve consumed compute units from simulation".to_owned(),
//             ),
//             solana_rpc_client_api::request::RpcRequest::SimulateTransaction,
//         ))
// }

/// Calculates slot weighted median prioritiziation fee and generate compute
/// budget ixs
///
/// NOTE: assumes <= `MAX_SLOT_DISPLACEMENT` slots of sample size for `rpc_prio_fees`
pub fn get_compute_budget_ixs_with_rpc_prio_fees(
    rpc_prio_fees: &[RpcPrioritizationFee],
    unit_limit: u32,
    max_unit_price_micro_lamports: u64,
) -> Result<[Instruction; 2], ClientError> {
    let unit_price_micro_lamports = calc_slot_weighted_median_prioritization_fees(rpc_prio_fees);
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
