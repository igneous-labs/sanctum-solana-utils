//! Don't use super::multisig_impl::multisig_impl! here because
//! this is the only multisig ix that doesnt require the signatories to be signers

use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    instruction::{AccountMeta, Instruction},
    program::{invoke, invoke_signed},
    pubkey::Pubkey,
};
use spl_token_interface::{
    initialize_multisig_ix_with_program_id, InitializeMultisigAccounts, InitializeMultisigIxArgs,
    InitializeMultisigKeys, INITIALIZE_MULTISIG_IX_ACCOUNTS_LEN,
};

pub fn initialize_multisig_full_ix(
    keys: InitializeMultisigKeys,
    args: InitializeMultisigIxArgs,
    signatories: impl Iterator<Item = Pubkey>,
) -> std::io::Result<Instruction> {
    initialize_multisig_full_ix_with_program_id(spl_token_interface::ID, keys, args, signatories)
}

pub fn initialize_multisig_full_invoke<'a, 'info>(
    accounts: InitializeMultisigAccounts<'a, 'info>,
    args: InitializeMultisigIxArgs,
    signatories: &'a [AccountInfo<'info>],
) -> ProgramResult {
    initialize_multisig_full_invoke_with_program_id(
        spl_token_interface::ID,
        accounts,
        args,
        signatories,
    )
}

pub fn initialize_multisig_full_invoke_signed<'a, 'info>(
    accounts: InitializeMultisigAccounts<'a, 'info>,
    args: InitializeMultisigIxArgs,
    signatories: &'a [AccountInfo<'info>],
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_multisig_full_invoke_signed_with_program_id(
        spl_token_interface::ID,
        accounts,
        args,
        signatories,
        seeds,
    )
}

pub fn initialize_multisig_full_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeMultisigKeys,
    args: InitializeMultisigIxArgs,
    signatories: impl Iterator<Item = Pubkey>,
) -> std::io::Result<Instruction> {
    let mut ix = initialize_multisig_ix_with_program_id(program_id, keys, args)?;
    ix.accounts.extend(signatories.map(|pubkey| AccountMeta {
        pubkey,
        is_signer: false,
        is_writable: false,
    }));
    Ok(ix)
}

pub fn initialize_multisig_full_invoke_with_program_id<'a, 'info>(
    program_id: Pubkey,
    accounts: InitializeMultisigAccounts<'a, 'info>,
    args: InitializeMultisigIxArgs,
    signatories: &'a [AccountInfo<'info>],
) -> ProgramResult {
    let ix = initialize_multisig_full_ix_with_program_id(
        program_id,
        accounts.into(),
        args,
        signatories.iter().map(|a| *a.key),
    )?;
    let mut accounts =
        Vec::from(Into::<[AccountInfo; INITIALIZE_MULTISIG_IX_ACCOUNTS_LEN]>::into(accounts));
    accounts.extend(signatories.iter().cloned());
    invoke(&ix, &accounts)
}

pub fn initialize_multisig_full_invoke_signed_with_program_id<'a, 'info>(
    program_id: Pubkey,
    accounts: InitializeMultisigAccounts<'a, 'info>,
    args: InitializeMultisigIxArgs,
    signatories: &'a [AccountInfo<'info>],
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = initialize_multisig_full_ix_with_program_id(
        program_id,
        accounts.into(),
        args,
        signatories.iter().map(|a| *a.key),
    )?;
    let mut accounts =
        Vec::from(Into::<[AccountInfo; INITIALIZE_MULTISIG_IX_ACCOUNTS_LEN]>::into(accounts));
    accounts.extend(signatories.iter().cloned());
    invoke_signed(&ix, &accounts, seeds)
}
