use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    instruction::Instruction,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    pubkey::Pubkey,
};
use spl_token_2022::instruction::close_account;

pub const CLOSE_TOKEN_ACCOUNT_ACCOUNTS_LEN: usize = 3;

#[derive(Clone, Copy, Debug)]
pub struct CloseTokenAccountAccounts<'me, 'info> {
    pub token_program: &'me AccountInfo<'info>,
    pub account_to_close: &'me AccountInfo<'info>,
    pub refund_rent_to: &'me AccountInfo<'info>,
    pub authority: &'me AccountInfo<'info>,
}

#[derive(Clone, Copy, Debug)]
pub struct CloseTokenAccountKeys {
    pub token_program: Pubkey,
    pub account_to_close: Pubkey,
    pub refund_rent_to: Pubkey,
    pub authority: Pubkey,
}

impl From<CloseTokenAccountAccounts<'_, '_>> for CloseTokenAccountKeys {
    fn from(
        CloseTokenAccountAccounts {
            token_program,
            account_to_close,
            refund_rent_to,
            authority,
        }: CloseTokenAccountAccounts<'_, '_>,
    ) -> Self {
        Self {
            token_program: *token_program.key,
            account_to_close: *account_to_close.key,
            refund_rent_to: *refund_rent_to.key,
            authority: *authority.key,
        }
    }
}

impl<'info> From<CloseTokenAccountAccounts<'_, 'info>>
    for [AccountInfo<'info>; CLOSE_TOKEN_ACCOUNT_ACCOUNTS_LEN]
{
    fn from(
        CloseTokenAccountAccounts {
            token_program: _,
            account_to_close,
            refund_rent_to,
            authority,
        }: CloseTokenAccountAccounts<'_, 'info>,
    ) -> Self {
        [
            account_to_close.clone(),
            refund_rent_to.clone(),
            authority.clone(),
        ]
    }
}

pub fn close_token_account_ix(
    CloseTokenAccountKeys {
        token_program,
        account_to_close,
        refund_rent_to,
        authority,
    }: CloseTokenAccountKeys,
) -> Result<Instruction, ProgramError> {
    close_account(
        &token_program,
        &account_to_close,
        &refund_rent_to,
        &authority,
        &[],
    )
}

pub fn close_token_account_invoke(accounts: CloseTokenAccountAccounts) -> ProgramResult {
    let ix = close_token_account_ix(CloseTokenAccountKeys::from(accounts))?;
    let account_infos: [AccountInfo; CLOSE_TOKEN_ACCOUNT_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_infos)
}

pub fn close_token_account_invoke_signed(
    accounts: CloseTokenAccountAccounts,
    signer_seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = close_token_account_ix(CloseTokenAccountKeys::from(accounts))?;
    let account_infos: [AccountInfo; CLOSE_TOKEN_ACCOUNT_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_infos, signer_seeds)
}
