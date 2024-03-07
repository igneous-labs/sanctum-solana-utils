use medians::Medianf64;

use solana_client::nonblocking::rpc_client::RpcClient as NonblockingRpcClient;
use solana_client::rpc_client::RpcClient;
use solana_client::rpc_response::RpcPrioritizationFee;
use solana_rpc_client_api::client_error::Error as ClientError;
use solana_sdk::compute_budget::ComputeBudgetInstruction;
use solana_sdk::instruction::Instruction;
use solana_sdk::pubkey::Pubkey;

const MAX_SLOT_DISPLACEMENT: u64 = 150;
const WEIGHTED_MEDIAN_EPSILON: f64 = 0.0001;

fn get_compute_budget_ixs(unit_limit: u32, unit_price_micro_lamports: u64) -> [Instruction; 2] {
    [
        ComputeBudgetInstruction::set_compute_unit_limit(unit_limit),
        ComputeBudgetInstruction::set_compute_unit_price(unit_price_micro_lamports),
    ]
}

// Assume ~150 slots of sample size
fn get_slot_weighted_median_prioritization_fees(rpc_prio_fees: &[RpcPrioritizationFee]) -> u64 {
    if rpc_prio_fees.is_empty() {
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

    let (v, w): (Vec<f64>, Vec<f64>) = rpc_prio_fees
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
        .into_iter()
        .unzip();

    // Unwrap safty: the length of v and w are always the same
    let median = v.medf_weighted(&w, WEIGHTED_MEDIAN_EPSILON).unwrap();
    median.floor() as u64
}

fn get_compute_budget_ixs_with_rpc_prio_fees(
    rpc_prio_fees: &[RpcPrioritizationFee],
    unit_limit: u32,
) -> Result<[Instruction; 2], ClientError> {
    let unit_price_micro_lamports = get_slot_weighted_median_prioritization_fees(&rpc_prio_fees);
    Ok(get_compute_budget_ixs(
        unit_limit,
        unit_price_micro_lamports,
    ))
}

pub fn get_slot_weighted_mean_compute_budget_ixs(
    client: RpcClient,
    addresses: &[Pubkey],
    unit_limit: u32,
) -> Result<[Instruction; 2], ClientError> {
    let rpc_prio_fees = client.get_recent_prioritization_fees(addresses)?;
    get_compute_budget_ixs_with_rpc_prio_fees(&rpc_prio_fees, unit_limit)
}

pub async fn get_slot_weighted_mean_compute_budget_ixs_nonblocking(
    client: NonblockingRpcClient,
    addresses: &[Pubkey],
    unit_limit: u32,
) -> Result<[Instruction; 2], ClientError> {
    let rpc_prio_fees = client.get_recent_prioritization_fees(addresses).await?;
    get_compute_budget_ixs_with_rpc_prio_fees(&rpc_prio_fees, unit_limit)
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

        let res = get_slot_weighted_mean_compute_budget_ixs(rpc, &[], 1000).unwrap();
        println!("priority ixs: {:?}", res);
    }
}
