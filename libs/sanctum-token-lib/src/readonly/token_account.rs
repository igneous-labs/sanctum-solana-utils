use solana_program::pubkey::{Pubkey, PUBKEY_BYTES};
use solana_readonly_account::ReadonlyAccountData;
use spl_token_interface::AccountState;

use crate::{is_coption_discm_valid, unpack_coption_slice, SPL_TOKEN_ACCOUNT_PACKED_LEN};

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

/// Getter methods that only deserialize the required account
/// data subslice instead of the entire account data vec.
///
/// Works for both token and token-2022 accounts.
///
/// All getter methods are unchecked and will panic if data is malfored,
/// be sure to call
/// [`ReadonlyTokenAccount::token_account_data_is_valid`]
/// before calling the other methods.
///
/// If you're using this onchain, you probably want to call
/// [`ReadonlyTokenAccount::token_account_is_initialized`]
/// and also verify the account's program owner afterwards.
pub trait ReadonlyTokenAccount {
    fn token_account_data_is_valid(&self) -> bool;

    fn token_account_mint(&self) -> Pubkey;

    fn token_account_authority(&self) -> Pubkey;

    fn token_account_amount(&self) -> u64;

    fn token_account_delegate(&self) -> Option<Pubkey>;

    fn token_account_state(&self) -> AccountState;

    fn token_account_is_native(&self) -> Option<u64>;

    fn token_account_delegated_amount(&self) -> u64;

    fn token_account_close_authority(&self) -> Option<Pubkey>;

    fn token_account_is_initialized(&self) -> bool {
        self.token_account_state() != AccountState::Uninitialized
    }
}

impl<D: ReadonlyAccountData> ReadonlyTokenAccount for D {
    fn token_account_data_is_valid(&self) -> bool {
        let d = self.data();
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

    fn token_account_mint(&self) -> Pubkey {
        let d = self.data();
        Pubkey::try_from(
            &d[SPL_TOKEN_ACCOUNT_MINT_OFFSET..SPL_TOKEN_ACCOUNT_MINT_OFFSET + PUBKEY_BYTES],
        )
        .unwrap()
    }

    fn token_account_authority(&self) -> Pubkey {
        let d = self.data();
        Pubkey::try_from(
            &d[SPL_TOKEN_ACCOUNT_AUTHORITY_OFFSET
                ..SPL_TOKEN_ACCOUNT_AUTHORITY_OFFSET + PUBKEY_BYTES],
        )
        .unwrap()
    }

    fn token_account_amount(&self) -> u64 {
        let d = self.data();
        let b: &[u8; 8] = &d[SPL_TOKEN_ACCOUNT_AMOUNT_OFFSET..SPL_TOKEN_ACCOUNT_AMOUNT_OFFSET + 8]
            .try_into()
            .unwrap();
        u64::from_le_bytes(*b)
    }

    fn token_account_delegate(&self) -> Option<Pubkey> {
        let d = self.data();
        unpack_coption_slice(
            &d[SPL_TOKEN_ACCOUNT_DELEGATE_OFFSET
                ..SPL_TOKEN_ACCOUNT_DELEGATE_OFFSET + 4 + PUBKEY_BYTES],
        )
        .map(|b| Pubkey::try_from(b).unwrap())
    }

    fn token_account_state(&self) -> AccountState {
        let d = self.data();
        let b = d[SPL_TOKEN_ACCOUNT_STATE_OFFSET];
        match b {
            SPL_TOKEN_ACCOUNT_STATE_UNINITIALIZED_DISCM => AccountState::Uninitialized,
            SPL_TOKEN_ACCOUNT_STATE_INITIALIZED_DISCM => AccountState::Initialized,
            SPL_TOKEN_ACCOUNT_STATE_FROZEN_DISCM => AccountState::Frozen,
            _ => panic!("invalid AccountState {b:?}"),
        }
    }

    fn token_account_is_native(&self) -> Option<u64> {
        let d = self.data();
        unpack_coption_slice(
            &d[SPL_TOKEN_ACCOUNT_IS_NATIVE_OFFSET..SPL_TOKEN_ACCOUNT_IS_NATIVE_OFFSET + 4 + 8],
        )
        .map(|b| {
            let le: &[u8; 8] = b.try_into().unwrap();
            u64::from_le_bytes(*le)
        })
    }

    fn token_account_delegated_amount(&self) -> u64 {
        let d = self.data();
        let b: &[u8; 8] = &d[SPL_TOKEN_ACCOUNT_DELEGATED_AMOUNT_OFFSET
            ..SPL_TOKEN_ACCOUNT_DELEGATED_AMOUNT_OFFSET + 8]
            .try_into()
            .unwrap();
        u64::from_le_bytes(*b)
    }

    fn token_account_close_authority(&self) -> Option<Pubkey> {
        let d = self.data();
        unpack_coption_slice(
            &d[SPL_TOKEN_ACCOUNT_CLOSE_AUTHORITY_OFFSET
                ..SPL_TOKEN_ACCOUNT_CLOSE_AUTHORITY_OFFSET + 4 + PUBKEY_BYTES],
        )
        .map(|b| Pubkey::try_from(b).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;
    use solana_program::program_pack::IsInitialized;
    use spl_token_2022::extension::StateWithExtensions;

    use crate::readonly::test_utils::{to_account, valid_coption_discm};

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
            let account = to_account(&bytes);
            let unpack_res = StateWithExtensions::<spl_token_2022::state::Account>::unpack(&bytes);
            if !account.token_account_data_is_valid() {
                prop_assert!(unpack_res.is_err());
            }
        }
    }

    proptest! {
        #[test]
        fn token_account_readonly_matches_full_deser_valid(
            mut bytes: [u8; SPL_TOKEN_ACCOUNT_PACKED_LEN],
            delegate_discm in valid_coption_discm(),
            is_native_discm in valid_coption_discm(),
            close_authority_discm in valid_coption_discm(),
            account_state in SPL_TOKEN_ACCOUNT_STATE_INITIALIZED_DISCM..=SPL_TOKEN_ACCOUNT_STATE_FROZEN_DISCM,
        ) {
            bytes.get_mut(SPL_TOKEN_ACCOUNT_DELEGATE_OFFSET..SPL_TOKEN_ACCOUNT_DELEGATE_OFFSET + 4)
                .unwrap()
                .copy_from_slice(&delegate_discm);
            bytes.get_mut(SPL_TOKEN_ACCOUNT_IS_NATIVE_OFFSET..SPL_TOKEN_ACCOUNT_IS_NATIVE_OFFSET + 4)
                .unwrap()
                .copy_from_slice(&is_native_discm);
            bytes.get_mut(SPL_TOKEN_ACCOUNT_CLOSE_AUTHORITY_OFFSET..SPL_TOKEN_ACCOUNT_CLOSE_AUTHORITY_OFFSET + 4)
                .unwrap()
                .copy_from_slice(&close_authority_discm);
            bytes[SPL_TOKEN_ACCOUNT_STATE_OFFSET] = account_state;

            let StateWithExtensions { base: expected, .. }
                = StateWithExtensions::<spl_token_2022::state::Account>::unpack(&bytes).unwrap();
            let account = to_account(&bytes);
            prop_assert_eq!(account.token_account_mint(), expected.mint);
            prop_assert_eq!(account.token_account_authority(), expected.owner);
            prop_assert_eq!(account.token_account_amount(), expected.amount);
            prop_assert_eq!(account.token_account_delegate(), expected.delegate.into());
            prop_assert_eq!(conv_account_state(account.token_account_state()), expected.state);
            prop_assert_eq!(account.token_account_is_native(), expected.is_native.into());
            prop_assert_eq!(account.token_account_delegated_amount(), expected.delegated_amount);
            prop_assert_eq!(account.token_account_close_authority(), expected.close_authority.into());
            prop_assert_eq!(account.token_account_is_initialized(), expected.is_initialized());
        }
    }
}
