use std::sync::Arc;

use solana_program::pubkey::Pubkey;
use solana_readonly_account::{
    ReadonlyAccountData, ReadonlyAccountIsExecutable, ReadonlyAccountLamports,
    ReadonlyAccountOwner, ReadonlyAccountRentEpoch,
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
    type SliceDeref<'s> = Arc<[u8]>
    where
        Self: 's;

    type DataDeref<'d> = &'d Arc<[u8]>
    where
        Self: 'd;

    fn data(&self) -> Self::DataDeref<'_> {
        &self.data
    }
}

impl ReadonlyAccountIsExecutable for ArcAccount {
    fn executable(&self) -> bool {
        self.executable
    }
}

impl ReadonlyAccountLamports for ArcAccount {
    fn lamports(&self) -> u64 {
        self.lamports
    }
}

impl ReadonlyAccountOwner for ArcAccount {
    fn owner(&self) -> &Pubkey {
        &self.owner
    }
}

impl ReadonlyAccountRentEpoch for ArcAccount {
    fn rent_epoch(&self) -> u64 {
        self.rent_epoch
    }
}
