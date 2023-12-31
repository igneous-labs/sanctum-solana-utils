use solana_program::pubkey::Pubkey;
use solana_readonly_account::{
    ReadonlyAccountData, ReadonlyAccountIsExecutable, ReadonlyAccountLamports,
    ReadonlyAccountOwner, ReadonlyAccountRentEpoch,
};
use std::{ops::Deref, sync::Arc};

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

pub enum StoredAccountDataDeref<'a> {
    Small(SmallAccountDataRef<'a>),
    Arc(&'a Arc<[u8]>),
}

impl<'a> Deref for StoredAccountDataDeref<'a> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Arc(a) => a,
            Self::Small(s) => **s,
        }
    }
}

// Newtype to workaround cannot return reference to temporary
pub struct StoredAccountDataRef<'a>(pub StoredAccountDataDeref<'a>);

impl<'a> Deref for StoredAccountDataRef<'a> {
    type Target = StoredAccountDataDeref<'a>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ReadonlyAccountData for StoredAccount {
    type SliceDeref<'s> = StoredAccountDataDeref<'s>
    where
        Self: 's;

    type DataDeref<'d> = StoredAccountDataRef<'d>
    where
        Self: 'd;

    fn data(&self) -> Self::DataDeref<'_> {
        StoredAccountDataRef(match self {
            Self::Arc(a) => StoredAccountDataDeref::Arc(a.data()),
            Self::Small(s) => StoredAccountDataDeref::Small(s.data()),
        })
    }
}

impl ReadonlyAccountIsExecutable for StoredAccount {
    fn executable(&self) -> bool {
        match self {
            Self::Arc(a) => a.executable(),
            Self::Small(s) => s.executable(),
        }
    }
}

impl ReadonlyAccountLamports for StoredAccount {
    fn lamports(&self) -> u64 {
        match self {
            Self::Arc(a) => a.lamports(),
            Self::Small(s) => s.lamports(),
        }
    }
}

impl ReadonlyAccountOwner for StoredAccount {
    fn owner(&self) -> &Pubkey {
        match self {
            Self::Arc(a) => a.owner(),
            Self::Small(s) => s.owner(),
        }
    }
}

impl ReadonlyAccountRentEpoch for StoredAccount {
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
