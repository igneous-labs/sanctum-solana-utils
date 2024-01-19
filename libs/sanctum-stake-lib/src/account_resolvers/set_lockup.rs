use solana_program::{program_error::ProgramError, pubkey::Pubkey};
use solana_readonly_account::{ReadonlyAccountData, ReadonlyAccountPubkey};
use stake_program_interface::{SetLockupCheckedKeys, SetLockupKeys};

use crate::{ReadonlyStakeAccount, StakeOrInitializedStakeAccount};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SetLockupFreeAccounts<S> {
    pub stake: S,
}

impl<S: ReadonlyAccountData + ReadonlyAccountPubkey> SetLockupFreeAccounts<S> {
    pub fn resolve_withdrawer(&self) -> Result<SetLockupKeys, ProgramError> {
        self.resolve_with_authority_getter(
            StakeOrInitializedStakeAccount::stake_meta_authorized_withdrawer,
        )
    }

    pub fn resolve_custodian(&self) -> Result<SetLockupKeys, ProgramError> {
        self.resolve_with_authority_getter(
            StakeOrInitializedStakeAccount::stake_meta_lockup_custodian,
        )
    }

    pub fn resolve_checked_withdrawer(&self) -> Result<SetLockupCheckedKeys, ProgramError> {
        self.resolve_checked_with_authority_getter(
            StakeOrInitializedStakeAccount::stake_meta_authorized_withdrawer,
        )
    }

    pub fn resolve_checked_custodian(&self) -> Result<SetLockupCheckedKeys, ProgramError> {
        self.resolve_checked_with_authority_getter(
            StakeOrInitializedStakeAccount::stake_meta_lockup_custodian,
        )
    }

    fn resolve_with_authority_getter<'a>(
        &'a self,
        getter: fn(&StakeOrInitializedStakeAccount<&'a S>) -> Pubkey,
    ) -> Result<SetLockupKeys, ProgramError> {
        let Self { stake } = self;
        Ok(SetLockupKeys {
            stake: *stake.pubkey(),
            authority: self.get_authority_checked(getter)?,
        })
    }

    fn resolve_checked_with_authority_getter<'a>(
        &'a self,
        getter: fn(&StakeOrInitializedStakeAccount<&'a S>) -> Pubkey,
    ) -> Result<SetLockupCheckedKeys, ProgramError> {
        let Self { stake } = self;
        Ok(SetLockupCheckedKeys {
            stake: *stake.pubkey(),
            authority: self.get_authority_checked(getter)?,
        })
    }

    fn get_authority_checked<'a>(
        &'a self,
        getter: fn(&StakeOrInitializedStakeAccount<&'a S>) -> Pubkey,
    ) -> Result<Pubkey, ProgramError> {
        let Self { stake } = self;
        let s = ReadonlyStakeAccount::try_new(stake)?;
        let s = s.try_into_stake_or_initialized()?;
        Ok(getter(&s))
    }
}
