use borsh::BorshDeserialize;
use solana_program::program_error::ProgramError;
use spl_stake_pool_interface::{AccountType, StakePool};

pub fn deserialize_stake_pool_checked(mut account_data: &[u8]) -> Result<StakePool, ProgramError> {
    let sp = StakePool::deserialize(&mut account_data)?;
    if sp.account_type != AccountType::StakePool {
        return Err(ProgramError::InvalidAccountData);
    }
    Ok(sp)
}
