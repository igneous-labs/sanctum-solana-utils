use solana_program::{program_error::ProgramError, pubkey::Pubkey};
use solana_readonly_account::{ReadonlyAccountData, ReadonlyAccountPubkeyBytes};
use spl_stake_pool_interface::{SetStakerKeys, StakePool};

use crate::deserialize_stake_pool_checked;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SetStaker<P> {
    pub stake_pool: P,
    pub new_staker: Pubkey,
}

impl<P: ReadonlyAccountData + ReadonlyAccountPubkeyBytes> SetStaker<P> {
    pub fn resolve_with_manager_signer(&self) -> Result<SetStakerKeys, ProgramError> {
        let StakePool { manager, .. } =
            deserialize_stake_pool_checked(self.stake_pool.data().as_ref())?;
        Ok(SetStakerKeys {
            stake_pool: Pubkey::new_from_array(self.stake_pool.pubkey_bytes()),
            signer: manager,
            new_staker: self.new_staker,
        })
    }

    pub fn resolve_with_staker_signer(&self) -> Result<SetStakerKeys, ProgramError> {
        let StakePool { staker, .. } =
            deserialize_stake_pool_checked(self.stake_pool.data().as_ref())?;
        Ok(SetStakerKeys {
            stake_pool: Pubkey::new_from_array(self.stake_pool.pubkey_bytes()),
            signer: staker,
            new_staker: self.new_staker,
        })
    }
}
