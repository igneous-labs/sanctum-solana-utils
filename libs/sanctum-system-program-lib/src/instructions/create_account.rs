use solana_program::{
    entrypoint::ProgramResult, instruction::Instruction, program_error::ProgramError,
    pubkey::Pubkey, rent::Rent,
};
use system_program_interface::{
    create_account_invoke, create_account_invoke_signed, create_account_ix, CreateAccountAccounts,
    CreateAccountIxArgs, CreateAccountKeys,
};

use crate::{onchain_rent_exempt_lamports_for, space_to_u64};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CreateRentExemptAccountArgs {
    pub space: usize,
    pub owner: Pubkey,
}

pub fn create_rent_exempt_account_ix(
    keys: CreateAccountKeys,
    CreateRentExemptAccountArgs { space, owner }: CreateRentExemptAccountArgs,
    rent: Rent,
) -> Result<Instruction, ProgramError> {
    let lamports = rent.minimum_balance(space);
    let space = space_to_u64(space)?;
    Ok(create_account_ix(
        keys,
        CreateAccountIxArgs {
            lamports,
            space,
            owner,
        },
    ))
}

pub fn create_rent_exempt_account_invoke(
    accounts: CreateAccountAccounts,
    CreateRentExemptAccountArgs { space, owner }: CreateRentExemptAccountArgs,
) -> ProgramResult {
    let lamports = onchain_rent_exempt_lamports_for(space)?;
    let space = space_to_u64(space)?;
    create_account_invoke(
        accounts,
        CreateAccountIxArgs {
            lamports,
            space,
            owner,
        },
    )
}

pub fn create_rent_exempt_account_invoke_signed(
    accounts: CreateAccountAccounts,
    CreateRentExemptAccountArgs { space, owner }: CreateRentExemptAccountArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let lamports = onchain_rent_exempt_lamports_for(space)?;
    let space = space_to_u64(space)?;
    create_account_invoke_signed(
        accounts,
        CreateAccountIxArgs {
            lamports,
            space,
            owner,
        },
        seeds,
    )
}
