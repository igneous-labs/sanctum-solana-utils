use solana_program::{pubkey::Pubkey, sysvar};
use spl_token_interface::InitializeMultisigKeys;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InitializeMultisigFreeKeys {
    pub multisig: Pubkey,
}

impl InitializeMultisigFreeKeys {
    pub fn resolve(&self) -> InitializeMultisigKeys {
        InitializeMultisigKeys {
            rent: sysvar::rent::ID,
            multisig: self.multisig,
        }
    }
}

impl From<InitializeMultisigFreeKeys> for InitializeMultisigKeys {
    fn from(value: InitializeMultisigFreeKeys) -> Self {
        value.resolve()
    }
}
