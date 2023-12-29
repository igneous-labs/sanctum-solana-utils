use solana_program::{
    program_error::ProgramError, pubkey::Pubkey, stake::state::StakeAuthorize, sysvar,
};
use solana_readonly_account::{ReadonlyAccountData, ReadonlyAccountPubkey};
use stake_program_interface::AuthorizeCheckedKeys;

use crate::{ReadonlyStakeAccount, StakeStateMarker};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AuthorizeCheckedFreeAccounts<S> {
    pub stake: S,
    pub new_authority: Pubkey,
}

impl<S: ReadonlyAccountData + ReadonlyAccountPubkey> AuthorizeCheckedFreeAccounts<S> {
    pub fn resolve(
        &self,
        stake_authorize: StakeAuthorize,
    ) -> Result<AuthorizeCheckedKeys, ProgramError> {
        self.resolve_to_free_keys(stake_authorize).map(Into::into)
    }

    pub fn resolve_to_free_keys(
        &self,
        stake_authorize: StakeAuthorize,
    ) -> Result<AuthorizeCheckedFreeKeys, ProgramError> {
        match stake_authorize {
            StakeAuthorize::Staker => self.resolve_to_free_keys_staker(),
            StakeAuthorize::Withdrawer => self.resolve_to_free_keys_withdrawer(),
        }
    }

    pub fn resolve_to_free_keys_staker(&self) -> Result<AuthorizeCheckedFreeKeys, ProgramError> {
        self.resolve_to_free_keys_with_authority_getter(
            ReadonlyStakeAccount::stake_meta_authorized_staker_unchecked,
        )
    }

    pub fn resolve_staker(&self) -> Result<AuthorizeCheckedKeys, ProgramError> {
        self.resolve_to_free_keys_staker().map(Into::into)
    }

    pub fn resolve_to_free_keys_withdrawer(
        &self,
    ) -> Result<AuthorizeCheckedFreeKeys, ProgramError> {
        self.resolve_to_free_keys_with_authority_getter(
            ReadonlyStakeAccount::stake_meta_authorized_withdrawer_unchecked,
        )
    }

    pub fn resolve_withdrawer(&self) -> Result<AuthorizeCheckedKeys, ProgramError> {
        self.resolve_to_free_keys_withdrawer().map(Into::into)
    }

    fn resolve_to_free_keys_with_authority_getter(
        &self,
        getter: fn(&S) -> Pubkey,
    ) -> Result<AuthorizeCheckedFreeKeys, ProgramError> {
        let Self {
            stake,
            new_authority,
        } = self;
        if !stake.stake_data_is_valid()
            || !matches!(
                stake.stake_state_marker(),
                StakeStateMarker::Initialized | StakeStateMarker::Stake
            )
        {
            return Err(ProgramError::InvalidAccountData);
        }
        Ok(AuthorizeCheckedFreeKeys {
            stake: *stake.pubkey(),
            authority: getter(stake),
            new_authority: *new_authority,
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AuthorizeCheckedFreeKeys {
    pub stake: Pubkey,
    pub authority: Pubkey,
    pub new_authority: Pubkey,
}

impl AuthorizeCheckedFreeKeys {
    pub fn resolve(&self) -> AuthorizeCheckedKeys {
        let Self {
            stake,
            authority,
            new_authority,
        } = self;
        AuthorizeCheckedKeys {
            stake: *stake,
            authority: *authority,
            new_authority: *new_authority,
            clock: sysvar::clock::ID,
        }
    }
}

impl From<AuthorizeCheckedFreeKeys> for AuthorizeCheckedKeys {
    fn from(value: AuthorizeCheckedFreeKeys) -> Self {
        value.resolve()
    }
}
