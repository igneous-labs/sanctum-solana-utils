use solana_program::{program_error::ProgramError, pubkey::Pubkey};
use solana_readonly_account::{ReadonlyAccountData, ReadonlyAccountPubkeyBytes};
use spl_token_interface::CloseAccountKeys;

use crate::ReadonlyTokenAccount;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CloseAccountFreeAccounts<A> {
    pub token_account: A,
    pub to: Pubkey,
}

impl<A: ReadonlyAccountData + ReadonlyAccountPubkeyBytes> CloseAccountFreeAccounts<A> {
    pub fn resolve(&self) -> Result<CloseAccountKeys, ProgramError> {
        let Self { token_account, to } = self;

        let t = ReadonlyTokenAccount(token_account)
            .try_into_valid()?
            .try_into_initialized()?;

        Ok(CloseAccountKeys {
            token_account: Pubkey::new_from_array(token_account.pubkey_bytes()),
            authority: t.token_account_authority(),
            to: *to,
        })
    }
}

impl<A: ReadonlyAccountData + ReadonlyAccountPubkeyBytes> TryFrom<CloseAccountFreeAccounts<A>>
    for CloseAccountKeys
{
    type Error = ProgramError;

    fn try_from(value: CloseAccountFreeAccounts<A>) -> Result<Self, Self::Error> {
        value.resolve()
    }
}
