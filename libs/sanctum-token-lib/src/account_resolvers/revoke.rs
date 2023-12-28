use solana_program::program_error::ProgramError;
use solana_readonly_account::{ReadonlyAccountData, ReadonlyAccountPubkey};
use spl_token_interface::RevokeKeys;

use crate::ReadonlyTokenAccount;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RevokeFreeAccounts<D> {
    pub token_account: D,
}

impl<D: ReadonlyAccountData + ReadonlyAccountPubkey> RevokeFreeAccounts<D> {
    pub fn resolve(&self) -> Result<RevokeKeys, ProgramError> {
        let Self { token_account } = self;

        if !token_account.token_account_data_is_valid()
            || !token_account.token_account_is_initialized()
        {
            return Err(ProgramError::InvalidAccountData);
        }

        let authority = token_account.token_account_authority();

        Ok(RevokeKeys {
            token_account: *token_account.pubkey(),
            authority,
        })
    }
}

impl<D: ReadonlyAccountData + ReadonlyAccountPubkey> TryFrom<RevokeFreeAccounts<D>> for RevokeKeys {
    type Error = ProgramError;

    fn try_from(value: RevokeFreeAccounts<D>) -> Result<Self, Self::Error> {
        value.resolve()
    }
}
