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
    inner_instructions: false,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ComputeBudgetIxs {
    pub set_limit: Instruction,
    pub set_price: Instruction,
}

impl ComputeBudgetIxs {
    pub fn new(cu_limit: u32, micro_lamports_per_cu: u64) -> Self {
        Self {
            set_limit: ComputeBudgetInstruction::set_compute_unit_limit(cu_limit),
            set_price: ComputeBudgetInstruction::set_compute_unit_price(micro_lamports_per_cu),
        }
    }

    pub const fn as_ref_arr(&self) -> [&Instruction; 2] {
        [&self.set_limit, &self.set_limit]
    }

    pub fn to_arr(self) -> [Instruction; 2] {
        [self.set_limit, self.set_price]
    }
}

impl IntoIterator for ComputeBudgetIxs {
    type Item = Instruction;

    type IntoIter = std::array::IntoIter<Instruction, 2>;

    fn into_iter(self) -> Self::IntoIter {
        self.to_arr().into_iter()
    }
}

impl<'a> IntoIterator for &'a ComputeBudgetIxs {
    type Item = &'a Instruction;

    type IntoIter = std::array::IntoIter<&'a Instruction, 2>;

    fn into_iter(self) -> Self::IntoIter {
        self.as_ref_arr().into_iter()
    }
}

pub fn writable_addresses(ixs: &[Instruction]) -> impl Iterator<Item = Pubkey> + '_ {
    ixs.iter().flat_map(|ix| {
        ix.accounts.iter().filter_map(|acc| {
            if acc.is_writable {
                Some(acc.pubkey)
            } else {
                None
            }
        })
    })
}

/// Calculate slot weighted median value of given sample of prioritization fees
/// (see get_recent_prioritization_fees rpc call). Returns the median microlamport per CU.
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
    writable_addresses: &[Pubkey],
) -> Result<u64, ClientError> {
    let rpc_prio_fees = client.get_recent_prioritization_fees(writable_addresses)?;
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
    writable_addresses: &[Pubkey],
) -> Result<u64, ClientError> {
    let rpc_prio_fees = client
        .get_recent_prioritization_fees(writable_addresses)
        .await?;
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
    let sim_res = client.simulate_transaction_with_config(tx, EST_CU_SIM_TX_CONFIG)?;
    if let Some(err) = sim_res.value.err {
        return Err(ClientError::new_with_request(
            solana_rpc_client_api::client_error::ErrorKind::TransactionError(err),
            solana_rpc_client_api::request::RpcRequest::SimulateTransaction,
        ));
    }
    sim_res
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
    let sim_res = client
        .simulate_transaction_with_config(tx, EST_CU_SIM_TX_CONFIG)
        .await?;
    if let Some(err) = sim_res.value.err {
        return Err(ClientError::new_with_request(
            solana_rpc_client_api::client_error::ErrorKind::TransactionError(err),
            solana_rpc_client_api::request::RpcRequest::SimulateTransaction,
        ));
    }
    sim_res
        .value
        .units_consumed
        .ok_or(ClientError::new_with_request(
            solana_rpc_client_api::client_error::ErrorKind::Custom(
                "Could not retrieve consumed compute units from simulation".to_owned(),
            ),
            solana_rpc_client_api::request::RpcRequest::SimulateTransaction,
        ))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ComputeBudgetFeeLimit {
    TotalLamports(u64),
    MicroLamportsPerCu(u64),
}

impl ComputeBudgetFeeLimit {
    pub fn to_micro_lamports_per_cu(self, cu_limit: u32) -> u64 {
        match self {
            Self::MicroLamportsPerCu(micro_lamports) => micro_lamports,
            Self::TotalLamports(lamports) => calc_compute_unit_price(cu_limit, lamports),
        }
    }
}

/// Given a compute unit limit and number of lamports
/// the user is willing to pay for the tx, return the micro_lamports_per_cu
/// that should be used with [`ComputeBudgetInstruction::set_compute_unit_price()`].
pub fn calc_compute_unit_price(cus: u32, lamports: u64) -> u64 {
    let lamports_per_cu = (lamports as f64) / (cus as f64);
    let micro_lamports_per_cu = (lamports_per_cu * 1_000_000.0).floor();
    micro_lamports_per_cu as u64
}

/// Returns `cus * cu_buffer_ratio`.
///
/// `cu_buffer_ratio` should be >= 1.0
pub fn buffer_compute_units(cus: u64, cu_buffer_ratio: f64) -> u32 {
    let cus = ((cus as f64) * cu_buffer_ratio).ceil();
    cus as u32
}

/// Calculates slot weighted median prioritiziation fee and generate compute
/// budget ixs, picking the smaller value
pub fn get_compute_budget_ixs_with_rpc_prio_fees(
    rpc_prio_fees: &[RpcPrioritizationFee],
    cu_limit: u32,
    fee_limit: &ComputeBudgetFeeLimit,
) -> Result<ComputeBudgetIxs, ClientError> {
    let unit_price_micro_lamports = calc_slot_weighted_median_prioritization_fees(rpc_prio_fees)
        .ok_or(ClientError::new_with_request(
            solana_rpc_client_api::client_error::ErrorKind::Custom(
                "Could not retrieve samples for prioritization fees".to_owned(),
            ),
            solana_rpc_client_api::request::RpcRequest::GetRecentPrioritizationFees,
        ))?;
    let limit_micro_lamports_per_cu = fee_limit.to_micro_lamports_per_cu(cu_limit);
    Ok(ComputeBudgetIxs::new(
        cu_limit,
        min(unit_price_micro_lamports, limit_micro_lamports_per_cu),
    ))
}

/// Simulates a tx, calculates median priority fees,
/// and return the corresponding ComputeBudget instructions.
///
/// NB: this fn makes 2 RPC requests - simulateTransaction and getRecentPriorityFees
///
/// ## Args
/// - `fee_limit`: total amount of lamports the user is willing to pay for this transaction
/// - `cu_buffer_ratio`: multiple to multiply simulation CU result by to give some room for error.
///    Should be >= 1.0
pub fn get_compute_budget_ixs_auto(
    client: &RpcClient,
    payer_pk: &Pubkey,
    ixs: &[Instruction],
    luts: &[AddressLookupTableAccount],
    fee_limit: &ComputeBudgetFeeLimit,
    cu_buffer_ratio: f64,
) -> Result<ComputeBudgetIxs, ClientError> {
    let tx_to_sim = to_est_cu_sim_tx(payer_pk, ixs, luts).map_err(|e| {
        ClientError::new_with_request(
            solana_rpc_client_api::client_error::ErrorKind::Custom(format!("{e}")),
            solana_rpc_client_api::request::RpcRequest::SimulateTransaction,
        )
    })?;
    let cus = estimate_compute_unit_limit(client, &tx_to_sim)?;
    let cu_limit = buffer_compute_units(cus, cu_buffer_ratio);
    let limit_micro_lamports_per_cu = fee_limit.to_micro_lamports_per_cu(cu_limit);
    let writable: Vec<Pubkey> = writable_addresses(ixs).collect();
    let slot_weighted_micro_lamports_per_cu =
        get_slot_weighted_median_unit_price(client, &writable)?;
    let micro_lamports_per_cu = min(
        slot_weighted_micro_lamports_per_cu,
        limit_micro_lamports_per_cu as u64,
    );
    Ok(ComputeBudgetIxs::new(cu_limit, micro_lamports_per_cu))
}

/// async version of [`get_compute_budget_ixs_auto`]
pub async fn get_compute_budget_ixs_auto_nonblocking(
    client: &NonblockingRpcClient,
    payer_pk: &Pubkey,
    ixs: &[Instruction],
    luts: &[AddressLookupTableAccount],
    fee_limit: &ComputeBudgetFeeLimit,
    cu_buffer_ratio: f64,
) -> Result<ComputeBudgetIxs, ClientError> {
    let tx_to_sim = to_est_cu_sim_tx(payer_pk, ixs, luts).map_err(|e| {
        ClientError::new_with_request(
            solana_rpc_client_api::client_error::ErrorKind::Custom(format!("{e}")),
            solana_rpc_client_api::request::RpcRequest::SimulateTransaction,
        )
    })?;
    let cus = estimate_compute_unit_limit_nonblocking(client, &tx_to_sim).await?;
    let cu_limit = buffer_compute_units(cus, cu_buffer_ratio);
    let limit_micro_lamports_per_cu = fee_limit.to_micro_lamports_per_cu(cu_limit);
    let writable: Vec<Pubkey> = writable_addresses(ixs).collect();
    let slot_weighted_micro_lamports_per_cu =
        get_slot_weighted_median_unit_price_nonblocking(client, &writable).await?;
    let micro_lamports_per_cu = min(
        slot_weighted_micro_lamports_per_cu,
        limit_micro_lamports_per_cu as u64,
    );
    Ok(ComputeBudgetIxs::new(cu_limit, micro_lamports_per_cu))
}
