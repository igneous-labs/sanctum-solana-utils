use solana_program::{entrypoint::ProgramResult, pubkey::Pubkey};
use system_program_interface::{
    allocate_with_seed_invoke, allocate_with_seed_invoke_signed, assign_with_seed_invoke,
    assign_with_seed_invoke_signed, transfer_invoke, transfer_invoke_signed,
    AllocateWithSeedAccounts, AllocateWithSeedIxArgs, AssignWithSeedAccounts, AssignWithSeedIxArgs,
    CreateAccountWithSeedAccounts, TransferAccounts, TransferIxArgs,
};

use crate::{onchain_rent_exempt_lamports_for, space_to_u64};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct InitRentExemptAccountWithSeedArgs {
    pub space: usize,
    pub owner: Pubkey,
    pub seed: String,
}

/// Allocates + assign + transfer required rent exempt lamports to a new account.
///
/// Does not use the CreateAccount ix since that fails if the account already has some lamports in it.
pub fn init_rent_exempt_account_with_seed_invoke(
    CreateAccountWithSeedAccounts { from, to, base }: CreateAccountWithSeedAccounts,
    InitRentExemptAccountWithSeedArgs { space, owner, seed }: InitRentExemptAccountWithSeedArgs,
) -> ProgramResult {
    let required_lamports = onchain_rent_exempt_lamports_for(space)?.saturating_sub(to.lamports());
    let space = space_to_u64(space)?;
    allocate_with_seed_invoke(
        AllocateWithSeedAccounts { allocate: to, base },
        AllocateWithSeedIxArgs {
            space,
            base: *base.key,
            // TODO: update interfaces to generate &str instead of String
            // so we dont have needless allocations like this
            seed: seed.clone(),
            owner,
        },
    )?;
    assign_with_seed_invoke(
        AssignWithSeedAccounts { assign: to, base },
        AssignWithSeedIxArgs {
            owner,
            base: *base.key,
            seed,
        },
    )?;
    if required_lamports > 0 {
        transfer_invoke(
            TransferAccounts { from, to },
            TransferIxArgs {
                lamports: required_lamports,
            },
        )?;
    }
    Ok(())
}

pub fn init_rent_exempt_account_with_seed_invoke_signed(
    CreateAccountWithSeedAccounts { from, to, base }: CreateAccountWithSeedAccounts,
    InitRentExemptAccountWithSeedArgs { space, owner, seed }: InitRentExemptAccountWithSeedArgs,
    signer_seeds: &[&[&[u8]]],
) -> ProgramResult {
    let required_lamports = onchain_rent_exempt_lamports_for(space)?.saturating_sub(to.lamports());
    let space = space_to_u64(space)?;
    allocate_with_seed_invoke_signed(
        AllocateWithSeedAccounts { allocate: to, base },
        AllocateWithSeedIxArgs {
            space,
            base: *base.key,
            // TODO: update interfaces to generate &str instead of String
            // so we dont have needless allocations like this
            seed: seed.clone(),
            owner,
        },
        signer_seeds,
    )?;
    assign_with_seed_invoke_signed(
        AssignWithSeedAccounts { assign: to, base },
        AssignWithSeedIxArgs {
            owner,
            base: *base.key,
            seed,
        },
        signer_seeds,
    )?;
    if required_lamports > 0 {
        transfer_invoke_signed(
            TransferAccounts { from, to },
            TransferIxArgs {
                lamports: required_lamports,
            },
            signer_seeds,
        )?;
    }
    Ok(())
}
