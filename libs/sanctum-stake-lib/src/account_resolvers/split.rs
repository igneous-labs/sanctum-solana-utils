use solana_program::{program_error::ProgramError, pubkey::Pubkey};
use solana_readonly_account::{ReadonlyAccountData, ReadonlyAccountPubkeyBytes};
use stake_program_interface::SplitKeys;

use crate::ReadonlyStakeAccount;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SplitFreeAccounts<S> {
    pub from: S,
    pub to: Pubkey,
}

impl<S: ReadonlyAccountData + ReadonlyAccountPubkeyBytes> SplitFreeAccounts<S> {
    pub fn resolve(&self) -> Result<SplitKeys, ProgramError> {
        let Self { from, to } = self;
        let s = ReadonlyStakeAccount(from);
        let s = s.try_into_valid()?;
        let s = s.try_into_stake_or_initialized()?;
        Ok(SplitKeys {
            from: Pubkey::new_from_array(from.pubkey_bytes()),
            to: *to,
            stake_authority: s.stake_meta_authorized_staker(),
        })
    }
}

impl<S: ReadonlyAccountData + ReadonlyAccountPubkeyBytes> TryFrom<SplitFreeAccounts<S>>
    for SplitKeys
{
    type Error = ProgramError;

    fn try_from(value: SplitFreeAccounts<S>) -> Result<Self, Self::Error> {
        value.resolve()
    }
}
