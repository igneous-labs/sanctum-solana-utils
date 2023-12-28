use solana_program::{pubkey::Pubkey, sysvar};
use spl_token_interface::InitializeAccountKeys;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InitializeAccountFreeKeys {
    pub token_account: Pubkey,
    pub mint: Pubkey,
    pub authority: Pubkey,
}

impl InitializeAccountFreeKeys {
    pub fn resolve(&self) -> InitializeAccountKeys {
        InitializeAccountKeys {
            token_account: self.token_account,
            mint: self.mint,
            authority: self.authority,
            rent: sysvar::rent::ID,
        }
    }
}

impl From<InitializeAccountFreeKeys> for InitializeAccountKeys {
    fn from(value: InitializeAccountFreeKeys) -> Self {
        value.resolve()
    }
}
