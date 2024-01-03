use std::{error::Error, fmt::Display, hash::Hash, ops::Deref};

use solana_program::pubkey::Pubkey;
use solana_readonly_account::{
    ReadonlyAccountData, ReadonlyAccountIsExecutable, ReadonlyAccountLamports,
    ReadonlyAccountOwner, ReadonlyAccountRentEpoch,
};

pub const SMALL_ACCOUNT_DATA_MAX_LEN: u8 = 16;

pub const SMALL_ACCOUNT_DATA_MAX_LEN_USIZE: usize = SMALL_ACCOUNT_DATA_MAX_LEN as usize;

pub const SMALL_ACCOUNT_FLAGS_DATA_LEN_BITWIDTH: usize = 5;

pub const SMALL_ACCOUNT_FLAGS_IS_EXECUTABLE_BIT_OFFSET: usize = 7;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DataTooLong;

impl Display for DataTooLong {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Account data too long")
    }
}

impl Error for DataTooLong {}

/// Rightmost `SMALL_ACCOUNT_FLAGS_DATA_LEN_BITWIDTH` bits are account data len,
/// Leftmost bit is is_executable
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SmallAccountFlags(u8);

impl SmallAccountFlags {
    pub const IS_EXECUTABLE_AND_MASK: u8 = 0b1000_0000;

    pub const DATA_LEN_AND_MASK: u8 = 0b0001_1111;

    pub fn try_new(is_executable: bool, data_len: usize) -> Result<Self, DataTooLong> {
        if data_len > SMALL_ACCOUNT_DATA_MAX_LEN_USIZE {
            return Err(DataTooLong);
        }
        let base = if is_executable {
            Self::IS_EXECUTABLE_AND_MASK
        } else {
            0b0000_0000
        };
        let data_len: u8 = data_len.try_into().unwrap();
        Ok(Self(base | data_len))
    }

    pub fn is_executable(&self) -> bool {
        (self.0 & Self::IS_EXECUTABLE_AND_MASK) == Self::IS_EXECUTABLE_AND_MASK
    }

    pub fn data_len(&self) -> u8 {
        self.0 & Self::DATA_LEN_AND_MASK
    }
}

/// An account with data len < SMALL_ACCOUNT_DATA_MAX_LEN
/// that stores its data inline
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct SmallAccount {
    data: [u8; SMALL_ACCOUNT_DATA_MAX_LEN_USIZE], // data first so that it's always 8-byte aligned since this struct will be 8-byte aligned
    pub lamports: u64,
    pub rent_epoch: u64,
    pub owner: Pubkey,
    flags: SmallAccountFlags,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct SmallAccountTryNewParams<'a> {
    pub data: &'a [u8],
    pub lamports: u64,
    pub rent_epoch: u64,
    pub owner: Pubkey,
    pub executable: bool,
}

impl SmallAccount {
    pub fn try_new(
        SmallAccountTryNewParams {
            data,
            lamports,
            rent_epoch,
            owner,
            executable,
        }: SmallAccountTryNewParams,
    ) -> Result<Self, DataTooLong> {
        let len = data.len();
        let flags = SmallAccountFlags::try_new(executable, len)?;
        let mut res = Self {
            data: Default::default(),
            lamports,
            rent_epoch,
            owner,
            flags,
        };
        res.data[..len].copy_from_slice(data);
        Ok(res)
    }

    pub fn data_len(&self) -> u8 {
        self.flags.data_len()
    }

    pub fn data_slice(&self) -> &[u8] {
        &self.data[..self.data_len().into()]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SmallAccountDataRef<'a>(pub &'a [u8]);

impl<'a> Deref for SmallAccountDataRef<'a> {
    type Target = &'a [u8];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ReadonlyAccountData for SmallAccount {
    type SliceDeref<'s> = &'s[u8]
    where
        Self: 's;

    type DataDeref<'d> = SmallAccountDataRef<'d>
    where
        Self: 'd;

    fn data(&self) -> Self::DataDeref<'_> {
        SmallAccountDataRef(self.data_slice())
    }
}

impl ReadonlyAccountIsExecutable for SmallAccount {
    fn executable(&self) -> bool {
        self.flags.is_executable()
    }
}

impl ReadonlyAccountLamports for SmallAccount {
    fn lamports(&self) -> u64 {
        self.lamports
    }
}

impl ReadonlyAccountOwner for SmallAccount {
    fn owner(&self) -> &Pubkey {
        &self.owner
    }
}

impl ReadonlyAccountRentEpoch for SmallAccount {
    fn rent_epoch(&self) -> u64 {
        self.rent_epoch
    }
}

impl PartialEq for SmallAccount {
    fn eq(&self, other: &Self) -> bool {
        self.data_slice() == other.data_slice()
            && self.lamports == other.lamports
            && self.rent_epoch == other.rent_epoch
            && self.owner == other.owner
            && self.flags == other.flags
    }
}

impl Eq for SmallAccount {}

impl Hash for SmallAccount {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.data_slice().hash(state);
        self.lamports.hash(state);
        self.rent_epoch.hash(state);
        self.owner.hash(state);
        self.flags.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;
    use sanctum_solana_test_utils::proptest_utils::pubkey;
    use static_assertions::const_assert_eq;

    use super::*;

    const_assert_eq!(
        SmallAccountFlags::IS_EXECUTABLE_AND_MASK.trailing_zeros(),
        7
    );
    const_assert_eq!(SMALL_ACCOUNT_FLAGS_IS_EXECUTABLE_BIT_OFFSET, 7);
    const_assert_eq!(SmallAccountFlags::DATA_LEN_AND_MASK.trailing_ones(), 5);
    const_assert_eq!(SMALL_ACCOUNT_FLAGS_DATA_LEN_BITWIDTH, 5);
    const_assert_eq!(SMALL_ACCOUNT_DATA_MAX_LEN.leading_zeros(), 8 - 5);

    proptest! {
        #[test]
        fn eq_check(
            owner in pubkey(),
            data in proptest::collection::vec(any::<u8>(), 0..=SMALL_ACCOUNT_DATA_MAX_LEN_USIZE),
            executable: bool,
            lamports: u64,
            rent_epoch: u64,
        ) {
            let small = SmallAccount::try_new(SmallAccountTryNewParams {
                data: &data,
                lamports,
                rent_epoch,
                owner,
                executable
            }).unwrap();

            prop_assert_eq!(*small.data(), data.as_slice());
            prop_assert_eq!(small.executable(), executable);
            prop_assert_eq!(*small.owner(), owner);
            prop_assert_eq!(small.lamports(), lamports);
            prop_assert_eq!(small.rent_epoch(), rent_epoch);
        }
    }

    proptest! {
        #[test]
        fn too_large_fails(
            owner in pubkey(),
            data in proptest::collection::vec(any::<u8>(), SMALL_ACCOUNT_DATA_MAX_LEN_USIZE+1..=1024),
            executable: bool,
            lamports: u64,
            rent_epoch: u64,
        ) {
            prop_assert_eq!(SmallAccount::try_new(SmallAccountTryNewParams {
                data: &data,
                lamports,
                rent_epoch,
                owner,
                executable
            }).unwrap_err(), DataTooLong);
        }
    }
}
