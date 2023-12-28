use solana_program::{pubkey::Pubkey, sysvar};
use spl_token_interface::InitializeMintKeys;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InitializeMintFreeKeys {
    pub mint: Pubkey,
}

impl InitializeMintFreeKeys {
    pub fn resolve(&self) -> InitializeMintKeys {
        InitializeMintKeys {
            mint: self.mint,
            rent: sysvar::rent::ID,
        }
    }
}

impl From<InitializeMintFreeKeys> for InitializeMintKeys {
    fn from(value: InitializeMintFreeKeys) -> Self {
        value.resolve()
    }
}
