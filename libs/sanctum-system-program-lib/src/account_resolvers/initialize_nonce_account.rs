use solana_program::{pubkey::Pubkey, sysvar};
use system_program_interface::InitializeNonceAccountKeys;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct InitializeNonceAccountFreeKeys {
    pub nonce: Pubkey,
}

impl InitializeNonceAccountFreeKeys {
    pub fn resolve(&self) -> InitializeNonceAccountKeys {
        let Self { nonce } = self;
        InitializeNonceAccountKeys {
            nonce: *nonce,
            recent_blockhashes: sysvar::recent_blockhashes::ID,
            rent: sysvar::rent::ID,
        }
    }
}

impl From<InitializeNonceAccountFreeKeys> for InitializeNonceAccountKeys {
    fn from(value: InitializeNonceAccountFreeKeys) -> Self {
        value.resolve()
    }
}
