use borsh::BorshDeserialize;
use solana_program::program_error::ProgramError;
use spl_stake_pool_interface::{AccountType, StakePool, ValidatorList};

pub fn deserialize_stake_pool_checked(mut account_data: &[u8]) -> Result<StakePool, ProgramError> {
    let sp = StakePool::deserialize(&mut account_data)?;
    if sp.account_type != AccountType::StakePool {
        return Err(ProgramError::InvalidAccountData);
    }
    Ok(sp)
}

pub fn deserialize_validator_list_checked(
    mut account_data: &[u8],
) -> Result<ValidatorList, ProgramError> {
    let vl = ValidatorList::deserialize(&mut account_data)?;
    if vl.header.account_type != AccountType::ValidatorList {
        return Err(ProgramError::InvalidAccountData);
    }
    Ok(vl)
}
