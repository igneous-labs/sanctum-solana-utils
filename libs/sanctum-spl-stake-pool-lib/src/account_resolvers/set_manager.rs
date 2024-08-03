use solana_program::{program_error::ProgramError, pubkey::Pubkey};
use solana_readonly_account::{ReadonlyAccountData, ReadonlyAccountPubkeyBytes};
use spl_stake_pool_interface::{SetManagerKeys, StakePool};

use crate::deserialize_stake_pool_checked;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NewManagerSetting {
    ManagerOnly(Pubkey),
    ManagerFeeAccountOnly(Pubkey),
    Both {
        new_manager: Pubkey,
        new_manager_fee_account: Pubkey,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SetManager<P> {
    pub stake_pool: P,
    pub new_manager_setting: NewManagerSetting,
}

impl<P: ReadonlyAccountData + ReadonlyAccountPubkeyBytes> SetManager<P> {
    pub fn resolve(&self) -> Result<SetManagerKeys, ProgramError> {
        let StakePool {
            manager,
            manager_fee_account,
            ..
        } = deserialize_stake_pool_checked(self.stake_pool.data().as_ref())?;
        let (new_manager, new_manager_fee_account) = match self.new_manager_setting {
            NewManagerSetting::Both {
                new_manager,
                new_manager_fee_account,
            } => (new_manager, new_manager_fee_account),
            NewManagerSetting::ManagerOnly(new_manager) => (new_manager, manager_fee_account),
            NewManagerSetting::ManagerFeeAccountOnly(new_manager_fee_account) => {
                (manager, new_manager_fee_account)
            }
        };
        Ok(SetManagerKeys {
            stake_pool: Pubkey::new_from_array(self.stake_pool.pubkey_bytes()),
            manager,
            new_manager,
            new_manager_fee_account,
        })
    }
}
