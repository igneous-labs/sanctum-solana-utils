use std::fmt::Display;

use num_traits::ToPrimitive;
use solana_program::{instruction::InstructionError, program_error::ProgramError};
use solana_program_test::BanksClientError;
use solana_sdk::{
    address_lookup_table::AddressLookupTableAccount,
    compute_budget::ComputeBudgetInstruction,
    hash::Hash,
    instruction::Instruction,
    message::{v0::Message, VersionedMessage},
    pubkey::Pubkey,
    signature::Signature,
    transaction::{TransactionError, VersionedTransaction},
};

/// Extremely fucked up: TransactionReturnData truncates all rightmost zero bytes:
/// https://solana.stackexchange.com/questions/7141/program-return-data-to-client-error
pub fn zero_padded_return_data<const N: usize>(return_data: &[u8]) -> [u8; N] {
    let mut res = [0u8; N];
    let subslice = res.get_mut(..return_data.len()).unwrap();
    subslice.copy_from_slice(return_data);
    res
}

pub fn extract_ix_err(banks_client_err: BanksClientError) -> InstructionError {
    let tx_err = extract_tx_err(banks_client_err);
    match tx_err {
        TransactionError::InstructionError(_, e) => e,
        _ => panic!("Unexpected TransactionError {tx_err}"),
    }
}

pub fn extract_ix_err_code(ix_err: &InstructionError) -> u32 {
    match ix_err {
        InstructionError::Custom(c) => *c,
        _ => panic!("Unexpected InstructionError {ix_err}"),
    }
}

pub fn extract_tx_err(banks_client_err: BanksClientError) -> TransactionError {
    match banks_client_err {
        BanksClientError::TransactionError(e) => e,
        BanksClientError::SimulationError { err, .. } => err,
        _ => panic!("Unexpected BanksClientError {banks_client_err}"),
    }
}

pub fn assert_custom_err<E: Into<ProgramError> + Display + Copy>(
    banks_client_err: BanksClientError,
    expected_err: E,
) {
    let ix_err = extract_ix_err(banks_client_err);
    let actual_code = extract_ix_err_code(&ix_err);
    let expected_program_err: ProgramError = expected_err.into();
    let expected_code = match expected_program_err {
        ProgramError::Custom(c) => c,
        _ => panic!("Unexpected ProgramError {expected_program_err}. This doesn't look like a custom error type.")
    };
    assert_eq!(
        actual_code, expected_code,
        "Expected: {expected_err}. Actual: {ix_err}"
    );
}

/// Some types like SystemError implement different traits
pub fn assert_built_in_prog_err<E: ToPrimitive + Display>(
    banks_client_err: BanksClientError,
    expected_err: E,
) {
    let ix_err = extract_ix_err(banks_client_err);
    let actual_code = extract_ix_err_code(&ix_err);
    let expected_code = expected_err.to_u32().unwrap();
    assert_eq!(
        actual_code, expected_code,
        "Expected: {expected_err}. Actual: {ix_err}"
    );
}

pub fn assert_program_error(banks_client_err: BanksClientError, expected_err: ProgramError) {
    let ix_err = extract_ix_err(banks_client_err);
    let actual_err: ProgramError = ix_err.try_into().unwrap();
    assert_eq!(
        actual_err, expected_err,
        "Expected: {expected_err}. Actual: {actual_err}"
    );
}

pub fn assert_transaction_error(
    banks_client_err: BanksClientError,
    expected_err: TransactionError,
) {
    let actual_err = extract_tx_err(banks_client_err);
    assert_eq!(
        actual_err, expected_err,
        "Expected: {expected_err}. Actual: {actual_err}"
    );
}

pub fn tx_ser_size_with_cb_ixs(
    payer: &Pubkey,
    ixs: impl Iterator<Item = Instruction>,
    luts: &[AddressLookupTableAccount],
) -> usize {
    let mut final_ixs = vec![
        ComputeBudgetInstruction::set_compute_unit_limit(0),
        ComputeBudgetInstruction::set_compute_unit_price(0),
    ];
    final_ixs.extend(ixs);
    tx_ser_size(payer, &final_ixs, luts)
}

pub fn tx_ser_size(
    payer: &Pubkey,
    ixs: &[Instruction],
    luts: &[AddressLookupTableAccount],
) -> usize {
    let message =
        VersionedMessage::V0(Message::try_compile(payer, ixs, luts, Hash::default()).unwrap());
    let n_signers = message.header().num_required_signatures;

    let tx = VersionedTransaction {
        signatures: vec![Signature::default(); n_signers.into()],
        message,
    };
    bincode::serialize(&tx).unwrap().len()
}

/// Asserts size of tx < [`solana_sdk::packet::PACKET_DATA_SIZE`]
pub fn assert_tx_within_size_limits(
    payer: &Pubkey,
    ixs: &[Instruction],
    luts: &[AddressLookupTableAccount],
) {
    assert!(tx_ser_size(payer, ixs, luts) <= solana_sdk::packet::PACKET_DATA_SIZE);
}

/// Asserts size of tx < [`solana_sdk::packet::PACKET_DATA_SIZE`]
pub fn assert_tx_with_cb_ixs_within_size_limits(
    payer: &Pubkey,
    ixs: impl Iterator<Item = Instruction>,
    luts: &[AddressLookupTableAccount],
) {
    assert!(tx_ser_size_with_cb_ixs(payer, ixs, luts) <= solana_sdk::packet::PACKET_DATA_SIZE);
}
