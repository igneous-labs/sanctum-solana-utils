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

fn nonce_checked_method<R: ReadonlyNonceAccount + ?Sized, T>(
    r: &R,
    unchecked_method: fn(&R) -> T,
) -> Result<T, ProgramError> {
    match r.nonce_state_marker() {
        NonceStateMarker::Initialized => Ok(unchecked_method(r)),
        NonceStateMarker::Uninitialized => Err(ProgramError::InvalidAccountData),
    }
}

/// Getter methods that only deserialize the required account
/// data subslice instead of the entire account data vec.
///
/// All getter methods are unchecked and will panic if data is malfored,
/// be sure to call
/// [`ReadonlyNonceAccount::nonce_data_is_valid`]
/// before calling the other methods
///
/// The `*_unchecked()` methods do not check that the nonce account is of the correct State enum
/// before reading the bytes
pub trait ReadonlyNonceAccount {
    fn nonce_data_is_valid(&self) -> bool;

    fn nonce_state_marker(&self) -> NonceStateMarker;

    fn nonce_data_authority_unchecked(&self) -> Pubkey;

    fn nonce_data_authority(&self) -> Result<Pubkey, ProgramError> {
        nonce_checked_method(self, Self::nonce_data_authority_unchecked)
    }

    /// This returns the private DurableNonce field, which is what you would get with durableNonce.as_hash()
    fn nonce_data_durable_nonce_unchecked(&self) -> Hash;

    fn nonce_data_durable_nonce(&self) -> Result<Hash, ProgramError> {
        nonce_checked_method(self, Self::nonce_data_durable_nonce_unchecked)
    }

    fn nonce_data_fee_calculator_unchecked(&self) -> FeeCalculator {
        FeeCalculator {
            lamports_per_signature: self
                .nonce_data_fee_calculator_lamports_per_signature_unchecked(),
        }
    }

    fn nonce_data_fee_calculator(&self) -> Result<FeeCalculator, ProgramError> {
        nonce_checked_method(self, Self::nonce_data_fee_calculator_unchecked)
    }

    fn nonce_data_fee_calculator_lamports_per_signature_unchecked(&self) -> u64;

    fn nonce_data_fee_calculator_lamports_per_signature(&self) -> Result<u64, ProgramError> {
        nonce_checked_method(
            self,
            Self::nonce_data_fee_calculator_lamports_per_signature_unchecked,
        )
    }
}

impl<R: ReadonlyAccountData> ReadonlyNonceAccount for R {
    fn nonce_data_is_valid(&self) -> bool {
        let d = self.data();
        if d.len() != nonce::State::size() {
            return false;
        }
        let b: &[u8; 4] = d[NONCE_DISCM_OFFSET..NONCE_DISCM_OFFSET + 4]
            .try_into()
            .unwrap();
        NonceStateMarker::try_from(*b).is_ok()
    }

    fn nonce_state_marker(&self) -> NonceStateMarker {
        let d = self.data();
        let b: &[u8; 4] = d[NONCE_DISCM_OFFSET..NONCE_DISCM_OFFSET + 4]
            .try_into()
            .unwrap();
        NonceStateMarker::try_from(*b).unwrap()
    }

    fn nonce_data_authority_unchecked(&self) -> Pubkey {
        let d = self.data();
        let b: &[u8; 32] = d
            [NONCE_DATA_AUTHORITY_OFFSET..NONCE_DATA_AUTHORITY_OFFSET + PUBKEY_BYTES]
            .try_into()
            .unwrap();
        Pubkey::from(*b)
    }

    fn nonce_data_durable_nonce_unchecked(&self) -> Hash {
        let d = self.data();
        let b: &[u8; 32] = d
            [NONCE_DATA_DURABLE_NONCE_OFFSET..NONCE_DATA_DURABLE_NONCE_OFFSET + HASH_BYTES]
            .try_into()
            .unwrap();
        Hash::from(*b)
    }

    fn nonce_data_fee_calculator_lamports_per_signature_unchecked(&self) -> u64 {
        let d = self.data();
        let b: &[u8; 8] = d[NONCE_DATA_FEE_CALCULATOR_LAMPORTS_PER_SIGNATURE_OFFSET
            ..NONCE_DATA_FEE_CALCULATOR_LAMPORTS_PER_SIGNATURE_OFFSET + 8]
            .try_into()
            .unwrap();
        u64::from_le_bytes(*b)
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
        type SliceDeref<'s> = &'s [u8]
        where
            Self: 's;

        type DataDeref<'d> = &'d &'d [u8]
        where
            Self: 'd;

        fn data(&self) -> Self::DataDeref<'_> {
            &self.0
        }
    }

    proptest! {
        #[test]
        fn nonce_readonly_matches_full_deser_invalid(data: [u8; nonce::State::size()]) {
            let account = AccountData(&data);
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
            let account = AccountData(&data);
            prop_assert!(account.nonce_data_is_valid());
            match nonce {
                nonce::State::Uninitialized => prop_assert_eq!(account.nonce_state_marker(), NonceStateMarker::Uninitialized),
                nonce::State::Initialized(s) => {
                    prop_assert_eq!(account.nonce_data_authority_unchecked(), account.nonce_data_authority().unwrap());
                    prop_assert_eq!(account.nonce_data_authority_unchecked(), s.authority);

                    prop_assert_eq!(account.nonce_data_durable_nonce_unchecked(), account.nonce_data_durable_nonce().unwrap());
                    prop_assert_eq!(account.nonce_data_durable_nonce_unchecked(), *s.durable_nonce.as_hash());

                    prop_assert_eq!(account.nonce_data_fee_calculator_unchecked(), account.nonce_data_fee_calculator().unwrap());
                    prop_assert_eq!(account.nonce_data_fee_calculator_unchecked(), s.fee_calculator);

                    prop_assert_eq!(
                        account.nonce_data_fee_calculator_lamports_per_signature_unchecked(),
                        account.nonce_data_fee_calculator_lamports_per_signature().unwrap()
                    );
                    prop_assert_eq!(
                        account.nonce_data_fee_calculator_lamports_per_signature_unchecked(),
                        s.fee_calculator.lamports_per_signature
                    );
                }
            }
        }
    }
}
