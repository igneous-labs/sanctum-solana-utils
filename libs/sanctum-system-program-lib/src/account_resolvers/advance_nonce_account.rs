use solana_program::{program_error::ProgramError, pubkey::Pubkey, sysvar};
use solana_readonly_account::{ReadonlyAccountData, ReadonlyAccountPubkey};
use system_program_interface::AdvanceNonceAccountKeys;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AdvanceNonceAccountFreeAccounts<N> {
    pub nonce: N,
}

impl<N: ReadonlyAccountData + ReadonlyAccountPubkey> AdvanceNonceAccountFreeAccounts<N> {
    pub fn resolve_to_free_keys(&self) -> Result<AdvanceNonceAccountFreeKeys, ProgramError> {
        // let Self { nonce } = self;
        // let state: nonce::State = bincode::deserialize(&nonce.data())?;
        todo!()
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
