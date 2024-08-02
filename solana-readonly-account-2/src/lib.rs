#![cfg_attr(not(test), no_std)]
#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod keyed;
pub mod keyed_bytes;
pub mod program;
pub mod pubkey;
pub mod sdk;

use core::ops::Deref;

/// A readonly account that you can read the pubkey of
pub trait ReadonlyAccountPubkeyBytes {
    /// Returns the pubkey bytes of this account
    fn pubkey_bytes(&self) -> [u8; 32];
}

/// A readonly account that you can read the lamports of
pub trait ReadonlyAccountLamports {
    /// Returns the lamports of this account
    fn lamports(&self) -> u64;
}

/// A readonly account that you can read the data of
pub trait ReadonlyAccountData {
    type DataDeref<'d>: Deref<Target = [u8]>
    where
        Self: 'd;

    /// Returns the data buffer of this account that can be derefed into a byte-slice
    fn data(&self) -> Self::DataDeref<'_>;
}

/// A readonly account that you can read the owner program of
pub trait ReadonlyAccountOwnerBytes {
    /// Returns the pubkey bytes of the program owning this account
    fn owner_bytes(&self) -> [u8; 32];
}

/// A readonly account that you can read whether it's executable or not
pub trait ReadonlyAccountIsExecutable {
    /// Returns true if this is an executable account, false otherwise
    fn is_executable(&self) -> bool;
}

/// A readonly account that you can read the rent epoch of
pub trait ReadonlyAccountRentEpoch {
    /// Returns the rent epoch of this account
    fn rent_epoch(&self) -> u64;
}

// blanket impls for references
// Can't do blanket on Deref<Target = impl Trait> because
// upstream crates (solana-program) might create a new impl of Deref in the future

impl<T> ReadonlyAccountPubkeyBytes for &T
where
    T: ReadonlyAccountPubkeyBytes + ?Sized,
{
    fn pubkey_bytes(&self) -> [u8; 32] {
        (*self).pubkey_bytes()
    }
}

impl<T> ReadonlyAccountLamports for &T
where
    T: ReadonlyAccountLamports + ?Sized,
{
    fn lamports(&self) -> u64 {
        (*self).lamports()
    }
}

impl<T> ReadonlyAccountData for &T
where
    T: ReadonlyAccountData + ?Sized,
{
    type DataDeref<'d> = T::DataDeref<'d> where Self: 'd;

    fn data(&self) -> Self::DataDeref<'_> {
        (*self).data()
    }
}

impl<T> ReadonlyAccountOwnerBytes for &T
where
    T: ReadonlyAccountOwnerBytes + ?Sized,
{
    fn owner_bytes(&self) -> [u8; 32] {
        (*self).owner_bytes()
    }
}

impl<T> ReadonlyAccountIsExecutable for &T
where
    T: ReadonlyAccountIsExecutable + ?Sized,
{
    fn is_executable(&self) -> bool {
        (*self).is_executable()
    }
}

impl<T> ReadonlyAccountRentEpoch for &T
where
    T: ReadonlyAccountRentEpoch + ?Sized,
{
    fn rent_epoch(&self) -> u64 {
        (*self).rent_epoch()
    }
}

#[cfg(test)]
pub mod test_utils {
    use solana_program::{
        program_error::ProgramError, program_option::COption, program_pack::Pack, pubkey::Pubkey,
    };
    use spl_token_2022::state::{Account, AccountState};

    use super::*;

    /// This fn only uses data, but we just add the other traits to make sure
    /// we've implemented them
    pub fn try_deserialize_token_account<
        A: ReadonlyAccountLamports
            + ReadonlyAccountData
            + ReadonlyAccountOwnerBytes
            + ReadonlyAccountIsExecutable
            + ReadonlyAccountRentEpoch,
    >(
        acc: A,
    ) -> Result<Account, ProgramError> {
        Account::unpack(&acc.data())
    }

    pub fn gen_test_token_acc() -> Account {
        let owner = Pubkey::new_unique();
        Account {
            mint: Pubkey::new_unique(),
            owner,
            amount: 123,
            delegate: COption::None,
            state: AccountState::Initialized,
            is_native: COption::None,
            delegated_amount: 0,
            close_authority: COption::Some(owner),
        }
    }
}
