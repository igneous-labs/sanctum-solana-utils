use std::sync::Arc;

use solana_program::pubkey::Pubkey;
use solana_readonly_account::{
    ReadonlyAccountData, ReadonlyAccountIsExecutable, ReadonlyAccountLamports,
    ReadonlyAccountOwnerBytes, ReadonlyAccountRentEpoch,
};

/// An account with data pointed to with an Arc<[u8]> instead of Vec<u8>.
#[repr(C)]
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct ArcAccount {
    pub data: Arc<[u8]>,
    pub lamports: u64,
    pub rent_epoch: u64,
    pub owner: Pubkey,
    pub executable: bool,
}

impl ReadonlyAccountData for ArcAccount {
    type DataDeref<'d> = &'d [u8]
    where
        Self: 'd;

    #[inline]
    fn data(&self) -> Self::DataDeref<'_> {
        &self.data
    }
}

impl ReadonlyAccountIsExecutable for ArcAccount {
    #[inline]
    fn is_executable(&self) -> bool {
        self.executable
    }
}

impl ReadonlyAccountLamports for ArcAccount {
    #[inline]
    fn lamports(&self) -> u64 {
        self.lamports
    }
}

impl ReadonlyAccountOwnerBytes for ArcAccount {
    #[inline]
    fn owner_bytes(&self) -> [u8; 32] {
        self.owner.to_bytes()
    }
}

impl ReadonlyAccountRentEpoch for ArcAccount {
    #[inline]
    fn rent_epoch(&self) -> u64 {
        self.rent_epoch
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;
    use sanctum_solana_test_utils::proptest_utils::pubkey;

    use super::*;

    proptest! {
        #[test]
        fn eq_check(
            owner in pubkey(),
            data in proptest::collection::vec(any::<u8>(), 0..=1024),
            executable: bool,
            lamports: u64,
            rent_epoch: u64,
        ) {
            let acc = ArcAccount {
                data: data.clone().into(),
                lamports,
                rent_epoch,
                owner,
                executable
            };

            prop_assert_eq!(acc.data(), data);
            prop_assert_eq!(acc.is_executable(), executable);
            prop_assert_eq!(acc.owner_bytes(), owner.to_bytes());
            prop_assert_eq!(acc.lamports(), lamports);
            prop_assert_eq!(acc.rent_epoch(), rent_epoch);
        }
    }
}
