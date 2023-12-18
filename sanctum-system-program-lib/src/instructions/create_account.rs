use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    instruction::Instruction,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
};

use crate::{onchain_rent_exempt_lamports_for, space_to_u64};

pub const CREATE_ACCOUNT_ACCOUNTS_LEN: usize = 2;

#[derive(Clone, Copy, Debug)]
pub struct CreateAccountAccounts<'me, 'info> {
    pub from: &'me AccountInfo<'info>,
    pub to: &'me AccountInfo<'info>,
}

#[derive(Clone, Copy, Debug)]
pub struct CreateAccountKeys {
    pub from: Pubkey,
    pub to: Pubkey,
}

#[derive(Clone, Copy, Debug)]
pub struct CreateAccountArgs {
    pub space: u64,
    pub owner: Pubkey,
    pub lamports: u64,
}

impl From<CreateAccountAccounts<'_, '_>> for CreateAccountKeys {
    fn from(CreateAccountAccounts { from, to }: CreateAccountAccounts<'_, '_>) -> Self {
        Self {
            from: *from.key,
            to: *to.key,
        }
    }
}

impl<'info> From<CreateAccountAccounts<'_, 'info>>
    for [AccountInfo<'info>; CREATE_ACCOUNT_ACCOUNTS_LEN]
{
    fn from(CreateAccountAccounts { from, to }: CreateAccountAccounts<'_, 'info>) -> Self {
        [from.clone(), to.clone()]
    }
}

pub fn create_account_ix(
    CreateAccountKeys { from, to }: CreateAccountKeys,
    CreateAccountArgs {
        space,
        owner,
        lamports,
    }: CreateAccountArgs,
) -> Instruction {
    system_instruction::create_account(&from, &to, lamports, space, &owner)
}

pub fn create_account_invoke(
    accounts: CreateAccountAccounts,
    args: CreateAccountArgs,
) -> ProgramResult {
    let ix = create_account_ix(CreateAccountKeys::from(accounts), args);
    let account_infos: [AccountInfo; CREATE_ACCOUNT_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_infos)
}

pub fn create_account_invoke_signed(
    accounts: CreateAccountAccounts,
    args: CreateAccountArgs,
    signer_seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = create_account_ix(CreateAccountKeys::from(accounts), args);
    let account_infos: [AccountInfo; CREATE_ACCOUNT_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_infos, signer_seeds)
}

#[derive(Clone, Copy, Debug)]
pub struct CreateRentExemptAccountArgs {
    pub space: usize,
    pub owner: Pubkey,
}

impl CreateRentExemptAccountArgs {
    pub fn try_calc_lamports_onchain(&self) -> Result<CreateAccountArgs, ProgramError> {
        let lamports = onchain_rent_exempt_lamports_for(self.space)?;
        let space = space_to_u64(self.space)?;
        Ok(CreateAccountArgs {
            space,
            owner: self.owner,
            lamports,
        })
    }
}

pub fn create_rent_exempt_account_invoke(
    accounts: CreateAccountAccounts,
    args: CreateRentExemptAccountArgs,
) -> ProgramResult {
    let args = args.try_calc_lamports_onchain()?;
    create_account_invoke(accounts, args)
}

pub fn create_rent_exempt_account_invoke_signed(
    accounts: CreateAccountAccounts,
    args: CreateRentExemptAccountArgs,
    signer_seeds: &[&[&[u8]]],
) -> ProgramResult {
    let args = args.try_calc_lamports_onchain()?;
    create_account_invoke_signed(accounts, args, signer_seeds)
}
