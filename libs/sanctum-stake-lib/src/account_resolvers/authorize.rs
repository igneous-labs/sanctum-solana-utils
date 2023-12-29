use solana_program::{
    program_error::ProgramError, pubkey::Pubkey, stake::state::StakeAuthorize, sysvar,
};
use solana_readonly_account::{ReadonlyAccountData, ReadonlyAccountPubkey};
use stake_program_interface::AuthorizeKeys;

use crate::ReadonlyStakeAccount;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AuthorizeFreeAccounts<S> {
    pub stake: S,
}

impl<S: ReadonlyAccountData + ReadonlyAccountPubkey> AuthorizeFreeAccounts<S> {
    pub fn resolve(&self, stake_authorize: StakeAuthorize) -> Result<AuthorizeKeys, ProgramError> {
        match stake_authorize {
            StakeAuthorize::Staker => self.resolve_staker(),
            StakeAuthorize::Withdrawer => self.resolve_withdrawer(),
        }
    }

    fn resolve_with_authority_getter(
        &self,
        authority_getter: fn(&S) -> Result<Pubkey, ProgramError>,
    ) -> Result<AuthorizeKeys, ProgramError> {
        let Self { stake } = self;
        if !stake.stake_data_is_valid() {
            return Err(ProgramError::InvalidAccountData);
        }
        let authority = authority_getter(stake)?;
        Ok(AuthorizeKeys {
            stake: *stake.pubkey(),
            clock: sysvar::clock::ID,
            authority,
        })
    }

    pub fn resolve_staker(&self) -> Result<AuthorizeKeys, ProgramError> {
        self.resolve_with_authority_getter(ReadonlyStakeAccount::stake_meta_authorized_staker)
    }

    pub fn resolve_withdrawer(&self) -> Result<AuthorizeKeys, ProgramError> {
        self.resolve_with_authority_getter(ReadonlyStakeAccount::stake_meta_authorized_withdrawer)
    }
}
