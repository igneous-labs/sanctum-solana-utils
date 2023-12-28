use solana_program::{program_error::ProgramError, pubkey::Pubkey};
use solana_readonly_account::{ReadonlyAccountData, ReadonlyAccountPubkey};
use spl_token_interface::ApproveKeys;

use crate::ReadonlyTokenAccount;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ApproveFreeAccounts<D> {
    pub token_account: D,
    pub delegate: Pubkey,
}

impl<D: ReadonlyAccountData + ReadonlyAccountPubkey> ApproveFreeAccounts<D> {
    pub fn resolve(&self) -> Result<ApproveKeys, ProgramError> {
        let Self {
            token_account,
            delegate,
        } = self;

        if !token_account.token_account_data_is_valid()
            || !token_account.token_account_is_initialized()
        {
            return Err(ProgramError::InvalidAccountData);
        }

        let authority = token_account.token_account_authority();

        Ok(ApproveKeys {
            token_account: *token_account.pubkey(),
            delegate: *delegate,
            authority,
        })
    }
}

impl<D: ReadonlyAccountData + ReadonlyAccountPubkey> TryFrom<ApproveFreeAccounts<D>>
    for ApproveKeys
{
    type Error = ProgramError;

    fn try_from(value: ApproveFreeAccounts<D>) -> Result<Self, Self::Error> {
        value.resolve()
    }
}
