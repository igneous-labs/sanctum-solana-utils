use solana_program::{
    entrypoint::ProgramResult, instruction::Instruction, program_error::ProgramError,
    pubkey::Pubkey, rent::Rent,
};
use system_program_interface::{
    create_account_with_seed_invoke, create_account_with_seed_invoke_signed,
    create_account_with_seed_ix, CreateAccountWithSeedAccounts, CreateAccountWithSeedIxArgs,
    CreateAccountWithSeedKeys,
};

use crate::{onchain_rent_exempt_lamports_for, space_to_u64};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CreateRentExemptAccountWithSeedArgs {
    pub space: usize,
    pub owner: Pubkey,
    pub seed: String,
}

pub fn create_rent_exempt_account_with_seed_ix(
    keys: CreateAccountWithSeedKeys,
    CreateRentExemptAccountWithSeedArgs { space, owner, seed }: CreateRentExemptAccountWithSeedArgs,
    rent: Rent,
) -> Result<Instruction, ProgramError> {
    let lamports = rent.minimum_balance(space);
    let space = space_to_u64(space)?;
    Ok(create_account_with_seed_ix(
        keys,
        CreateAccountWithSeedIxArgs {
            base: keys.base,
            seed,
            lamports,
            space,
            owner,
        },
    ))
}

pub fn create_rent_exempt_account_with_seed_invoke(
    accounts: CreateAccountWithSeedAccounts,
    CreateRentExemptAccountWithSeedArgs { space, owner, seed }: CreateRentExemptAccountWithSeedArgs,
) -> ProgramResult {
    let lamports = onchain_rent_exempt_lamports_for(space)?;
    let space = space_to_u64(space)?;
    create_account_with_seed_invoke(
        accounts,
        CreateAccountWithSeedIxArgs {
            base: *accounts.base.key,
            seed,
            lamports,
            space,
            owner,
        },
    )
}

pub fn create_rent_exempt_account_with_seed_invoke_signed(
    accounts: CreateAccountWithSeedAccounts,
    CreateRentExemptAccountWithSeedArgs { space, owner, seed }: CreateRentExemptAccountWithSeedArgs,
    signer_seeds: &[&[&[u8]]],
) -> ProgramResult {
    let lamports = onchain_rent_exempt_lamports_for(space)?;
    let space = space_to_u64(space)?;
    create_account_with_seed_invoke_signed(
        accounts,
        CreateAccountWithSeedIxArgs {
            base: *accounts.base.key,
            seed,
            lamports,
            space,
            owner,
        },
        signer_seeds,
    )
}
