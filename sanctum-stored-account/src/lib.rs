use solana_readonly_account::{
    ReadonlyAccountData, ReadonlyAccountIsExecutable, ReadonlyAccountLamports,
    ReadonlyAccountOwnerBytes, ReadonlyAccountRentEpoch,
};

#[cfg(feature = "solana-sdk")]
#[cfg_attr(docsrs, doc(cfg(feature = "solana-sdk")))]
pub mod solana_sdk_conv;

mod arc_account;
mod small_account;

pub use arc_account::*;
pub use small_account::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum StoredAccount {
    Small(SmallAccount),
    Arc(ArcAccount),
}

impl Default for StoredAccount {
    fn default() -> Self {
        Self::Small(SmallAccount::default())
    }
}

impl ReadonlyAccountData for StoredAccount {
    type DataDeref<'d> = &'d [u8]
    where
        Self: 'd;

    #[inline]
    fn data(&self) -> Self::DataDeref<'_> {
        match self {
            Self::Arc(a) => a.data(),
            Self::Small(s) => s.data(),
        }
    }
}

impl ReadonlyAccountIsExecutable for StoredAccount {
    #[inline]
    fn is_executable(&self) -> bool {
        match self {
            Self::Arc(a) => a.is_executable(),
            Self::Small(s) => s.is_executable(),
        }
    }
}

impl ReadonlyAccountLamports for StoredAccount {
    #[inline]
    fn lamports(&self) -> u64 {
        match self {
            Self::Arc(a) => a.lamports(),
            Self::Small(s) => s.lamports(),
        }
    }
}

impl ReadonlyAccountOwnerBytes for StoredAccount {
    #[inline]
    fn owner_bytes(&self) -> [u8; 32] {
        match self {
            Self::Arc(a) => a.owner_bytes(),
            Self::Small(s) => s.owner_bytes(),
        }
    }
}

impl ReadonlyAccountRentEpoch for StoredAccount {
    #[inline]
    fn rent_epoch(&self) -> u64 {
        match self {
            Self::Arc(a) => a.rent_epoch(),
            Self::Small(s) => s.rent_epoch(),
        }
    }
}

#[cfg(test)]
mod tests {
    use static_assertions::{const_assert, const_assert_eq};
    use std::mem::{align_of, size_of};

    use super::*;

    const_assert_eq!(size_of::<ArcAccount>(), 72);
    const_assert_eq!(size_of::<SmallAccount>(), size_of::<ArcAccount>());
    const_assert_eq!(size_of::<StoredAccount>(), 80);
    const_assert!(size_of::<StoredAccount>() <= size_of::<solana_sdk::account::Account>());

    const_assert_eq!(align_of::<ArcAccount>(), 8);
    const_assert_eq!(align_of::<SmallAccount>(), 8);
    const_assert_eq!(align_of::<StoredAccount>(), 8);
}
