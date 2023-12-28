use solana_program::{program_error::ProgramError, pubkey::Pubkey};
use solana_readonly_account::{ReadonlyAccountData, ReadonlyAccountPubkey};
use spl_token_interface::{BurnCheckedKeys, BurnKeys};

use crate::ReadonlyTokenAccount;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BurnFreeAccounts<A> {
    pub token_account: A,
}

impl<A: ReadonlyAccountData + ReadonlyAccountPubkey> BurnFreeAccounts<A> {
    pub fn resolve(&self) -> Result<BurnKeys, ProgramError> {
        let mint = self.token_account_mint()?;
        Ok(BurnKeys {
            token_account: *self.token_account.pubkey(),
            authority: self.token_account.token_account_authority(),
            mint,
        })
    }

    pub fn resolve_checked(&self) -> Result<BurnCheckedKeys, ProgramError> {
        let mint = self.token_account_mint()?;
        Ok(BurnCheckedKeys {
            token_account: *self.token_account.pubkey(),
            authority: self.token_account.token_account_authority(),
            mint,
        })
    }

    fn token_account_mint(&self) -> Result<Pubkey, ProgramError> {
        if !self.token_account.token_account_data_is_valid()
            || !self.token_account.token_account_is_initialized()
        {
            return Err(ProgramError::InvalidAccountData);
        }
        Ok(self.token_account.token_account_mint())
    }
}

impl<A: ReadonlyAccountData + ReadonlyAccountPubkey> TryFrom<BurnFreeAccounts<A>> for BurnKeys {
    type Error = ProgramError;

    fn try_from(value: BurnFreeAccounts<A>) -> Result<Self, Self::Error> {
        value.resolve()
    }
}

impl<A: ReadonlyAccountData + ReadonlyAccountPubkey> TryFrom<BurnFreeAccounts<A>>
    for BurnCheckedKeys
{
    type Error = ProgramError;

    fn try_from(value: BurnFreeAccounts<A>) -> Result<Self, Self::Error> {
        value.resolve_checked()
    }
}
