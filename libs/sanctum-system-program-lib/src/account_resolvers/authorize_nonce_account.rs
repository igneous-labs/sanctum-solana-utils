use solana_program::program_error::ProgramError;
use solana_readonly_account::{ReadonlyAccountData, ReadonlyAccountPubkey};
use system_program_interface::AuthorizeNonceAccountKeys;

use crate::{NonceStateMarker, ReadonlyNonceAccount};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AuthorizeNonceAccountFreeAccounts<N> {
    pub nonce: N,
}

impl<N: ReadonlyAccountData + ReadonlyAccountPubkey> AuthorizeNonceAccountFreeAccounts<N> {
    pub fn resolve(&self) -> Result<AuthorizeNonceAccountKeys, ProgramError> {
        let Self { nonce } = self;
        if !nonce.nonce_data_is_valid()
            || !matches!(nonce.nonce_state_marker(), NonceStateMarker::Initialized)
        {
            return Err(ProgramError::InvalidAccountData);
        }
        Ok(AuthorizeNonceAccountKeys {
            nonce: *nonce.pubkey(),
            authority: nonce.nonce_data_authority_unchecked(),
        })
    }
}
