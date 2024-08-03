use solana_program::{
    fee_calculator::FeeCalculator,
    hash::{Hash, HASH_BYTES},
    nonce,
    program_error::ProgramError,
    pubkey::{Pubkey, PUBKEY_BYTES},
};
use solana_readonly_account::ReadonlyAccountData;

pub const NONCE_STATE_UNINITIALIZED_DISCM: [u8; 4] = 0u32.to_le_bytes();
pub const NONCE_STATE_INITIALIZED_DISCM: [u8; 4] = 1u32.to_le_bytes();

pub const NONCE_DISCM_OFFSET: usize = 0;
pub const NONCE_DATA_OFFSET: usize = NONCE_DISCM_OFFSET + 4;
pub const NONCE_DATA_AUTHORITY_OFFSET: usize = NONCE_DATA_OFFSET;
pub const NONCE_DATA_DURABLE_NONCE_OFFSET: usize = NONCE_DATA_AUTHORITY_OFFSET + PUBKEY_BYTES;
pub const NONCE_DATA_FEE_CALCULATOR_OFFSET: usize = NONCE_DATA_DURABLE_NONCE_OFFSET + HASH_BYTES;
pub const NONCE_DATA_FEE_CALCULATOR_LAMPORTS_PER_SIGNATURE_OFFSET: usize =
    NONCE_DATA_FEE_CALCULATOR_OFFSET;

/// A possible nonce account
///
/// ## Example
///
/// ```rust
/// use sanctum_system_program_lib::ReadonlyNonceAccount;
/// use solana_program::{
///     account_info::AccountInfo,
///     entrypoint::ProgramResult
/// };
///
/// pub fn process(account: &AccountInfo) -> ProgramResult {
///     let account = ReadonlyNonceAccount(account);
///     let account = account.try_into_valid()?;
///     let account = account.try_into_initialized()?;
///     solana_program::msg!("{}", account.nonce_data_authority());
///     Ok(())
/// }
/// ```
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ReadonlyNonceAccount<T>(pub T);

impl<T> ReadonlyNonceAccount<T> {
    pub fn as_inner(&self) -> &T {
        &self.0
    }

    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T: ReadonlyAccountData> ReadonlyNonceAccount<T> {
    pub fn nonce_data_is_valid(&self) -> bool {
        let d = self.0.data();
        if d.len() != nonce::State::size() {
            return false;
        }
        let b: &[u8; 4] = d[NONCE_DISCM_OFFSET..NONCE_DISCM_OFFSET + 4]
            .try_into()
            .unwrap();
        NonceStateMarker::try_from(*b).is_ok()
    }

    pub fn try_into_valid(self) -> Result<ValidNonceAccount<T>, ProgramError> {
        if self.nonce_data_is_valid() {
            Ok(ValidNonceAccount(self))
        } else {
            Err(ProgramError::InvalidAccountData)
        }
    }
}

impl<T> AsRef<T> for ReadonlyNonceAccount<T> {
    fn as_ref(&self) -> &T {
        self.as_inner()
    }
}

// can't impl From<ReadonlyNonceAccount<T>> for T due to orphan rules

#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ValidNonceAccount<T>(ReadonlyNonceAccount<T>);

impl<T> ValidNonceAccount<T> {
    pub fn as_readonly(&self) -> &ReadonlyNonceAccount<T> {
        &self.0
    }

    pub fn into_readonly(self) -> ReadonlyNonceAccount<T> {
        self.0
    }
}

impl<T: ReadonlyAccountData> ValidNonceAccount<T> {
    pub fn nonce_state_marker(&self) -> NonceStateMarker {
        let d = self.0.as_inner().data();
        let b: &[u8; 4] = d[NONCE_DISCM_OFFSET..NONCE_DISCM_OFFSET + 4]
            .try_into()
            .unwrap();
        NonceStateMarker::try_from(*b).unwrap()
    }

    pub fn try_into_initialized(self) -> Result<InitializedNonceAccount<T>, ProgramError> {
        match self.nonce_state_marker() {
            NonceStateMarker::Initialized => Ok(InitializedNonceAccount(self)),
            NonceStateMarker::Uninitialized => Err(ProgramError::InvalidAccountData),
        }
    }
}

impl<T> AsRef<ReadonlyNonceAccount<T>> for ValidNonceAccount<T> {
    fn as_ref(&self) -> &ReadonlyNonceAccount<T> {
        &self.0
    }
}

impl<T> From<ValidNonceAccount<T>> for ReadonlyNonceAccount<T> {
    fn from(value: ValidNonceAccount<T>) -> Self {
        value.0
    }
}

impl<T: ReadonlyAccountData> TryFrom<ReadonlyNonceAccount<T>> for ValidNonceAccount<T> {
    type Error = ProgramError;

    fn try_from(value: ReadonlyNonceAccount<T>) -> Result<Self, Self::Error> {
        value.try_into_valid()
    }
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InitializedNonceAccount<T>(ValidNonceAccount<T>);

impl<T> InitializedNonceAccount<T> {
    pub fn as_valid(&self) -> &ValidNonceAccount<T> {
        &self.0
    }

    pub fn into_valid(self) -> ValidNonceAccount<T> {
        self.0
    }
}

impl<T: ReadonlyAccountData> InitializedNonceAccount<T> {
    pub fn nonce_data_authority(&self) -> Pubkey {
        let d = self.0.as_readonly().as_inner().data();
        let b: &[u8; PUBKEY_BYTES] = d
            [NONCE_DATA_AUTHORITY_OFFSET..NONCE_DATA_AUTHORITY_OFFSET + PUBKEY_BYTES]
            .try_into()
            .unwrap();
        Pubkey::from(*b)
    }

    pub fn nonce_data_durable_nonce(&self) -> Hash {
        let d = self.0.as_readonly().as_inner().data();
        let b: &[u8; HASH_BYTES] = d
            [NONCE_DATA_DURABLE_NONCE_OFFSET..NONCE_DATA_DURABLE_NONCE_OFFSET + HASH_BYTES]
            .try_into()
            .unwrap();
        Hash::from(*b)
    }

    pub fn nonce_data_fee_calculator(&self) -> FeeCalculator {
        FeeCalculator {
            lamports_per_signature: self.nonce_data_fee_calculator_lamports_per_signature(),
        }
    }

    pub fn nonce_data_fee_calculator_lamports_per_signature(&self) -> u64 {
        let d = self.0.as_readonly().as_inner().data();
        let b: &[u8; 8] = d[NONCE_DATA_FEE_CALCULATOR_LAMPORTS_PER_SIGNATURE_OFFSET
            ..NONCE_DATA_FEE_CALCULATOR_LAMPORTS_PER_SIGNATURE_OFFSET + 8]
            .try_into()
            .unwrap();
        u64::from_le_bytes(*b)
    }
}

impl<T> AsRef<ValidNonceAccount<T>> for InitializedNonceAccount<T> {
    fn as_ref(&self) -> &ValidNonceAccount<T> {
        &self.0
    }
}

impl<T> From<InitializedNonceAccount<T>> for ValidNonceAccount<T> {
    fn from(value: InitializedNonceAccount<T>) -> Self {
        value.0
    }
}

impl<T: ReadonlyAccountData> TryFrom<ValidNonceAccount<T>> for InitializedNonceAccount<T> {
    type Error = ProgramError;

    fn try_from(value: ValidNonceAccount<T>) -> Result<Self, Self::Error> {
        value.try_into_initialized()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum NonceStateMarker {
    Uninitialized,
    Initialized,
}

impl TryFrom<[u8; 4]> for NonceStateMarker {
    type Error = ProgramError;

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        match value {
            NONCE_STATE_UNINITIALIZED_DISCM => Ok(Self::Uninitialized),
            NONCE_STATE_INITIALIZED_DISCM => Ok(Self::Initialized),
            _ => Err(ProgramError::InvalidAccountData),
        }
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;
    use sanctum_solana_test_utils::proptest_utils::nonce;
    use solana_program::nonce;
    use solana_readonly_account::ReadonlyAccountData;

    use super::*;

    struct AccountData<'a>(pub &'a [u8]);

    impl<'a> ReadonlyAccountData for AccountData<'a> {
        type DataDeref<'d> = &'d [u8]
        where
            Self: 'd;

        fn data(&self) -> Self::DataDeref<'_> {
            self.0
        }
    }

    proptest! {
        #[test]
        fn nonce_readonly_matches_full_deser_invalid(data: [u8; nonce::State::size()]) {
            let account = ReadonlyNonceAccount(AccountData(&data));
            let unpack_res = bincode::deserialize::<nonce::State>(&data);
            if !account.nonce_data_is_valid() {
                prop_assert!(unpack_res.is_err());
            }
        }
    }

    proptest! {
        #[test]
        fn nonce_readonly_matches_full_deser_valid(nonce in nonce()) {
            let mut data = vec![0u8; nonce::State::size()];
            bincode::serialize_into(data.as_mut_slice(), &nonce).unwrap();
            let account = ReadonlyNonceAccount(AccountData(&data));
            prop_assert!(account.nonce_data_is_valid());
            let account = account.try_into_valid().unwrap();
            match nonce {
                nonce::State::Uninitialized => prop_assert_eq!(account.nonce_state_marker(), NonceStateMarker::Uninitialized),
                nonce::State::Initialized(s) => {
                    let account = account.try_into_initialized().unwrap();
                    prop_assert_eq!(account.nonce_data_authority(), s.authority);
                    prop_assert_eq!(account.nonce_data_durable_nonce(), *s.durable_nonce.as_hash());
                    prop_assert_eq!(account.nonce_data_fee_calculator(), s.fee_calculator);
                    prop_assert_eq!(
                        account.nonce_data_fee_calculator_lamports_per_signature(),
                        s.fee_calculator.lamports_per_signature
                    );
                }
            }
        }
    }
}
