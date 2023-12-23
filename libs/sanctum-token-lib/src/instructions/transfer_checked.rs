//! NB: transfer is deprecated in token-2022, so just use transfer_checked to support both token programs

use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    instruction::Instruction,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    pubkey::Pubkey,
};
use spl_token_2022::instruction::transfer_checked;

use crate::mint_decimals;

pub const TRANSFER_CHECKED_IX_ACCOUNTS_LEN: usize = 4;

#[derive(Clone, Copy, Debug)]
pub struct TransferCheckedAccounts<'me, 'info> {
    pub token_program: &'me AccountInfo<'info>,
    pub from: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
    pub to: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
}

#[derive(Clone, Copy, Debug)]
pub struct TransferCheckedKeys {
    pub token_program: Pubkey,
    pub from: Pubkey,
    pub mint: Pubkey,
    pub to: Pubkey,
    pub authority: Pubkey,
}

#[derive(Clone, Copy, Debug)]
pub struct TransferCheckedArgs {
    pub amount: u64,
    pub decimals: u8,
}

impl From<TransferCheckedAccounts<'_, '_>> for TransferCheckedKeys {
    fn from(
        TransferCheckedAccounts {
            token_program,
            from,
            mint,
            to,
            authority,
        }: TransferCheckedAccounts<'_, '_>,
    ) -> Self {
        Self {
            token_program: *token_program.key,
            from: *from.key,
            mint: *mint.key,
            to: *to.key,
            authority: *authority.key,
        }
    }
}

impl<'info> From<TransferCheckedAccounts<'_, 'info>>
    for [AccountInfo<'info>; TRANSFER_CHECKED_IX_ACCOUNTS_LEN]
{
    fn from(
        TransferCheckedAccounts {
            token_program: _,
            from,
            mint,
            to,
            authority,
        }: TransferCheckedAccounts<'_, 'info>,
    ) -> Self {
        [from.clone(), mint.clone(), to.clone(), authority.clone()]
    }
}

pub fn transfer_checked_ix(
    TransferCheckedKeys {
        token_program,
        from,
        mint,
        to,
        authority,
    }: TransferCheckedKeys,
    TransferCheckedArgs { amount, decimals }: TransferCheckedArgs,
) -> Result<Instruction, ProgramError> {
    transfer_checked(
        &token_program,
        &from,
        &mint,
        &to,
        &authority,
        &[],
        amount,
        decimals,
    )
}

pub fn transfer_checked_invoke(
    accounts: TransferCheckedAccounts,
    args: TransferCheckedArgs,
) -> ProgramResult {
    let ix = transfer_checked_ix(TransferCheckedKeys::from(accounts), args)?;
    let account_infos: [AccountInfo; TRANSFER_CHECKED_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_infos)
}

pub fn transfer_checked_invoke_signed(
    accounts: TransferCheckedAccounts,
    args: TransferCheckedArgs,
    signer_seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = transfer_checked_ix(TransferCheckedKeys::from(accounts), args)?;
    let account_infos: [AccountInfo; TRANSFER_CHECKED_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_infos, signer_seeds)
}

/// Reads `decimals` from deserializing the mint account.
/// Basically "transfer_checked_unchecked"
///
/// Deserializes the mint account, so it's more efficient to
/// just use [`transfer_checked_invoke`] with the decimals
/// read from the min account if you already have it deserialized.
pub fn transfer_checked_decimal_agnostic_invoke(
    accounts: TransferCheckedAccounts,
    amount: u64,
) -> ProgramResult {
    let decimals = mint_decimals(accounts.mint)?;
    let ix = transfer_checked_ix(
        TransferCheckedKeys::from(accounts),
        TransferCheckedArgs { amount, decimals },
    )?;
    let account_infos: [AccountInfo; TRANSFER_CHECKED_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_infos)
}

/// Reads `decimals` from deserializing the mint account.
/// Basically "transfer_checked_unchecked".
///
/// Deserializes the mint account, so it's more efficient to
/// just use [`transfer_checked_invoke_signed`] with the decimals
/// read from the min account if you already have it deserialized.
pub fn transfer_checked_decimal_agnostic_invoke_signed(
    accounts: TransferCheckedAccounts,
    amount: u64,
    signer_seeds: &[&[&[u8]]],
) -> ProgramResult {
    let decimals = mint_decimals(accounts.mint)?;
    let ix = transfer_checked_ix(
        TransferCheckedKeys::from(accounts),
        TransferCheckedArgs { amount, decimals },
    )?;
    let account_infos: [AccountInfo; TRANSFER_CHECKED_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_infos, signer_seeds)
}
