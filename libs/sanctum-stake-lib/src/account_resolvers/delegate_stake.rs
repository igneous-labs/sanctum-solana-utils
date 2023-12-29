use solana_program::{program_error::ProgramError, pubkey::Pubkey, stake, sysvar};
use solana_readonly_account::{ReadonlyAccountData, ReadonlyAccountPubkey};
use stake_program_interface::DelegateStakeKeys;

use crate::{ReadonlyStakeAccount, StakeStateMarker};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DelegateStakeFreeAccounts<S> {
    pub stake: S,
    pub vote: Pubkey,
}

impl<S: ReadonlyAccountData + ReadonlyAccountPubkey> DelegateStakeFreeAccounts<S> {
    pub fn resolve(&self) -> Result<DelegateStakeKeys, ProgramError> {
        let Self { stake, vote } = self;
        if !stake.stake_data_is_valid()
            || !matches!(
                stake.stake_state_marker(),
                StakeStateMarker::Initialized | StakeStateMarker::Stake
            )
        {
            return Err(ProgramError::InvalidAccountData);
        }
        Ok(DelegateStakeKeys {
            stake: *stake.pubkey(),
            vote: *vote,
            stake_authority: stake.stake_meta_authorized_staker_unchecked(),
            clock: sysvar::clock::ID,
            stake_history: sysvar::stake_history::ID,
            stake_config: stake::config::ID,
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
