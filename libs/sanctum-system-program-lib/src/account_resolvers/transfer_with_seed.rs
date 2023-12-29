use solana_program::pubkey::{Pubkey, PubkeyError};
use system_program_interface::TransferWithSeedKeys;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TransferWithSeedFreeKeys {
    pub base: Pubkey,
    pub to: Pubkey,
}

impl TransferWithSeedFreeKeys {
    pub fn resolve(
        &self,
        from_seed: &str,
        from_owner: Pubkey,
    ) -> Result<TransferWithSeedKeys, PubkeyError> {
        let Self { base, to } = self;
        let from = Pubkey::create_with_seed(base, from_seed, &from_owner)?;
        Ok(TransferWithSeedKeys {
            base: *base,
            to: *to,
            from,
        })
    }
}
