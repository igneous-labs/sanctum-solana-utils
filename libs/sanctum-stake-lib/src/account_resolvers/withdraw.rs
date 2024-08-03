use solana_program::{program_error::ProgramError, pubkey::Pubkey, sysvar};
use solana_readonly_account::{ReadonlyAccountData, ReadonlyAccountPubkeyBytes};
use stake_program_interface::WithdrawKeys;

use crate::ReadonlyStakeAccount;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct WithdrawFreeAccounts<S> {
    pub from: S,
    pub to: Pubkey,
}

impl<S: ReadonlyAccountData + ReadonlyAccountPubkeyBytes> WithdrawFreeAccounts<S> {
    pub fn resolve(&self) -> Result<WithdrawKeys, ProgramError> {
        self.resolve_to_free_keys().map(Into::into)
    }

    pub fn resolve_to_free_keys(&self) -> Result<WithdrawFreeKeys, ProgramError> {
        let Self { from, to } = self;
        let s = ReadonlyStakeAccount(from);
        let s = s.try_into_valid()?;
        let s = s.try_into_stake_or_initialized()?;
        let withdraw_authority = s.stake_meta_authorized_withdrawer();
        Ok(WithdrawFreeKeys {
            from: Pubkey::new_from_array(from.pubkey_bytes()),
            to: *to,
            withdraw_authority,
        })
    }
}

impl<S: ReadonlyAccountData + ReadonlyAccountPubkeyBytes> TryFrom<WithdrawFreeAccounts<S>>
    for WithdrawKeys
{
    type Error = ProgramError;

    fn try_from(value: WithdrawFreeAccounts<S>) -> Result<Self, Self::Error> {
        value.resolve()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct WithdrawFreeKeys {
    pub from: Pubkey,
    pub to: Pubkey,
    pub withdraw_authority: Pubkey,
}

impl WithdrawFreeKeys {
    pub fn resolve(&self) -> WithdrawKeys {
        let Self {
            from,
            to,
            withdraw_authority,
        } = self;
        WithdrawKeys {
            from: *from,
            to: *to,
            withdraw_authority: *withdraw_authority,
            clock: sysvar::clock::ID,
            stake_history: sysvar::stake_history::ID,
        }
    }
}

impl From<WithdrawFreeKeys> for WithdrawKeys {
    fn from(value: WithdrawFreeKeys) -> Self {
        value.resolve()
    }
}
