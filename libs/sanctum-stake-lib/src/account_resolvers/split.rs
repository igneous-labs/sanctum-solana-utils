use solana_program::{program_error::ProgramError, pubkey::Pubkey};
use solana_readonly_account::{ReadonlyAccountData, ReadonlyAccountPubkey};
use stake_program_interface::SplitKeys;

use crate::{ReadonlyStakeAccount, StakeStateMarker};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SplitFreeAccounts<S> {
    pub from: S,
    pub to: Pubkey,
}

impl<S: ReadonlyAccountData + ReadonlyAccountPubkey> SplitFreeAccounts<S> {
    pub fn resolve(&self) -> Result<SplitKeys, ProgramError> {
        let Self { from, to } = self;
        if !from.stake_data_is_valid()
            || !matches!(
                from.stake_state_marker(),
                StakeStateMarker::Initialized | StakeStateMarker::Stake
            )
        {
            return Err(ProgramError::InvalidAccountData);
        }
        Ok(SplitKeys {
            from: *from.pubkey(),
            to: *to,
            stake_authority: from.stake_meta_authorized_staker_unchecked(),
        })
    }
}

impl<S: ReadonlyAccountData + ReadonlyAccountPubkey> TryFrom<SplitFreeAccounts<S>> for SplitKeys {
    type Error = ProgramError;

    fn try_from(value: SplitFreeAccounts<S>) -> Result<Self, Self::Error> {
        value.resolve()
    }
}
