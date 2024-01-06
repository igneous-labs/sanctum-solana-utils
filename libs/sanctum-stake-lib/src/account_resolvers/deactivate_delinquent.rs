use solana_program::{program_error::ProgramError, pubkey::Pubkey};
use solana_readonly_account::{ReadonlyAccountData, ReadonlyAccountPubkey};
use stake_program_interface::DeactivateDelinquentKeys;

use crate::ReadonlyStakeAccount;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DeactivateDelinquentFreeAccounts<S> {
    pub stake: S,
    pub reference_vote: Pubkey,
}

impl<S: ReadonlyAccountData + ReadonlyAccountPubkey> DeactivateDelinquentFreeAccounts<S> {
    pub fn resolve(&self) -> Result<DeactivateDelinquentKeys, ProgramError> {
        let Self {
            stake,
            reference_vote,
        } = self;
        let s = ReadonlyStakeAccount(stake);
        let s = s.try_into_valid()?;
        let s = s.try_into_stake()?;
        Ok(DeactivateDelinquentKeys {
            stake: *stake.pubkey(),
            reference_vote: *reference_vote,
            vote: s.stake_stake_delegation_voter_pubkey(),
        })
    }
}

impl<S: ReadonlyAccountData + ReadonlyAccountPubkey> TryFrom<DeactivateDelinquentFreeAccounts<S>>
    for DeactivateDelinquentKeys
{
    type Error = ProgramError;

    fn try_from(value: DeactivateDelinquentFreeAccounts<S>) -> Result<Self, Self::Error> {
        value.resolve()
    }
}
