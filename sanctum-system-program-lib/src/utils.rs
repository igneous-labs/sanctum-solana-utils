use solana_program::{program_error::ProgramError, rent::Rent, sysvar::Sysvar};

/// Returns the rent exempt minimum lamports for `space` calculated from the `Rent` sysvar
pub fn onchain_rent_exempt_lamports_for(space: usize) -> Result<u64, ProgramError> {
    Ok(Rent::get()?.minimum_balance(space))
}

/// Most system instructions expects space as u64 instead of usize.
/// This fn performs the conversion.
pub fn space_to_u64(space: usize) -> Result<u64, ProgramError> {
    space.try_into().map_err(|_e| ProgramError::InvalidArgument)
}
