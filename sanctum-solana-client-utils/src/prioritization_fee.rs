use std::cmp::{max, min};

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
    address_lookup_table::AddressLookupTableAccount,
    commitment_config::{CommitmentConfig, CommitmentLevel},
    compute_budget::ComputeBudgetInstruction,
    hash::Hash,
    instruction::Instruction,
    message::{v0::Message, CompileError, VersionedMessage},
    pubkey::Pubkey,
    signature::Signature,
    transaction::VersionedTransaction,
};

const WEIGHTED_MEDIAN_EPSILON: f64 = 0.0001;

/// A [`RpcSimulateTransactionConfig`] solely for the purpose of simulating a tx to estimate compute units used
pub const EST_CU_SIM_TX_CONFIG: RpcSimulateTransactionConfig = RpcSimulateTransactionConfig {
    sig_verify: false,

    // must set to true or sim will error with blockhash not found
    replace_recent_blockhash: true,

    // set to processed so that this works for a dependent sequence of txs before the previous tx has finalized.
    // If not, the default commitment is finalized, and the next tx in the sequence will report an
    // unnaturally low CU level if the sim fails because it was dependent on the prev tx's state changes,
    commitment: Some(CommitmentConfig {
        commitment: CommitmentLevel::Processed,
    }),

    encoding: None,
    accounts: None,
    min_context_slot: None,
};

/// Calculate slot weighted median value of given sample of prioritization fees
/// (see get_recent_prioritization_fees rpc call)
///
/// Weights are assigned such that the values would range from 1/(# of slots) to 1
///
/// NOTE: assumes unreported slots has `prioritization_fee = 0`
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

    // Unwrap safety: the length of v and w are always the same
    let median = values
        .medf_weighted(&weights, WEIGHTED_MEDIAN_EPSILON)
        .unwrap();
    log::debug!("Calculated slot weighted median for prioritization fee: {median}");
    Some(median.floor() as u64)
}

pub fn get_slot_weighted_median_unit_price(
    client: &RpcClient,
    addresses: &[Pubkey],
) -> Result<u64, ClientError> {
    let rpc_prio_fees = client.get_recent_prioritization_fees(addresses)?;
    calc_slot_weighted_median_prioritization_fees(&rpc_prio_fees).ok_or(
        ClientError::new_with_request(
            solana_rpc_client_api::client_error::ErrorKind::Custom(
                "Could not retrieve samples for prioritization fees".to_owned(),
            ),
            solana_rpc_client_api::request::RpcRequest::GetRecentPrioritizationFees,
        ),
    )
}

pub async fn get_slot_weighted_median_unit_price_nonblocking(
    client: &NonblockingRpcClient,
    addresses: &[Pubkey],
) -> Result<u64, ClientError> {
    let rpc_prio_fees = client.get_recent_prioritization_fees(addresses).await?;
    calc_slot_weighted_median_prioritization_fees(&rpc_prio_fees).ok_or(
        ClientError::new_with_request(
            solana_rpc_client_api::client_error::ErrorKind::Custom(
                "Could not retrieve samples for prioritization fees".to_owned(),
            ),
            solana_rpc_client_api::request::RpcRequest::GetRecentPrioritizationFees,
        ),
    )
}

/// Crafts a versioned tx that can be fed into [`estimate_compute_unit_limit`]
/// or [`estimate_compute_unit_limit_nonblocking`] from the given data
pub fn to_est_cu_sim_tx(
    payer_pk: &Pubkey,
    ixs: &[Instruction],
    luts: &[AddressLookupTableAccount],
) -> Result<VersionedTransaction, CompileError> {
    let message = VersionedMessage::V0(Message::try_compile(payer_pk, ixs, luts, Hash::default())?);
    Ok(VersionedTransaction {
        signatures: vec![Signature::default(); message.header().num_required_signatures.into()],
        message,
    })
}

/// Runs a simulation and returns esimated compute units
pub fn estimate_compute_unit_limit(
    client: &RpcClient,
    tx: &impl SerializableTransaction,
) -> Result<u64, ClientError> {
    client
        .simulate_transaction_with_config(tx, EST_CU_SIM_TX_CONFIG)?
        .value
        .units_consumed
        .ok_or(ClientError::new_with_request(
            solana_rpc_client_api::client_error::ErrorKind::Custom(
                "Could not retrieve consumed compute units from simulation".to_owned(),
            ),
            solana_rpc_client_api::request::RpcRequest::SimulateTransaction,
        ))
}

/// Runs a simulation and returns esimated compute units
pub async fn estimate_compute_unit_limit_nonblocking(
    client: &NonblockingRpcClient,
    tx: &impl SerializableTransaction,
) -> Result<u64, ClientError> {
    client
        .simulate_transaction_with_config(tx, EST_CU_SIM_TX_CONFIG)
        .await?
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
    Ok([
        ComputeBudgetInstruction::set_compute_unit_limit(unit_limit),
        ComputeBudgetInstruction::set_compute_unit_price(min(
            unit_price_micro_lamports,
            max_unit_price_micro_lamports,
        )),
    ])
}

/// Given a compute unit limit and a maximum number of lamports
/// the user is willing to pay for the tx, return the microlamports_per_cu
/// that should be used with [`ComputeBudgetInstruction::set_compute_unit_price()`].
///
/// Returns a minimum of 1
pub fn calc_compute_unit_price(cu_limit: u32, max_lamports: u64) -> u64 {
    let lamport_per_cu = (max_lamports as f64) / (cu_limit as f64);
    let microlamports_per_cu = (lamport_per_cu * 1_000_000.0).floor();
    max(1, microlamports_per_cu as u64)
}
