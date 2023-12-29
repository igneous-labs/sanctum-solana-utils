use solana_program::{pubkey::Pubkey, sysvar};
use stake_program_interface::InitializeCheckedKeys;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct InitializeCheckedFreeKeys {
    pub stake: Pubkey,
    pub stake_authority: Pubkey,
    pub withdraw_authority: Pubkey,
}

impl InitializeCheckedFreeKeys {
    pub fn resolve(&self) -> InitializeCheckedKeys {
        let Self {
            stake,
            stake_authority,
            withdraw_authority,
        } = self;
        InitializeCheckedKeys {
            stake: *stake,
            stake_authority: *stake_authority,
            withdraw_authority: *withdraw_authority,
            rent: sysvar::rent::ID,
        }
    }
}

impl From<InitializeCheckedFreeKeys> for InitializeCheckedKeys {
    fn from(value: InitializeCheckedFreeKeys) -> Self {
        value.resolve()
    }
}
