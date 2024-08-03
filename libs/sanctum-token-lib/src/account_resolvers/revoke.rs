use solana_program::{program_error::ProgramError, pubkey::Pubkey};
use solana_readonly_account::{ReadonlyAccountData, ReadonlyAccountPubkeyBytes};
use spl_token_interface::RevokeKeys;

use crate::ReadonlyTokenAccount;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RevokeFreeAccounts<A> {
    pub token_account: A,
}

impl<A: ReadonlyAccountData + ReadonlyAccountPubkeyBytes> RevokeFreeAccounts<A> {
    pub fn resolve(&self) -> Result<RevokeKeys, ProgramError> {
        let Self { token_account } = self;
        let t = ReadonlyTokenAccount(&self.token_account)
            .try_into_valid()?
            .try_into_initialized()?;
        let authority = t.token_account_authority();
        Ok(RevokeKeys {
            token_account: Pubkey::new_from_array(token_account.pubkey_bytes()),
            authority,
        })
    }
}

impl<A: ReadonlyAccountData + ReadonlyAccountPubkeyBytes> TryFrom<RevokeFreeAccounts<A>>
    for RevokeKeys
{
    type Error = ProgramError;

    fn try_from(value: RevokeFreeAccounts<A>) -> Result<Self, Self::Error> {
        value.resolve()
    }
}
