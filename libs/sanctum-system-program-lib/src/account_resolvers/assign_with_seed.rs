use solana_program::pubkey::{Pubkey, PubkeyError};
use system_program_interface::AssignWithSeedKeys;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AssignWithSeedFreeKeys {
    pub base: Pubkey,
}

impl AssignWithSeedFreeKeys {
    pub fn resolve(&self, seed: &str, owner: Pubkey) -> Result<AssignWithSeedKeys, PubkeyError> {
        let Self { base } = self;
        let assign = Pubkey::create_with_seed(base, seed, &owner)?;
        Ok(AssignWithSeedKeys {
            assign,
            base: *base,
        })
    }
}
