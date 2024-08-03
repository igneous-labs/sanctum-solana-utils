//! TODO: can_merge() util fn

use solana_program::{program_error::ProgramError, pubkey::Pubkey, sysvar};
use solana_readonly_account::{ReadonlyAccountData, ReadonlyAccountPubkeyBytes};
use stake_program_interface::MergeKeys;

use crate::ReadonlyStakeAccount;

fn read_stake_authority_checked<T: ReadonlyAccountData>(stake: T) -> Result<Pubkey, ProgramError> {
    let s = ReadonlyStakeAccount(stake);
    let s = s.try_into_valid()?;
    let s = s.try_into_stake_or_initialized()?;
    Ok(s.stake_meta_authorized_staker())
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MergeFreeAccountsFromFetched<F> {
    pub from: F,
    pub to: Pubkey,
}

impl<F: ReadonlyAccountData + ReadonlyAccountPubkeyBytes> MergeFreeAccountsFromFetched<F> {
    pub fn resolve(&self) -> Result<MergeKeys, ProgramError> {
        self.resolve_to_free_keys().map(Into::into)
    }

    pub fn resolve_to_free_keys(&self) -> Result<MergeFreeKeys, ProgramError> {
        let Self { from, to } = self;
        Ok(MergeFreeKeys {
            from: Pubkey::new_from_array(from.pubkey_bytes()),
            to: *to,
            stake_authority: read_stake_authority_checked(from)?,
        })
    }
}

impl<F: ReadonlyAccountData + ReadonlyAccountPubkeyBytes> TryFrom<MergeFreeAccountsFromFetched<F>>
    for MergeKeys
{
    type Error = ProgramError;

    fn try_from(value: MergeFreeAccountsFromFetched<F>) -> Result<Self, Self::Error> {
        value.resolve()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MergeFreeAccountsToFetched<T> {
    pub from: Pubkey,
    pub to: T,
}

impl<T: ReadonlyAccountData + ReadonlyAccountPubkeyBytes> MergeFreeAccountsToFetched<T> {
    pub fn resolve(&self) -> Result<MergeKeys, ProgramError> {
        self.resolve_to_free_keys().map(Into::into)
    }

    pub fn resolve_to_free_keys(&self) -> Result<MergeFreeKeys, ProgramError> {
        let Self { from, to } = self;
        Ok(MergeFreeKeys {
            from: *from,
            to: Pubkey::new_from_array(to.pubkey_bytes()),
            stake_authority: read_stake_authority_checked(to)?,
        })
    }
}

impl<T: ReadonlyAccountData + ReadonlyAccountPubkeyBytes> TryFrom<MergeFreeAccountsToFetched<T>>
    for MergeKeys
{
    type Error = ProgramError;

    fn try_from(value: MergeFreeAccountsToFetched<T>) -> Result<Self, Self::Error> {
        value.resolve()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MergeFreeKeys {
    pub from: Pubkey,
    pub to: Pubkey,
    pub stake_authority: Pubkey,
}

impl MergeFreeKeys {
    pub fn resolve(&self) -> MergeKeys {
        let Self {
            from,
            to,
            stake_authority,
        } = self;
        MergeKeys {
            to: *to,
            from: *from,
            stake_authority: *stake_authority,
            clock: sysvar::clock::ID,
            stake_history: sysvar::stake_history::ID,
        }
    }
}

impl From<MergeFreeKeys> for MergeKeys {
    fn from(value: MergeFreeKeys) -> Self {
        value.resolve()
    }
}
