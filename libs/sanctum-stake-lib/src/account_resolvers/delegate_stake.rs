use solana_program::{program_error::ProgramError, pubkey::Pubkey, stake, sysvar};
use solana_readonly_account::{ReadonlyAccountData, ReadonlyAccountPubkey};
use stake_program_interface::DelegateStakeKeys;

use crate::ReadonlyStakeAccount;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DelegateStakeFreeAccounts<S> {
    pub stake: S,
    pub vote: Pubkey,
}

impl<S: ReadonlyAccountData + ReadonlyAccountPubkey> DelegateStakeFreeAccounts<S> {
    pub fn resolve(&self) -> Result<DelegateStakeKeys, ProgramError> {
        self.resolve_to_free_keys().map(Into::into)
    }

    pub fn resolve_to_free_keys(&self) -> Result<DelegateStakeFreeKeys, ProgramError> {
        let Self { stake, vote } = self;
        let s = ReadonlyStakeAccount(stake);
        let s = s.try_into_valid()?;
        let s = s.try_into_stake_or_initialized()?;
        Ok(DelegateStakeFreeKeys {
            stake: *stake.pubkey(),
            vote: *vote,
            stake_authority: s.stake_meta_authorized_staker(),
        })
    }
}

impl<S: ReadonlyAccountData + ReadonlyAccountPubkey> TryFrom<DelegateStakeFreeAccounts<S>>
    for DelegateStakeKeys
{
    type Error = ProgramError;

    fn try_from(value: DelegateStakeFreeAccounts<S>) -> Result<Self, Self::Error> {
        value.resolve()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DelegateStakeFreeKeys {
    pub stake: Pubkey,
    pub vote: Pubkey,
    pub stake_authority: Pubkey,
}

impl DelegateStakeFreeKeys {
    pub fn resolve(&self) -> DelegateStakeKeys {
        let Self {
            stake,
            vote,
            stake_authority,
        } = self;
        DelegateStakeKeys {
            stake: *stake,
            vote: *vote,
            stake_authority: *stake_authority,
            clock: sysvar::clock::ID,
            stake_history: sysvar::stake_history::ID,
            stake_config: stake::config::ID,
        }
    }
}

impl From<DelegateStakeFreeKeys> for DelegateStakeKeys {
    fn from(value: DelegateStakeFreeKeys) -> Self {
        value.resolve()
    }
}
