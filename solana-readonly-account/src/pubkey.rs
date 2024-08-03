#![cfg(feature = "solana-pubkey")]

use solana_program::pubkey::Pubkey;

use crate::{ReadonlyAccountOwnerBytes, ReadonlyAccountPubkeyBytes};

/// [`ReadonlyAccountPubkeyBytes`] but with [`Pubkey`] instead of `[u8; 32]`
pub trait ReadonlyAccountPubkey {
    /// Returns the pubkey of this account
    fn pubkey(&self) -> Pubkey;
}

impl<T: ReadonlyAccountPubkeyBytes> ReadonlyAccountPubkey for T {
    fn pubkey(&self) -> Pubkey {
        Pubkey::new_from_array(self.pubkey_bytes())
    }
}

/// [`ReadonlyAccountOwnerBytes`] but with [`Pubkey`] instead of `[u8; 32]`
pub trait ReadonlyAccountOwner {
    /// Returns the pubkey of the program owning this account
    fn owner(&self) -> Pubkey;
}

impl<T: ReadonlyAccountOwnerBytes> ReadonlyAccountOwner for T {
    fn owner(&self) -> Pubkey {
        Pubkey::new_from_array(self.owner_bytes())
    }
}
