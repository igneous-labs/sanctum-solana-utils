use solana_program::{program_error::ProgramError, pubkey::Pubkey};
use solana_readonly_account::ReadonlyAccountData;
use spl_token_interface::AccountState;

use crate::SPL_TOKEN_ACCOUNT_PACKED_LEN;

use super::{
    is_coption_discm_valid, unpack_coption_pubkey, unpack_coption_slice, unpack_le_u64,
    unpack_pubkey,
};

pub const SPL_TOKEN_ACCOUNT_MINT_OFFSET: usize = 0;
pub const SPL_TOKEN_ACCOUNT_AUTHORITY_OFFSET: usize = SPL_TOKEN_ACCOUNT_MINT_OFFSET + 32;
pub const SPL_TOKEN_ACCOUNT_AMOUNT_OFFSET: usize = SPL_TOKEN_ACCOUNT_AUTHORITY_OFFSET + 32;
pub const SPL_TOKEN_ACCOUNT_DELEGATE_OFFSET: usize = SPL_TOKEN_ACCOUNT_AMOUNT_OFFSET + 8;
pub const SPL_TOKEN_ACCOUNT_STATE_OFFSET: usize = SPL_TOKEN_ACCOUNT_DELEGATE_OFFSET + 36;
pub const SPL_TOKEN_ACCOUNT_IS_NATIVE_OFFSET: usize = SPL_TOKEN_ACCOUNT_STATE_OFFSET + 1;
pub const SPL_TOKEN_ACCOUNT_DELEGATED_AMOUNT_OFFSET: usize =
    SPL_TOKEN_ACCOUNT_IS_NATIVE_OFFSET + 12;
pub const SPL_TOKEN_ACCOUNT_CLOSE_AUTHORITY_OFFSET: usize =
    SPL_TOKEN_ACCOUNT_DELEGATED_AMOUNT_OFFSET + 8;

pub const SPL_TOKEN_ACCOUNT_STATE_UNINITIALIZED_DISCM: u8 = 0;
pub const SPL_TOKEN_ACCOUNT_STATE_INITIALIZED_DISCM: u8 = 1;
pub const SPL_TOKEN_ACCOUNT_STATE_FROZEN_DISCM: u8 = 2;

pub fn is_account_state_valid(byte: u8) -> bool {
    matches!(
        byte,
        SPL_TOKEN_ACCOUNT_STATE_UNINITIALIZED_DISCM
            | SPL_TOKEN_ACCOUNT_STATE_INITIALIZED_DISCM
            | SPL_TOKEN_ACCOUNT_STATE_FROZEN_DISCM
    )
}

/// A possible token account
///
/// ## Example
///
/// ```rust
/// use sanctum_token_lib::ReadonlyTokenAccount;
/// use solana_program::{
///     account_info::AccountInfo,
///     entrypoint::ProgramResult
/// };
///
/// pub fn process(account: &AccountInfo) -> ProgramResult {
///     let account = ReadonlyTokenAccount(account);
///     let account = account.try_into_valid()?;
///     let account = account.try_into_initialized()?;
///     solana_program::msg!("{}", account.token_account_amount());
///     Ok(())
/// }
/// ```
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ReadonlyTokenAccount<T>(pub T);

impl<T> ReadonlyTokenAccount<T> {
    pub fn as_inner(&self) -> &T {
        &self.0
    }

    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T: ReadonlyAccountData> ReadonlyTokenAccount<T> {
    pub fn token_account_data_is_valid(&self) -> bool {
        let d = self.0.data();
        d.len() >= SPL_TOKEN_ACCOUNT_PACKED_LEN
            && is_coption_discm_valid(
                &d[SPL_TOKEN_ACCOUNT_DELEGATE_OFFSET..SPL_TOKEN_ACCOUNT_DELEGATE_OFFSET + 4]
                    .try_into()
                    .unwrap(),
            )
            && is_account_state_valid(d[SPL_TOKEN_ACCOUNT_STATE_OFFSET])
            && is_coption_discm_valid(
                &d[SPL_TOKEN_ACCOUNT_IS_NATIVE_OFFSET..SPL_TOKEN_ACCOUNT_IS_NATIVE_OFFSET + 4]
                    .try_into()
                    .unwrap(),
            )
            && is_coption_discm_valid(
                &d[SPL_TOKEN_ACCOUNT_CLOSE_AUTHORITY_OFFSET
                    ..SPL_TOKEN_ACCOUNT_CLOSE_AUTHORITY_OFFSET + 4]
                    .try_into()
                    .unwrap(),
            )
    }

    pub fn try_into_valid(self) -> Result<ValidTokenAccount<T>, ProgramError> {
        match self.token_account_data_is_valid() {
            true => Ok(ValidTokenAccount(self)),
            false => Err(ProgramError::InvalidAccountData),
        }
    }
}

impl<T> AsRef<T> for ReadonlyTokenAccount<T> {
    fn as_ref(&self) -> &T {
        self.as_inner()
    }
}

// can't impl From<ReadonlyTokenAccount<T>> for T due to orphan rules

#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ValidTokenAccount<T>(ReadonlyTokenAccount<T>);

impl<T> ValidTokenAccount<T> {
    pub fn as_readonly(&self) -> &ReadonlyTokenAccount<T> {
        &self.0
    }

    pub fn into_readonly(self) -> ReadonlyTokenAccount<T> {
        self.0
    }
}

impl<T: ReadonlyAccountData> ValidTokenAccount<T> {
    pub fn token_account_state(&self) -> AccountState {
        let d = self.0.as_inner().data();
        let b = d[SPL_TOKEN_ACCOUNT_STATE_OFFSET];
        match b {
            SPL_TOKEN_ACCOUNT_STATE_UNINITIALIZED_DISCM => AccountState::Uninitialized,
            SPL_TOKEN_ACCOUNT_STATE_INITIALIZED_DISCM => AccountState::Initialized,
            SPL_TOKEN_ACCOUNT_STATE_FROZEN_DISCM => AccountState::Frozen,
            _ => unreachable!(),
        }
    }

    pub fn token_account_is_initialized(&self) -> bool {
        self.token_account_state() != AccountState::Uninitialized
    }

    pub fn try_into_initialized(self) -> Result<InitializedTokenAccount<T>, ProgramError> {
        match self.token_account_is_initialized() {
            true => Ok(InitializedTokenAccount(self)),
            false => Err(ProgramError::InvalidAccountData),
        }
    }
}

impl<T> AsRef<ReadonlyTokenAccount<T>> for ValidTokenAccount<T> {
    fn as_ref(&self) -> &ReadonlyTokenAccount<T> {
        self.as_readonly()
    }
}

impl<T> From<ValidTokenAccount<T>> for ReadonlyTokenAccount<T> {
    fn from(value: ValidTokenAccount<T>) -> Self {
        value.into_readonly()
    }
}

impl<T: ReadonlyAccountData> TryFrom<ReadonlyTokenAccount<T>> for ValidTokenAccount<T> {
    type Error = ProgramError;

    fn try_from(value: ReadonlyTokenAccount<T>) -> Result<Self, Self::Error> {
        value.try_into_valid()
    }
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InitializedTokenAccount<T>(ValidTokenAccount<T>);

impl<T> InitializedTokenAccount<T> {
    pub fn as_valid(&self) -> &ValidTokenAccount<T> {
        &self.0
    }

    pub fn into_valid(self) -> ValidTokenAccount<T> {
        self.0
    }
}

impl<T: ReadonlyAccountData> InitializedTokenAccount<T> {
    pub fn token_account_mint(&self) -> Pubkey {
        let d = self.0.as_readonly().as_inner().data();
        unpack_pubkey(&d, SPL_TOKEN_ACCOUNT_MINT_OFFSET)
    }

    pub fn token_account_authority(&self) -> Pubkey {
        let d = self.0.as_readonly().as_inner().data();
        unpack_pubkey(&d, SPL_TOKEN_ACCOUNT_AUTHORITY_OFFSET)
    }

    pub fn token_account_amount(&self) -> u64 {
        let d = self.0.as_readonly().as_inner().data();
        unpack_le_u64(&d, SPL_TOKEN_ACCOUNT_AMOUNT_OFFSET)
    }

    pub fn token_account_delegate(&self) -> Option<Pubkey> {
        let d = self.0.as_readonly().as_inner().data();
        unpack_coption_pubkey(&d, SPL_TOKEN_ACCOUNT_DELEGATE_OFFSET)
    }

    pub fn token_account_is_frozen(&self) -> bool {
        match self.as_valid().token_account_state() {
            AccountState::Frozen => true,
            AccountState::Initialized => false,
            AccountState::Uninitialized => unreachable!(),
        }
    }

    pub fn token_account_is_native(&self) -> Option<u64> {
        let d = self.0.as_readonly().as_inner().data();
        unpack_coption_slice(
            &d[SPL_TOKEN_ACCOUNT_IS_NATIVE_OFFSET..SPL_TOKEN_ACCOUNT_IS_NATIVE_OFFSET + 4 + 8],
        )
        .map(|s| unpack_le_u64(s, 0))
    }

    pub fn token_account_delegated_amount(&self) -> u64 {
        let d = self.0.as_readonly().as_inner().data();
        unpack_le_u64(&d, SPL_TOKEN_ACCOUNT_DELEGATED_AMOUNT_OFFSET)
    }

    pub fn token_account_close_authority(&self) -> Option<Pubkey> {
        let d = self.0.as_readonly().as_inner().data();
        unpack_coption_pubkey(&d, SPL_TOKEN_ACCOUNT_CLOSE_AUTHORITY_OFFSET)
    }
}

impl<T> AsRef<ValidTokenAccount<T>> for InitializedTokenAccount<T> {
    fn as_ref(&self) -> &ValidTokenAccount<T> {
        self.as_valid()
    }
}

impl<T> From<InitializedTokenAccount<T>> for ValidTokenAccount<T> {
    fn from(value: InitializedTokenAccount<T>) -> Self {
        value.into_valid()
    }
}

impl<T: ReadonlyAccountData> TryFrom<ValidTokenAccount<T>> for InitializedTokenAccount<T> {
    type Error = ProgramError;

    fn try_from(value: ValidTokenAccount<T>) -> Result<Self, Self::Error> {
        value.try_into_initialized()
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;
    use sanctum_solana_test_utils::token::proptest_utils::token_2022::token22_account_no_extensions;
    use solana_program::program_pack::{IsInitialized, Pack};
    use spl_token_2022::extension::StateWithExtensions;

    use crate::readonly::test_utils::AccountData;

    use super::*;

    fn conv_account_state(account_state: AccountState) -> spl_token_2022::state::AccountState {
        match account_state {
            AccountState::Uninitialized => spl_token_2022::state::AccountState::Uninitialized,
            AccountState::Initialized => spl_token_2022::state::AccountState::Initialized,
            AccountState::Frozen => spl_token_2022::state::AccountState::Frozen,
        }
    }

    proptest! {
        #[test]
        fn token_account_readonly_matches_full_deser_invalid(bytes: [u8; SPL_TOKEN_ACCOUNT_PACKED_LEN]) {
            let account = ReadonlyTokenAccount(AccountData(&bytes));
            let unpack_res = StateWithExtensions::<spl_token_2022::state::Account>::unpack(&bytes);
            if !account.token_account_data_is_valid() {
                prop_assert!(unpack_res.is_err());
            }
        }
    }

    proptest! {
        #[test]
        fn token_account_readonly_matches_full_deser_valid(
            expected in token22_account_no_extensions()
        ) {
            let mut data = vec![0u8; SPL_TOKEN_ACCOUNT_PACKED_LEN];
            expected.pack_into_slice(&mut data);
            let account = ReadonlyTokenAccount(AccountData(&data)).try_into_valid().unwrap();
            prop_assert_eq!(conv_account_state(account.token_account_state()), expected.state);
            prop_assert_eq!(account.token_account_is_initialized(), expected.is_initialized());
            if account.token_account_is_initialized() {
                let account = account.try_into_initialized().unwrap();
                prop_assert_eq!(account.token_account_mint(), expected.mint);
                prop_assert_eq!(account.token_account_authority(), expected.owner);
                prop_assert_eq!(account.token_account_amount(), expected.amount);
                prop_assert_eq!(account.token_account_delegate(), expected.delegate.into());
                prop_assert_eq!(account.token_account_is_native(), expected.is_native.into());
                prop_assert_eq!(account.token_account_delegated_amount(), expected.delegated_amount);
                prop_assert_eq!(account.token_account_close_authority(), expected.close_authority.into());
            }
        }
    }
}
