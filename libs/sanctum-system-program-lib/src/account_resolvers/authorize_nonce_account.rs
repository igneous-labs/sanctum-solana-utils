use solana_program::{program_error::ProgramError, pubkey::Pubkey};
use solana_readonly_account::{ReadonlyAccountData, ReadonlyAccountPubkeyBytes};
use system_program_interface::AuthorizeNonceAccountKeys;

use crate::ReadonlyNonceAccount;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AuthorizeNonceAccountFreeAccounts<N> {
    pub nonce: N,
}

impl<N: ReadonlyAccountData + ReadonlyAccountPubkeyBytes> AuthorizeNonceAccountFreeAccounts<N> {
    pub fn resolve(&self) -> Result<AuthorizeNonceAccountKeys, ProgramError> {
        let Self { nonce } = self;
        let n = ReadonlyNonceAccount(nonce);
        let n = n.try_into_valid()?;
        let n = n.try_into_initialized()?;
        Ok(AuthorizeNonceAccountKeys {
            nonce: Pubkey::new_from_array(nonce.pubkey_bytes()),
            authority: n.nonce_data_authority(),
        })
    }
}
