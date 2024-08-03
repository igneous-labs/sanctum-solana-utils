#![cfg(all(feature = "keyed", feature = "keyed-bytes"))]

use solana_sdk::pubkey::Pubkey;

use crate::{keyed::Keyed, keyed_bytes::KeyedBytes};

impl<T> From<Keyed<T>> for KeyedBytes<T> {
    fn from(Keyed { pubkey, account }: Keyed<T>) -> Self {
        Self {
            pubkey_bytes: pubkey.to_bytes(),
            account,
        }
    }
}

impl<T> From<KeyedBytes<T>> for Keyed<T> {
    fn from(
        KeyedBytes {
            pubkey_bytes,
            account,
        }: KeyedBytes<T>,
    ) -> Self {
        Self {
            pubkey: Pubkey::new_from_array(pubkey_bytes),
            account,
        }
    }
}
