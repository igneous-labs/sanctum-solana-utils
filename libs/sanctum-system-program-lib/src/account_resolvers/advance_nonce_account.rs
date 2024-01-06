use solana_program::{program_error::ProgramError, pubkey::Pubkey, sysvar};
use solana_readonly_account::{ReadonlyAccountData, ReadonlyAccountPubkey};
use system_program_interface::AdvanceNonceAccountKeys;

use crate::ReadonlyNonceAccount;

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
        let n = ReadonlyNonceAccount(nonce);
        let n = n.try_into_valid()?;
        let n = n.try_into_initialized()?;
        Ok(AdvanceNonceAccountFreeKeys {
            nonce: *nonce.pubkey(),
            authority: n.nonce_data_authority(),
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
