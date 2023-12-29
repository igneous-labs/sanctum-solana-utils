use solana_program::{program_error::ProgramError, pubkey::Pubkey};
use solana_readonly_account::{ReadonlyAccountData, ReadonlyAccountPubkey};
use stake_program_interface::{SetLockupCheckedKeys, SetLockupKeys};

use crate::{ReadonlyStakeAccount, StakeStateMarker};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SetLockupFreeAccounts<S> {
    pub stake: S,
}

impl<S: ReadonlyAccountData + ReadonlyAccountPubkey> SetLockupFreeAccounts<S> {
    pub fn resolve_withdrawer(&self) -> Result<SetLockupKeys, ProgramError> {
        self.resolve_with_authority_getter(
            ReadonlyStakeAccount::stake_meta_authorized_withdrawer_unchecked,
        )
    }

    pub fn resolve_custodian(&self) -> Result<SetLockupKeys, ProgramError> {
        self.resolve_with_authority_getter(
            ReadonlyStakeAccount::stake_meta_lockup_custodian_unchecked,
        )
    }

    pub fn resolve_checked_withdrawer(&self) -> Result<SetLockupCheckedKeys, ProgramError> {
        self.resolve_checked_with_authority_getter(
            ReadonlyStakeAccount::stake_meta_authorized_withdrawer_unchecked,
        )
    }

    pub fn resolve_checked_custodian(&self) -> Result<SetLockupCheckedKeys, ProgramError> {
        self.resolve_checked_with_authority_getter(
            ReadonlyStakeAccount::stake_meta_lockup_custodian_unchecked,
        )
    }

    fn resolve_with_authority_getter(
        &self,
        getter: fn(&S) -> Pubkey,
    ) -> Result<SetLockupKeys, ProgramError> {
        let Self { stake } = self;
        Ok(SetLockupKeys {
            stake: *stake.pubkey(),
            authority: self.get_authority_checked(getter)?,
        })
    }

    fn resolve_checked_with_authority_getter(
        &self,
        getter: fn(&S) -> Pubkey,
    ) -> Result<SetLockupCheckedKeys, ProgramError> {
        let Self { stake } = self;
        Ok(SetLockupCheckedKeys {
            stake: *stake.pubkey(),
            authority: self.get_authority_checked(getter)?,
        })
    }

    fn get_authority_checked(&self, getter: fn(&S) -> Pubkey) -> Result<Pubkey, ProgramError> {
        let Self { stake } = self;
        if !stake.stake_data_is_valid()
            || !matches!(
                stake.stake_state_marker(),
                StakeStateMarker::Initialized | StakeStateMarker::Stake
            )
        {
            return Err(ProgramError::InvalidAccountData);
        }
        Ok(getter(stake))
    }
}
