use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    instruction::Instruction,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
};

use crate::{onchain_rent_exempt_calc, OnchainRentExemptCalcResult};

pub const CREATE_ACCOUNT_WITH_SEED_ACCOUNTS_LEN: usize = 3;

#[derive(Clone, Copy, Debug)]
pub struct CreateAccountWithSeedAccounts<'me, 'info> {
    pub from: &'me AccountInfo<'info>,
    pub to: &'me AccountInfo<'info>,
    pub base: &'me AccountInfo<'info>,
}

#[derive(Clone, Copy, Debug)]
pub struct CreateAccountWithSeedKeys {
    pub from: Pubkey,
    pub to: Pubkey,
    pub base: Pubkey,
}

#[derive(Clone, Copy, Debug)]
pub struct CreateAccountWithSeedArgs<'a> {
    pub space: u64,
    pub owner: Pubkey,
    pub lamports: u64,
    pub seed: &'a str,
}

impl From<CreateAccountWithSeedAccounts<'_, '_>> for CreateAccountWithSeedKeys {
    fn from(
        CreateAccountWithSeedAccounts { from, to, base }: CreateAccountWithSeedAccounts<'_, '_>,
    ) -> Self {
        Self {
            from: *from.key,
            to: *to.key,
            base: *base.key,
        }
    }
}

impl<'info> From<CreateAccountWithSeedAccounts<'_, 'info>>
    for [AccountInfo<'info>; CREATE_ACCOUNT_WITH_SEED_ACCOUNTS_LEN]
{
    fn from(
        CreateAccountWithSeedAccounts { from, to, base }: CreateAccountWithSeedAccounts<'_, 'info>,
    ) -> Self {
        [from.clone(), to.clone(), base.clone()]
    }
}

/// Creates the `to` account at
/// sha256[based, seed, owner].
/// `to`'s pubkey can be found using `Pubkey::create_with_seed()`
pub fn create_account_with_seed_ix(
    CreateAccountWithSeedKeys { from, to, base }: CreateAccountWithSeedKeys,
    CreateAccountWithSeedArgs {
        space,
        owner,
        lamports,
        seed,
    }: CreateAccountWithSeedArgs<'_>,
) -> Instruction {
    system_instruction::create_account_with_seed(&from, &to, &base, seed, lamports, space, &owner)
}

pub fn create_account_with_seed_invoke(
    accounts: CreateAccountWithSeedAccounts,
    args: CreateAccountWithSeedArgs,
) -> ProgramResult {
    let ix = create_account_with_seed_ix(CreateAccountWithSeedKeys::from(accounts), args);
    let account_infos: [AccountInfo; CREATE_ACCOUNT_WITH_SEED_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_infos)
}

pub fn create_account_with_seed_invoke_signed(
    accounts: CreateAccountWithSeedAccounts,
    args: CreateAccountWithSeedArgs,
    signer_seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = create_account_with_seed_ix(CreateAccountWithSeedKeys::from(accounts), args);
    let account_infos: [AccountInfo; CREATE_ACCOUNT_WITH_SEED_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_infos, signer_seeds)
}

#[derive(Clone, Copy, Debug)]
pub struct CreateRentExemptAccountWithSeedArgs<'a> {
    pub space: usize,
    pub owner: Pubkey,
    pub seed: &'a str,
}

impl CreateRentExemptAccountWithSeedArgs<'_> {
    pub fn try_calc_lamports(&self) -> Result<CreateAccountWithSeedArgs, ProgramError> {
        let OnchainRentExemptCalcResult { space, lamports } = onchain_rent_exempt_calc(self.space)?;
        Ok(CreateAccountWithSeedArgs {
            space,
            owner: self.owner,
            lamports,
            seed: self.seed,
        })
    }
}

pub fn create_rent_exempt_account_with_seed_invoke(
    accounts: CreateAccountWithSeedAccounts,
    args: CreateRentExemptAccountWithSeedArgs,
) -> ProgramResult {
    let args = args.try_calc_lamports()?;
    create_account_with_seed_invoke(accounts, args)
}

pub fn create_rent_exempt_account_with_seed_invoke_signed(
    accounts: CreateAccountWithSeedAccounts,
    args: CreateRentExemptAccountWithSeedArgs,
    signer_seeds: &[&[&[u8]]],
) -> ProgramResult {
    let args = args.try_calc_lamports()?;
    create_account_with_seed_invoke_signed(accounts, args, signer_seeds)
}
