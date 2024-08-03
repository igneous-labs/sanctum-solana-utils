#![cfg(feature = "keyed-bytes")]

use crate::{
    ReadonlyAccountData, ReadonlyAccountIsExecutable, ReadonlyAccountLamports,
    ReadonlyAccountOwnerBytes, ReadonlyAccountPubkeyBytes, ReadonlyAccountRentEpoch,
};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KeyedBytes<T> {
    pub pubkey_bytes: [u8; 32],
    pub account: T,
}

impl<T> ReadonlyAccountPubkeyBytes for KeyedBytes<T> {
    fn pubkey_bytes(&self) -> [u8; 32] {
        self.pubkey_bytes
    }
}

impl<T: ReadonlyAccountLamports> ReadonlyAccountLamports for KeyedBytes<T> {
    fn lamports(&self) -> u64 {
        self.account.lamports()
    }
}

impl<T: ReadonlyAccountData> ReadonlyAccountData for KeyedBytes<T> {
    type DataDeref<'d> = T::DataDeref<'d> where Self: 'd;

    fn data(&self) -> Self::DataDeref<'_> {
        self.account.data()
    }
}

impl<T: ReadonlyAccountOwnerBytes> ReadonlyAccountOwnerBytes for KeyedBytes<T> {
    fn owner_bytes(&self) -> [u8; 32] {
        self.account.owner_bytes()
    }
}

impl<T: ReadonlyAccountIsExecutable> ReadonlyAccountIsExecutable for KeyedBytes<T> {
    fn is_executable(&self) -> bool {
        self.account.is_executable()
    }
}

impl<T: ReadonlyAccountRentEpoch> ReadonlyAccountRentEpoch for KeyedBytes<T> {
    fn rent_epoch(&self) -> u64 {
        self.account.rent_epoch()
    }
}
