use solana_program::pubkey::{Pubkey, PubkeyError};
use system_program_interface::CreateAccountWithSeedKeys;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CreateAccountWithSeedFreeKeys {
    pub from: Pubkey,
    pub base: Pubkey,
}

impl CreateAccountWithSeedFreeKeys {
    pub fn resolve(
        &self,
        seed: &str,
        owner: Pubkey,
    ) -> Result<CreateAccountWithSeedKeys, PubkeyError> {
        let Self { from, base } = self;
        let to = Pubkey::create_with_seed(base, seed, &owner)?;
        Ok(CreateAccountWithSeedKeys {
            from: *from,
            to,
            base: *base,
        })
    }
}
