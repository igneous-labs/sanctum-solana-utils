use solana_program::{
    program_error::ProgramError, pubkey::Pubkey, stake::state::StakeAuthorize, sysvar,
};
use solana_readonly_account::{ReadonlyAccountData, ReadonlyAccountPubkey};
use stake_program_interface::AuthorizeKeys;

use crate::{ReadonlyStakeAccount, StakeOrInitializedStakeAccount};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AuthorizeFreeAccounts<S> {
    pub stake: S,
}

impl<S: ReadonlyAccountData + ReadonlyAccountPubkey> AuthorizeFreeAccounts<S> {
    pub fn resolve(&self, stake_authorize: StakeAuthorize) -> Result<AuthorizeKeys, ProgramError> {
        self.resolve_to_free_keys(stake_authorize).map(Into::into)
    }

    pub fn resolve_to_free_keys(
        &self,
        stake_authorize: StakeAuthorize,
    ) -> Result<AuthorizeFreeKeys, ProgramError> {
        match stake_authorize {
            StakeAuthorize::Staker => self.resolve_to_free_keys_staker(),
            StakeAuthorize::Withdrawer => self.resolve_to_free_keys_withdrawer(),
        }
    }

    fn resolve_to_free_keys_with_authority_getter<'a>(
        &'a self,
        authority_getter: fn(&StakeOrInitializedStakeAccount<&'a S>) -> Pubkey,
    ) -> Result<AuthorizeFreeKeys, ProgramError> {
        let Self { stake } = self;
        let s = ReadonlyStakeAccount::try_new(stake)?;
        let s = s.try_into_stake_or_initialized()?;
        let authority = authority_getter(&s);
        Ok(AuthorizeFreeKeys {
            stake: *stake.pubkey(),
            authority,
        })
    }

    pub fn resolve_staker(&self) -> Result<AuthorizeKeys, ProgramError> {
        self.resolve_to_free_keys_staker().map(Into::into)
    }

    pub fn resolve_to_free_keys_staker(&self) -> Result<AuthorizeFreeKeys, ProgramError> {
        self.resolve_to_free_keys_with_authority_getter(
            StakeOrInitializedStakeAccount::stake_meta_authorized_staker,
        )
    }

    pub fn resolve_withdrawer(&self) -> Result<AuthorizeKeys, ProgramError> {
        self.resolve_to_free_keys_withdrawer().map(Into::into)
    }

    pub fn resolve_to_free_keys_withdrawer(&self) -> Result<AuthorizeFreeKeys, ProgramError> {
        self.resolve_to_free_keys_with_authority_getter(
            StakeOrInitializedStakeAccount::stake_meta_authorized_withdrawer,
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AuthorizeFreeKeys {
    pub stake: Pubkey,
    pub authority: Pubkey,
}

impl AuthorizeFreeKeys {
    pub fn resolve(&self) -> AuthorizeKeys {
        let Self { stake, authority } = self;
        AuthorizeKeys {
            stake: *stake,
            authority: *authority,
            clock: sysvar::clock::ID,
        }
    }
}

impl From<AuthorizeFreeKeys> for AuthorizeKeys {
    fn from(value: AuthorizeFreeKeys) -> Self {
        value.resolve()
    }
}
