#[cfg(feature = "solana-sdk")]
#[cfg_attr(docsrs, doc(cfg(feature = "solana-sdk")))]
pub mod solana_sdk_conv;

mod arc_account;
mod small_account;

use std::{ops::Deref, sync::Arc};

pub use arc_account::*;
pub use small_account::*;
use solana_program::pubkey::Pubkey;
use solana_readonly_account::{
    ReadonlyAccountData, ReadonlyAccountIsExecutable, ReadonlyAccountLamports,
    ReadonlyAccountOwner, ReadonlyAccountRentEpoch,
};

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
