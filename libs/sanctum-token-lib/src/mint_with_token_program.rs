use solana_program::pubkey::Pubkey;
use solana_readonly_account::{ReadonlyAccountOwnerBytes, ReadonlyAccountPubkeyBytes};

/// A mint and its owner token program
#[derive(Default, Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct MintWithTokenProgram {
    /// The mint's pubkey
    pub pubkey: Pubkey,

    /// The mint's owner token program
    pub token_program: Pubkey,
}

impl ReadonlyAccountOwnerBytes for MintWithTokenProgram {
    fn owner_bytes(&self) -> [u8; 32] {
        self.token_program.to_bytes()
    }
}

impl ReadonlyAccountPubkeyBytes for MintWithTokenProgram {
    fn pubkey_bytes(&self) -> [u8; 32] {
        self.pubkey.to_bytes()
    }
}
