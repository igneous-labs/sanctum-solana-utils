use solana_program::{
    program_pack::Pack,
    pubkey::{Pubkey, PUBKEY_BYTES},
};
use solana_readonly_account::ReadonlyAccountData;
use spl_token_2022::state::AccountState;

use crate::{is_coption_discm_valid, unpack_coption_slice};

pub const SPL_TOKEN_ACCOUNT_MINT_OFFSET: usize = 0;
pub const SPL_TOKEN_ACCOUNT_OWNER_OFFSET: usize = SPL_TOKEN_ACCOUNT_MINT_OFFSET + 32;
pub const SPL_TOKEN_ACCOUNT_AMOUNT_OFFSET: usize = SPL_TOKEN_ACCOUNT_OWNER_OFFSET + 32;
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
/// and verify the account's program owner afterwards.
pub trait ReadonlyTokenAccount {
    fn token_account_data_is_valid(&self) -> bool;

    fn token_account_mint(&self) -> Pubkey;

    fn token_account_owner(&self) -> Pubkey;

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
        d.len() >= spl_token_2022::state::Account::LEN
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

    fn token_account_owner(&self) -> Pubkey {
        let d = self.data();
        Pubkey::try_from(
            &d[SPL_TOKEN_ACCOUNT_OWNER_OFFSET..SPL_TOKEN_ACCOUNT_OWNER_OFFSET + PUBKEY_BYTES],
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
            0 => AccountState::Uninitialized,
            1 => AccountState::Initialized,
            2 => AccountState::Frozen,
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
    use proptest::{prop_assert, prop_assert_eq, proptest};
    use solana_program::program_pack::IsInitialized;
    use solana_sdk::account::Account;
    use spl_token_2022::extension::StateWithExtensions;

    use super::*;

    proptest! {
        #[test]
        fn zero_copy_matches_full_deser(bytes: [u8; spl_token_2022::state::Account::LEN]) {
            let account = Account {
                lamports: 0,
                data: bytes.to_vec(),
                owner: spl_token_2022::ID,
                executable: false,
                rent_epoch: u64::MAX
            };
            let unpack_res = StateWithExtensions::<spl_token_2022::state::Account>::unpack(&bytes);
            if !account.token_account_data_is_valid() {
                prop_assert!(unpack_res.is_err());
            } else {
                let StateWithExtensions { base: expected, .. } = unpack_res.unwrap();
                prop_assert_eq!(account.token_account_mint(), expected.mint);
                prop_assert_eq!(account.token_account_owner(), expected.owner);
                prop_assert_eq!(account.token_account_amount(), expected.amount);
                prop_assert_eq!(account.token_account_delegate(), expected.delegate.into());
                prop_assert_eq!(account.token_account_state(), expected.state);
                prop_assert_eq!(account.token_account_is_native(), expected.is_native.into());
                prop_assert_eq!(account.token_account_delegated_amount(), expected.delegated_amount);
                prop_assert_eq!(account.token_account_close_authority(), expected.close_authority.into());
                prop_assert_eq!(account.token_account_is_initialized(), expected.is_initialized());
            }
        }
    }
}
