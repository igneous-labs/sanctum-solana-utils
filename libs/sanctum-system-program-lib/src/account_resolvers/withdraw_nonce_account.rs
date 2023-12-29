use solana_program::{program_error::ProgramError, pubkey::Pubkey, sysvar};
use solana_readonly_account::{ReadonlyAccountData, ReadonlyAccountPubkey};
use system_program_interface::WithdrawNonceAccountKeys;

use crate::{NonceStateMarker, ReadonlyNonceAccount};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct WithdrawNonceAccountFreeAccounts<N> {
    pub nonce: N,
    pub to: Pubkey,
}

impl<N: ReadonlyAccountData + ReadonlyAccountPubkey> WithdrawNonceAccountFreeAccounts<N> {
    pub fn resolve(&self) -> Result<WithdrawNonceAccountKeys, ProgramError> {
        self.resolve_to_free_keys().map(Into::into)
    }

    pub fn resolve_to_free_keys(&self) -> Result<WithdrawNonceAccountFreeKeys, ProgramError> {
        let Self { nonce, to } = self;
        if !nonce.nonce_data_is_valid()
            || !matches!(nonce.nonce_state_marker(), NonceStateMarker::Initialized)
        {
            return Err(ProgramError::InvalidAccountData);
        }
        Ok(WithdrawNonceAccountFreeKeys {
            to: *to,
            nonce: *nonce.pubkey(),
            authority: nonce.nonce_data_authority_unchecked(),
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct WithdrawNonceAccountFreeKeys {
    pub nonce: Pubkey,
    pub to: Pubkey,
    pub authority: Pubkey,
}

impl WithdrawNonceAccountFreeKeys {
    pub fn resolve(&self) -> WithdrawNonceAccountKeys {
        let Self {
            nonce,
            authority,
            to,
        } = self;
        WithdrawNonceAccountKeys {
            nonce: *nonce,
            authority: *authority,
            to: *to,
            rent: sysvar::rent::ID,
            recent_blockhashes: sysvar::recent_blockhashes::ID,
        }
    }
}

impl From<WithdrawNonceAccountFreeKeys> for WithdrawNonceAccountKeys {
    fn from(value: WithdrawNonceAccountFreeKeys) -> Self {
        value.resolve()
    }
}
