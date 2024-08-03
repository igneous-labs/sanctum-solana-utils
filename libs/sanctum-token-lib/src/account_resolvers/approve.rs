use solana_program::{program_error::ProgramError, pubkey::Pubkey};
use solana_readonly_account::{ReadonlyAccountData, ReadonlyAccountPubkeyBytes};
use spl_token_interface::{ApproveCheckedKeys, ApproveKeys};

use crate::{InitializedTokenAccount, ReadonlyTokenAccount};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ApproveFreeAccounts<A> {
    pub token_account: A,
    pub delegate: Pubkey,
}

impl<A: ReadonlyAccountData + ReadonlyAccountPubkeyBytes> ApproveFreeAccounts<A> {
    pub fn resolve(&self) -> Result<ApproveKeys, ProgramError> {
        let authority = self.initialized_token_account()?.token_account_authority();
        Ok(ApproveKeys {
            token_account: Pubkey::new_from_array(self.token_account.pubkey_bytes()),
            delegate: self.delegate,
            authority,
        })
    }

    pub fn resolve_checked(&self) -> Result<ApproveCheckedKeys, ProgramError> {
        let t = self.initialized_token_account()?;
        let authority = t.token_account_authority();
        let mint = t.token_account_mint();
        Ok(ApproveCheckedKeys {
            token_account: Pubkey::new_from_array(self.token_account.pubkey_bytes()),
            delegate: self.delegate,
            authority,
            mint,
        })
    }

    fn initialized_token_account(&self) -> Result<InitializedTokenAccount<&A>, ProgramError> {
        ReadonlyTokenAccount(&self.token_account)
            .try_into_valid()?
            .try_into_initialized()
    }
}

impl<A: ReadonlyAccountData + ReadonlyAccountPubkeyBytes> TryFrom<ApproveFreeAccounts<A>>
    for ApproveKeys
{
    type Error = ProgramError;

    fn try_from(value: ApproveFreeAccounts<A>) -> Result<Self, Self::Error> {
        value.resolve()
    }
}

impl<A: ReadonlyAccountData + ReadonlyAccountPubkeyBytes> TryFrom<ApproveFreeAccounts<A>>
    for ApproveCheckedKeys
{
    type Error = ProgramError;

    fn try_from(value: ApproveFreeAccounts<A>) -> Result<Self, Self::Error> {
        value.resolve_checked()
    }
}
