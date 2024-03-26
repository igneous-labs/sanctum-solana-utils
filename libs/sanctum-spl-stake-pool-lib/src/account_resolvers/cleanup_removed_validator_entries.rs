use solana_program::program_error::ProgramError;
use solana_readonly_account::{ReadonlyAccountData, ReadonlyAccountPubkey};
use spl_stake_pool_interface::{CleanupRemovedValidatorEntriesKeys, StakePool};

use crate::deserialize_stake_pool_checked;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CleanupRemovedValidatorEntries<P> {
    pub stake_pool: P,
}

impl<P: ReadonlyAccountData + ReadonlyAccountPubkey> CleanupRemovedValidatorEntries<P> {
    pub fn resolve(&self) -> Result<CleanupRemovedValidatorEntriesKeys, ProgramError> {
        let StakePool { validator_list, .. } =
            deserialize_stake_pool_checked(self.stake_pool.data().as_ref())?;
        Ok(CleanupRemovedValidatorEntriesKeys {
            stake_pool: *self.stake_pool.pubkey(),
            validator_list,
        })
    }
}
