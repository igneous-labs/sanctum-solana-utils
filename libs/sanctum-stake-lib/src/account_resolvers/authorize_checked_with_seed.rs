use solana_program::{pubkey::Pubkey, sysvar};
use stake_program_interface::AuthorizeCheckedWithSeedKeys;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AuthorizeCheckedWithSeedFreeKeys {
    pub stake: Pubkey,
    pub authority_base: Pubkey,
    pub new_authority: Pubkey,
}

impl AuthorizeCheckedWithSeedFreeKeys {
    pub fn resolve(&self) -> AuthorizeCheckedWithSeedKeys {
        let Self {
            stake,
            authority_base,
            new_authority,
        } = self;
        AuthorizeCheckedWithSeedKeys {
            stake: *stake,
            authority_base: *authority_base,
            new_authority: *new_authority,
            clock: sysvar::clock::ID,
        }
    }
}

impl From<AuthorizeCheckedWithSeedFreeKeys> for AuthorizeCheckedWithSeedKeys {
    fn from(value: AuthorizeCheckedWithSeedFreeKeys) -> Self {
        value.resolve()
    }
}
