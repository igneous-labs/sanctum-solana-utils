use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    instruction::Instruction,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    pubkey::Pubkey,
};
use spl_token_2022::instruction::{set_authority, AuthorityType};

pub const SET_AUTHORITY_IX_ACCOUNTS_LEN: usize = 2;

#[derive(Clone, Copy, Debug)]
pub struct SetAuthorityAccounts<'me, 'info> {
    pub token_program: &'me AccountInfo<'info>,
    pub to_change: &'me AccountInfo<'info>,
    pub current_authority: &'me AccountInfo<'info>,
}

#[derive(Clone, Copy, Debug)]
pub struct SetAuthorityKeys {
    pub token_program: Pubkey,
    pub to_change: Pubkey,
    pub current_authority: Pubkey,
}

#[derive(Clone, Debug)]
pub struct SetAuthorityArgs {
    pub authority_type: AuthorityType,
    pub new_authority: Option<Pubkey>,
}

impl From<SetAuthorityAccounts<'_, '_>> for SetAuthorityKeys {
    fn from(
        SetAuthorityAccounts {
            token_program,
            to_change,
            current_authority,
        }: SetAuthorityAccounts<'_, '_>,
    ) -> Self {
        Self {
            token_program: *token_program.key,
            to_change: *to_change.key,
            current_authority: *current_authority.key,
        }
    }
}

impl<'info> From<SetAuthorityAccounts<'_, 'info>>
    for [AccountInfo<'info>; SET_AUTHORITY_IX_ACCOUNTS_LEN]
{
    fn from(
        SetAuthorityAccounts {
            token_program: _,
            to_change,
            current_authority,
        }: SetAuthorityAccounts<'_, 'info>,
    ) -> Self {
        [to_change.clone(), current_authority.clone()]
    }
}

pub fn set_authority_ix(
    SetAuthorityKeys {
        token_program,
        to_change,
        current_authority,
    }: SetAuthorityKeys,
    SetAuthorityArgs {
        authority_type,
        new_authority,
    }: SetAuthorityArgs,
) -> Result<Instruction, ProgramError> {
    set_authority(
        &token_program,
        &to_change,
        new_authority.as_ref(),
        authority_type,
        &current_authority,
        &[],
    )
}

pub fn set_authority_invoke(
    accounts: SetAuthorityAccounts,
    args: SetAuthorityArgs,
) -> ProgramResult {
    let ix = set_authority_ix(SetAuthorityKeys::from(accounts), args)?;
    let account_infos: [AccountInfo; SET_AUTHORITY_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_infos)
}

pub fn set_authority_invoke_signed(
    accounts: SetAuthorityAccounts,
    args: SetAuthorityArgs,
    signer_seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = set_authority_ix(SetAuthorityKeys::from(accounts), args)?;
    let account_infos: [AccountInfo; SET_AUTHORITY_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_infos, signer_seeds)
}
