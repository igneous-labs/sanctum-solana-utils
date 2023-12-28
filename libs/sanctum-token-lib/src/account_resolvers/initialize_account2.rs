use solana_program::{pubkey::Pubkey, sysvar};
use spl_token_interface::InitializeAccount2Keys;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InitializeAccount2FreeKeys {
    pub token_account: Pubkey,
    pub mint: Pubkey,
}

impl InitializeAccount2FreeKeys {
    pub fn resolve(&self) -> InitializeAccount2Keys {
        InitializeAccount2Keys {
            token_account: self.token_account,
            mint: self.mint,
            rent: sysvar::rent::ID,
        }
    }
}

impl From<InitializeAccount2FreeKeys> for InitializeAccount2Keys {
    fn from(value: InitializeAccount2FreeKeys) -> Self {
        value.resolve()
    }
}
