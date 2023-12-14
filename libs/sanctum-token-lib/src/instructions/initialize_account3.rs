use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    instruction::Instruction,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    pubkey::Pubkey,
};
use spl_token_2022::instruction::initialize_account3;

pub const INITIALIZE_ACCOUNT_3_ACCOUNTS_LEN: usize = 2;

#[derive(Clone, Copy, Debug)]
pub struct InitializeAccount3Accounts<'me, 'info> {
    pub token_program: &'me AccountInfo<'info>,
    pub to_initialize: &'me AccountInfo<'info>,
    pub mint: &'me AccountInfo<'info>,
}

#[derive(Clone, Copy, Debug)]
pub struct InitializeAccount3Keys {
    pub token_program: Pubkey,
    pub to_initialize: Pubkey,
    pub mint: Pubkey,
}

impl From<InitializeAccount3Accounts<'_, '_>> for InitializeAccount3Keys {
    fn from(
        InitializeAccount3Accounts {
            token_program,
            to_initialize,
            mint,
        }: InitializeAccount3Accounts<'_, '_>,
    ) -> Self {
        Self {
            token_program: *token_program.key,
            to_initialize: *to_initialize.key,
            mint: *mint.key,
        }
    }
}

impl<'info> From<InitializeAccount3Accounts<'_, 'info>>
    for [AccountInfo<'info>; INITIALIZE_ACCOUNT_3_ACCOUNTS_LEN]
{
    fn from(
        InitializeAccount3Accounts {
            token_program: _,
            to_initialize,
            mint,
        }: InitializeAccount3Accounts<'_, 'info>,
    ) -> Self {
        [to_initialize.clone(), mint.clone()]
    }
}

pub fn initialize_account_3_ix(
    InitializeAccount3Keys {
        token_program,
        to_initialize,
        mint,
    }: InitializeAccount3Keys,
    authority: Pubkey,
) -> Result<Instruction, ProgramError> {
    initialize_account3(&token_program, &to_initialize, &mint, &authority)
}

pub fn initialize_account_3_invoke(
    accounts: InitializeAccount3Accounts,
    authority: Pubkey,
) -> ProgramResult {
    let ix = initialize_account_3_ix(InitializeAccount3Keys::from(accounts), authority)?;
    let account_infos: [AccountInfo; INITIALIZE_ACCOUNT_3_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_infos)
}

pub fn initialize_account_3_invoke_signed(
    accounts: InitializeAccount3Accounts,
    authority: Pubkey,
    signer_seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = initialize_account_3_ix(InitializeAccount3Keys::from(accounts), authority)?;
    let account_infos: [AccountInfo; INITIALIZE_ACCOUNT_3_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_infos, signer_seeds)
}
