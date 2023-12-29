use solana_program::{pubkey::Pubkey, sysvar};
use stake_program_interface::AuthorizeWithSeedKeys;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AuthorizeWithSeedFreeKeys {
    pub stake: Pubkey,
    pub authority_base: Pubkey,
}

impl AuthorizeWithSeedFreeKeys {
    pub fn resolve(&self) -> AuthorizeWithSeedKeys {
        let Self {
            stake,
            authority_base,
        } = self;
        AuthorizeWithSeedKeys {
            stake: *stake,
            authority_base: *authority_base,
            clock: sysvar::clock::ID,
        }
    }
}

impl From<AuthorizeWithSeedFreeKeys> for AuthorizeWithSeedKeys {
    fn from(value: AuthorizeWithSeedFreeKeys) -> Self {
        value.resolve()
    }
}
