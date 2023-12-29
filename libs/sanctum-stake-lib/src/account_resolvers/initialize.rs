use solana_program::{pubkey::Pubkey, sysvar};
use stake_program_interface::InitializeKeys;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct InitializeFreeKeys {
    pub stake: Pubkey,
}

impl InitializeFreeKeys {
    pub fn resolve(&self) -> InitializeKeys {
        InitializeKeys {
            stake: self.stake,
            rent: sysvar::rent::ID,
        }
    }
}

impl From<InitializeFreeKeys> for InitializeKeys {
    fn from(value: InitializeFreeKeys) -> Self {
        value.resolve()
    }
}
