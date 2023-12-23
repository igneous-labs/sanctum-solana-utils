use solana_program::{program_error::ProgramError, rent::Rent, sysvar::Sysvar};

use crate::STAKE_ACCOUNT_LEN;

pub fn onchain_rent_exempt_lamports_for_stake_account() -> Result<u64, ProgramError> {
    Ok(Rent::get()?.minimum_balance(STAKE_ACCOUNT_LEN))
}
