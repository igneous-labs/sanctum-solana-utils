use solana_program::{entrypoint::ProgramResult, pubkey::Pubkey};
use system_program_interface::{
    allocate_invoke, allocate_invoke_signed, assign_invoke, assign_invoke_signed, transfer_invoke,
    transfer_invoke_signed, AllocateAccounts, AllocateIxArgs, AssignAccounts, AssignIxArgs,
    CreateAccountAccounts, TransferAccounts, TransferIxArgs,
};

use crate::{onchain_rent_exempt_lamports_for, space_to_u64};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct InitRentExemptAccountArgs {
    pub space: usize,
    pub owner: Pubkey,
}

/// Allocates + assign + transfer required rent exempt lamports to a new account.
///
/// Does not use the CreateAccount ix since that fails if the account already has some lamports in it.
pub fn init_rent_exempt_account_invoke(
    CreateAccountAccounts { from, to }: CreateAccountAccounts,
    InitRentExemptAccountArgs { space, owner }: InitRentExemptAccountArgs,
) -> ProgramResult {
    let required_lamports = onchain_rent_exempt_lamports_for(space)?.saturating_sub(to.lamports());
    let space = space_to_u64(space)?;
    allocate_invoke(AllocateAccounts { allocate: to }, AllocateIxArgs { space })?;
    assign_invoke(AssignAccounts { assign: to }, AssignIxArgs { owner })?;
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

/// Allocates + assign + transfer required rent exempt lamports to a new account.
///
/// Does not use the CreateAccount ix since that fails if the account already has some lamports in it.
pub fn init_rent_exempt_account_invoke_signed(
    CreateAccountAccounts { from, to }: CreateAccountAccounts,
    InitRentExemptAccountArgs { space, owner }: InitRentExemptAccountArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let required_lamports = onchain_rent_exempt_lamports_for(space)?.saturating_sub(to.lamports());
    let space = space_to_u64(space)?;
    allocate_invoke_signed(
        AllocateAccounts { allocate: to },
        AllocateIxArgs { space },
        seeds,
    )?;
    assign_invoke_signed(AssignAccounts { assign: to }, AssignIxArgs { owner }, seeds)?;
    if required_lamports > 0 {
        transfer_invoke_signed(
            TransferAccounts { from, to },
            TransferIxArgs {
                lamports: required_lamports,
            },
            seeds,
        )?;
    }
    Ok(())
}
