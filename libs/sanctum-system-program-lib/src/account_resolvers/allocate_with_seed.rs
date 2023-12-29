use solana_program::pubkey::{Pubkey, PubkeyError};
use system_program_interface::AllocateWithSeedKeys;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AllocateWithSeedFreeKeys {
    pub base: Pubkey,
}

impl AllocateWithSeedFreeKeys {
    pub fn resolve(&self, seed: &str, owner: Pubkey) -> Result<AllocateWithSeedKeys, PubkeyError> {
        let Self { base } = self;
        let allocate = Pubkey::create_with_seed(base, seed, &owner)?;
        Ok(AllocateWithSeedKeys {
            allocate,
            base: *base,
        })
    }
}
