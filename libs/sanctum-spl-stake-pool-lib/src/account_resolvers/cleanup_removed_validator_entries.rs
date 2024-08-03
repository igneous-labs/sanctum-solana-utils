use solana_program::{program_error::ProgramError, pubkey::Pubkey};
use solana_readonly_account::{ReadonlyAccountData, ReadonlyAccountPubkeyBytes};
use spl_stake_pool_interface::{CleanupRemovedValidatorEntriesKeys, StakePool};

use crate::deserialize_stake_pool_checked;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CleanupRemovedValidatorEntries<P> {
    pub stake_pool: P,
}

impl<P: ReadonlyAccountData + ReadonlyAccountPubkeyBytes> CleanupRemovedValidatorEntries<P> {
    pub fn resolve(&self) -> Result<CleanupRemovedValidatorEntriesKeys, ProgramError> {
        let StakePool { validator_list, .. } =
            deserialize_stake_pool_checked(self.stake_pool.data().as_ref())?;
        Ok(CleanupRemovedValidatorEntriesKeys {
            stake_pool: Pubkey::new_from_array(self.stake_pool.pubkey_bytes()),
            validator_list,
        })
    }
}
