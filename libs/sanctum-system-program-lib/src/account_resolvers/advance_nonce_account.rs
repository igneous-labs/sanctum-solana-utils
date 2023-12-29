use solana_program::{program_error::ProgramError, pubkey::Pubkey, sysvar};
use solana_readonly_account::{ReadonlyAccountData, ReadonlyAccountPubkey};
use system_program_interface::AdvanceNonceAccountKeys;

use crate::{NonceStateMarker, ReadonlyNonceAccount};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AdvanceNonceAccountFreeAccounts<N> {
    pub nonce: N,
}

impl<N: ReadonlyAccountData + ReadonlyAccountPubkey> AdvanceNonceAccountFreeAccounts<N> {
    pub fn resolve(&self) -> Result<AdvanceNonceAccountKeys, ProgramError> {
        self.resolve_to_free_keys().map(Into::into)
    }

    pub fn resolve_to_free_keys(&self) -> Result<AdvanceNonceAccountFreeKeys, ProgramError> {
        let Self { nonce } = self;
        if !nonce.nonce_data_is_valid()
            || !matches!(nonce.nonce_state_marker(), NonceStateMarker::Initialized)
        {
            return Err(ProgramError::InvalidAccountData);
        }
        Ok(AdvanceNonceAccountFreeKeys {
            nonce: *nonce.pubkey(),
            authority: nonce.nonce_data_authority_unchecked(),
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AdvanceNonceAccountFreeKeys {
    pub nonce: Pubkey,
    pub authority: Pubkey,
}

impl AdvanceNonceAccountFreeKeys {
    pub fn resolve(&self) -> AdvanceNonceAccountKeys {
        let Self { nonce, authority } = self;
        AdvanceNonceAccountKeys {
            nonce: *nonce,
            recent_blockhashes: sysvar::recent_blockhashes::ID,
            authority: *authority,
        }
    }
}

impl From<AdvanceNonceAccountFreeKeys> for AdvanceNonceAccountKeys {
    fn from(value: AdvanceNonceAccountFreeKeys) -> Self {
        value.resolve()
    }
}
