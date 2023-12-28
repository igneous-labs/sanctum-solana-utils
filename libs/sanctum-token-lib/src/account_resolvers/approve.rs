use solana_program::{program_error::ProgramError, pubkey::Pubkey};
use solana_readonly_account::{ReadonlyAccountData, ReadonlyAccountPubkey};
use spl_token_interface::{ApproveCheckedKeys, ApproveKeys};

use crate::ReadonlyTokenAccount;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ApproveFreeAccounts<A> {
    pub token_account: A,
    pub delegate: Pubkey,
}

impl<A: ReadonlyAccountData + ReadonlyAccountPubkey> ApproveFreeAccounts<A> {
    pub fn resolve(&self) -> Result<ApproveKeys, ProgramError> {
        let authority = self.token_account_authority()?;
        Ok(ApproveKeys {
            token_account: *self.token_account.pubkey(),
            delegate: self.delegate,
            authority,
        })
    }

    pub fn resolve_checked(&self) -> Result<ApproveCheckedKeys, ProgramError> {
        let authority = self.token_account_authority()?;
        let mint = self.token_account.token_account_mint();
        Ok(ApproveCheckedKeys {
            token_account: *self.token_account.pubkey(),
            delegate: self.delegate,
            authority,
            mint,
        })
    }

    fn token_account_authority(&self) -> Result<Pubkey, ProgramError> {
        if !self.token_account.token_account_data_is_valid()
            || !self.token_account.token_account_is_initialized()
        {
            return Err(ProgramError::InvalidAccountData);
        }

        Ok(self.token_account.token_account_authority())
    }
}

impl<A: ReadonlyAccountData + ReadonlyAccountPubkey> TryFrom<ApproveFreeAccounts<A>>
    for ApproveKeys
{
    type Error = ProgramError;

    fn try_from(value: ApproveFreeAccounts<A>) -> Result<Self, Self::Error> {
        value.resolve()
    }
}

impl<A: ReadonlyAccountData + ReadonlyAccountPubkey> TryFrom<ApproveFreeAccounts<A>>
    for ApproveCheckedKeys
{
    type Error = ProgramError;

    fn try_from(value: ApproveFreeAccounts<A>) -> Result<Self, Self::Error> {
        value.resolve_checked()
    }
}
