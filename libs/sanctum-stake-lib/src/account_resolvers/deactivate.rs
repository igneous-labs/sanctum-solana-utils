use solana_program::{program_error::ProgramError, pubkey::Pubkey, sysvar};
use solana_readonly_account::{ReadonlyAccountData, ReadonlyAccountPubkey};
use stake_program_interface::DeactivateKeys;

use crate::ReadonlyStakeAccount;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DeactivateFreeAccounts<S> {
    pub stake: S,
}

impl<S: ReadonlyAccountData + ReadonlyAccountPubkey> DeactivateFreeAccounts<S> {
    pub fn resolve(&self) -> Result<DeactivateKeys, ProgramError> {
        self.resolve_to_free_keys().map(Into::into)
    }

    pub fn resolve_to_free_keys(&self) -> Result<DeactivateFreeKeys, ProgramError> {
        let Self { stake } = self;
        let s = ReadonlyStakeAccount::try_new(stake)?;
        let s = s.try_into_stake_or_initialized()?;
        let stake_authority = s.stake_meta_authorized_staker();
        Ok(DeactivateFreeKeys {
            stake: *stake.pubkey(),
            stake_authority,
        })
    }
}

impl<S: ReadonlyAccountData + ReadonlyAccountPubkey> TryFrom<DeactivateFreeAccounts<S>>
    for DeactivateKeys
{
    type Error = ProgramError;

    fn try_from(value: DeactivateFreeAccounts<S>) -> Result<Self, Self::Error> {
        value.resolve()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DeactivateFreeKeys {
    pub stake: Pubkey,
    pub stake_authority: Pubkey,
}

impl DeactivateFreeKeys {
    pub fn resolve(&self) -> DeactivateKeys {
        let Self {
            stake,
            stake_authority,
        } = self;
        DeactivateKeys {
            stake: *stake,
            stake_authority: *stake_authority,
            clock: sysvar::clock::ID,
        }
    }
}

impl From<DeactivateFreeKeys> for DeactivateKeys {
    fn from(value: DeactivateFreeKeys) -> Self {
        value.resolve()
    }
}
