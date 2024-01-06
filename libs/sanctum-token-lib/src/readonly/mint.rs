use solana_program::{program_error::ProgramError, pubkey::Pubkey};
use solana_readonly_account::ReadonlyAccountData;

use crate::SPL_MINT_ACCOUNT_PACKED_LEN;

use super::{is_coption_discm_valid, unpack_coption_pubkey, unpack_le_u64};

pub const SPL_MINT_MINT_AUTHORITY_OFFSET: usize = 0;
pub const SPL_MINT_SUPPLY_OFFSET: usize = SPL_MINT_MINT_AUTHORITY_OFFSET + 36;
pub const SPL_MINT_DECIMALS_OFFSET: usize = SPL_MINT_SUPPLY_OFFSET + 8;
pub const SPL_MINT_IS_INITIALIZED_OFFSET: usize = SPL_MINT_DECIMALS_OFFSET + 1;
pub const SPL_MINT_FREEZE_AUTHORITY_OFFSET: usize = SPL_MINT_IS_INITIALIZED_OFFSET + 1;

pub const SPL_MINT_IS_INITIALIZED_FALSE: u8 = 0;
pub const SPL_MINT_IS_INITIALIZED_TRUE: u8 = 1;

pub fn is_is_initialized_valid(byte: u8) -> bool {
    matches!(
        byte,
        SPL_MINT_IS_INITIALIZED_FALSE | SPL_MINT_IS_INITIALIZED_TRUE
    )
}

/// A possible mint account
///
/// ## Example
///
/// ```rust
/// use sanctum_token_lib::ReadonlyMintAccount;
/// use solana_program::{
///     account_info::AccountInfo,
///     entrypoint::ProgramResult
/// };
///
/// pub fn process(account: &AccountInfo) -> ProgramResult {
///     let account = ReadonlyMintAccount(account);
///     let account = account.try_into_valid()?;
///     let account = account.try_into_initialized()?;
///     solana_program::msg!("{}", account.mint_supply());
///     Ok(())
/// }
/// ```
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ReadonlyMintAccount<T>(pub T);

impl<T> ReadonlyMintAccount<T> {
    pub fn as_inner(&self) -> &T {
        &self.0
    }

    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T: ReadonlyAccountData> ReadonlyMintAccount<T> {
    pub fn mint_data_is_valid(&self) -> bool {
        let d = self.0.data();
        d.len() >= SPL_MINT_ACCOUNT_PACKED_LEN
            && is_coption_discm_valid(
                &d[SPL_MINT_MINT_AUTHORITY_OFFSET..SPL_MINT_MINT_AUTHORITY_OFFSET + 4]
                    .try_into()
                    .unwrap(),
            )
            && is_is_initialized_valid(d[SPL_MINT_IS_INITIALIZED_OFFSET])
            && is_coption_discm_valid(
                &d[SPL_MINT_FREEZE_AUTHORITY_OFFSET..SPL_MINT_FREEZE_AUTHORITY_OFFSET + 4]
                    .try_into()
                    .unwrap(),
            )
    }

    pub fn try_into_valid(self) -> Result<ValidMintAccount<T>, ProgramError> {
        match self.mint_data_is_valid() {
            true => Ok(ValidMintAccount(self)),
            false => Err(ProgramError::InvalidAccountData),
        }
    }
}

impl<T> AsRef<T> for ReadonlyMintAccount<T> {
    fn as_ref(&self) -> &T {
        self.as_inner()
    }
}

// can't impl From<ReadonlyMintAccount<T>> for T due to orphan rules

#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ValidMintAccount<T>(ReadonlyMintAccount<T>);

impl<T> ValidMintAccount<T> {
    pub fn as_readonly(&self) -> &ReadonlyMintAccount<T> {
        &self.0
    }

    pub fn into_readonly(self) -> ReadonlyMintAccount<T> {
        self.0
    }
}

impl<T: ReadonlyAccountData> ValidMintAccount<T> {
    pub fn mint_is_initialized(&self) -> bool {
        let d = self.0.as_inner().data();
        let b = d[SPL_MINT_IS_INITIALIZED_OFFSET];
        match b {
            SPL_MINT_IS_INITIALIZED_FALSE => false,
            SPL_MINT_IS_INITIALIZED_TRUE => true,
            _ => unreachable!(),
        }
    }

    pub fn try_into_initialized(self) -> Result<InitializedMintAccount<T>, ProgramError> {
        match self.mint_is_initialized() {
            true => Ok(InitializedMintAccount(self)),
            false => Err(ProgramError::InvalidAccountData),
        }
    }
}

impl<T> AsRef<ReadonlyMintAccount<T>> for ValidMintAccount<T> {
    fn as_ref(&self) -> &ReadonlyMintAccount<T> {
        self.as_readonly()
    }
}

impl<T> From<ValidMintAccount<T>> for ReadonlyMintAccount<T> {
    fn from(value: ValidMintAccount<T>) -> Self {
        value.into_readonly()
    }
}

impl<T: ReadonlyAccountData> TryFrom<ReadonlyMintAccount<T>> for ValidMintAccount<T> {
    type Error = ProgramError;

    fn try_from(value: ReadonlyMintAccount<T>) -> Result<Self, Self::Error> {
        value.try_into_valid()
    }
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InitializedMintAccount<T>(ValidMintAccount<T>);

impl<T> InitializedMintAccount<T> {
    pub fn as_valid(&self) -> &ValidMintAccount<T> {
        &self.0
    }

    pub fn into_valid(self) -> ValidMintAccount<T> {
        self.0
    }
}

impl<T: ReadonlyAccountData> InitializedMintAccount<T> {
    pub fn mint_mint_authority(&self) -> Option<Pubkey> {
        let d = self.0.as_readonly().as_inner().data();
        unpack_coption_pubkey(&d, SPL_MINT_MINT_AUTHORITY_OFFSET)
    }

    pub fn mint_supply(&self) -> u64 {
        let d = self.0.as_readonly().as_inner().data();
        unpack_le_u64(&d, SPL_MINT_SUPPLY_OFFSET)
    }

    pub fn mint_decimals(&self) -> u8 {
        let d = self.0.as_readonly().as_inner().data();
        d[SPL_MINT_DECIMALS_OFFSET]
    }

    pub fn mint_freeze_authority(&self) -> Option<Pubkey> {
        let d = self.0.as_readonly().as_inner().data();
        unpack_coption_pubkey(&d, SPL_MINT_FREEZE_AUTHORITY_OFFSET)
    }
}

impl<T> AsRef<ValidMintAccount<T>> for InitializedMintAccount<T> {
    fn as_ref(&self) -> &ValidMintAccount<T> {
        self.as_valid()
    }
}

impl<T> From<InitializedMintAccount<T>> for ValidMintAccount<T> {
    fn from(value: InitializedMintAccount<T>) -> Self {
        value.into_valid()
    }
}

impl<T: ReadonlyAccountData> TryFrom<ValidMintAccount<T>> for InitializedMintAccount<T> {
    type Error = ProgramError;

    fn try_from(value: ValidMintAccount<T>) -> Result<Self, Self::Error> {
        value.try_into_initialized()
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;
    use sanctum_solana_test_utils::token::proptest_utils::token_2022::token22_mint_no_extensions;
    use solana_program::program_pack::Pack;
    use spl_token_2022::{extension::StateWithExtensions, state::Mint};

    use crate::readonly::test_utils::AccountData;

    use super::*;

    proptest! {
        #[test]
        fn mint_readonly_matches_full_deser_invalid(bytes: [u8; SPL_MINT_ACCOUNT_PACKED_LEN]) {
            let account = ReadonlyMintAccount(AccountData(&bytes));
            let unpack_res = StateWithExtensions::<Mint>::unpack(&bytes);
            if !account.mint_data_is_valid() {
                prop_assert!(unpack_res.is_err());
            }
        }
    }

    proptest! {
        #[test]
        fn mint_readonly_matches_full_deser_valid(expected in token22_mint_no_extensions()) {
            let mut data = vec![0u8; SPL_MINT_ACCOUNT_PACKED_LEN];
            expected.pack_into_slice(&mut data);
            let account = ReadonlyMintAccount(AccountData(&data)).try_into_valid().unwrap();
            prop_assert_eq!(account.mint_is_initialized(), expected.is_initialized);
            if let Ok(account) = account.try_into_initialized() {
                prop_assert_eq!(account.mint_mint_authority(), expected.mint_authority.into());
                prop_assert_eq!(account.mint_supply(), expected.supply);
                prop_assert_eq!(account.mint_decimals(), expected.decimals);
                prop_assert_eq!(account.mint_freeze_authority(), expected.freeze_authority.into());
            }
        }
    }
}
