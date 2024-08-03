#![cfg(feature = "keyed")]

use solana_program::pubkey::Pubkey;

use crate::{
    ReadonlyAccountData, ReadonlyAccountIsExecutable, ReadonlyAccountLamports,
    ReadonlyAccountOwnerBytes, ReadonlyAccountPubkeyBytes, ReadonlyAccountRentEpoch,
};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Keyed<T> {
    pub pubkey: Pubkey,
    pub account: T,
}

impl<T> ReadonlyAccountPubkeyBytes for Keyed<T> {
    fn pubkey_bytes(&self) -> [u8; 32] {
        self.pubkey.to_bytes()
    }
}

impl<T: ReadonlyAccountLamports> ReadonlyAccountLamports for Keyed<T> {
    fn lamports(&self) -> u64 {
        self.account.lamports()
    }
}

impl<T: ReadonlyAccountData> ReadonlyAccountData for Keyed<T> {
    type DataDeref<'d> = T::DataDeref<'d> where Self: 'd;

    fn data(&self) -> Self::DataDeref<'_> {
        self.account.data()
    }
}

impl<T: ReadonlyAccountOwnerBytes> ReadonlyAccountOwnerBytes for Keyed<T> {
    fn owner_bytes(&self) -> [u8; 32] {
        self.account.owner_bytes()
    }
}

impl<T: ReadonlyAccountIsExecutable> ReadonlyAccountIsExecutable for Keyed<T> {
    fn is_executable(&self) -> bool {
        self.account.is_executable()
    }
}

impl<T: ReadonlyAccountRentEpoch> ReadonlyAccountRentEpoch for Keyed<T> {
    fn rent_epoch(&self) -> u64 {
        self.account.rent_epoch()
    }
}
