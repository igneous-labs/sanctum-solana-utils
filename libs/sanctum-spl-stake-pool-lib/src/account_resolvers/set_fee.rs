use solana_program::program_error::ProgramError;
use solana_readonly_account::{ReadonlyAccountData, ReadonlyAccountPubkey};
use spl_stake_pool_interface::{SetFeeKeys, StakePool};

use crate::deserialize_stake_pool_checked;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SetFee<P> {
    pub stake_pool: P,
}

impl<P: ReadonlyAccountData + ReadonlyAccountPubkey> SetFee<P> {
    pub fn resolve(&self) -> Result<SetFeeKeys, ProgramError> {
        let StakePool { manager, .. } =
            deserialize_stake_pool_checked(self.stake_pool.data().as_ref())?;
        Ok(SetFeeKeys {
            stake_pool: *self.stake_pool.pubkey(),
            manager,
        })
    }
}
