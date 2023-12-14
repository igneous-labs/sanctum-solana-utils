use solana_program::{program_error::ProgramError, rent::Rent, sysvar::Sysvar};

pub struct OnchainRentExemptCalcResult {
    /// most system instructions accept u64 space instead of usize
    pub space: u64,
    pub lamports: u64,
}

pub fn onchain_rent_exempt_calc(space: usize) -> Result<OnchainRentExemptCalcResult, ProgramError> {
    let lamports = Rent::get()?.minimum_balance(space);
    let space_u64: u64 = space
        .try_into()
        .map_err(|_e| ProgramError::InvalidArgument)?;
    Ok(OnchainRentExemptCalcResult {
        space: space_u64,
        lamports,
    })
}
