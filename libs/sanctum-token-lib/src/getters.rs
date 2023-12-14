//! Sometimes you just want to read a single field of a token account/mint without caring about the other fields
//! Functions work for both token-2022 and tokenkeg accounts

use solana_program::{program_error::ProgramError, pubkey::Pubkey};
use solana_readonly_account::ReadonlyAccountData;
use spl_token_2022::extension::StateWithExtensions;

/// Deserializes the account, so it's more efficient to keep a
/// fully deserialized instance around instead of using this fn
/// if you're gonna be needing the other fields
pub fn token_account_balance<D: ReadonlyAccountData>(
    token_account: D,
) -> Result<u64, ProgramError> {
    let data = token_account.data();
    let state = StateWithExtensions::<spl_token_2022::state::Account>::unpack(&data)?;
    Ok(state.base.amount)
}

/// Deserializes the account, so it's more efficient to keep a
/// fully deserialized instance around instead of using this fn
/// if you're gonna be needing the other fields
pub fn token_account_mint<D: ReadonlyAccountData>(
    token_account: D,
) -> Result<Pubkey, ProgramError> {
    let data = token_account.data();
    let state = StateWithExtensions::<spl_token_2022::state::Account>::unpack(&data)?;
    Ok(state.base.mint)
}

/// Deserializes the account, so it's more efficient to keep a
/// fully deserialized instance around instead of using this fn
/// if you're gonna be needing the other fields
pub fn mint_supply<D: ReadonlyAccountData>(mint_account: D) -> Result<u64, ProgramError> {
    let data = mint_account.data();
    let state = StateWithExtensions::<spl_token_2022::state::Mint>::unpack(&data)?;
    Ok(state.base.supply)
}

/// Deserializes the account, so it's more efficient to keep a
/// fully deserialized instance around instead of using this fn
/// if you're gonna be needing the other fields
pub fn mint_decimals<D: ReadonlyAccountData>(mint_account: D) -> Result<u8, ProgramError> {
    let data = mint_account.data();
    let state = StateWithExtensions::<spl_token_2022::state::Mint>::unpack(&data)?;
    Ok(state.base.decimals)
}
