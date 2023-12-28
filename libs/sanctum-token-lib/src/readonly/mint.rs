use solana_program::pubkey::{Pubkey, PUBKEY_BYTES};
use solana_readonly_account::ReadonlyAccountData;

use crate::SPL_MINT_ACCOUNT_PACKED_LEN;

use super::{is_coption_discm_valid, unpack_coption_slice};

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

/// Getter methods that only deserialize the required account
/// data subslice instead of the entire account data vec.
///
/// Works for both token and token-2022 mints.
///
/// All getter methods are unchecked and will panic if data is malfored,
/// be sure to call
/// [`ReadonlyMintAccount::mint_data_is_valid`]
/// before calling the other methods.
///
/// If you're using this onchain, you probably want to call
/// [`ReadonlyMintAccount::mint_is_initialized`]
/// and also verify the account's program owner afterwards.
pub trait ReadonlyMintAccount {
    fn mint_data_is_valid(&self) -> bool;

    fn mint_mint_authority(&self) -> Option<Pubkey>;

    fn mint_supply(&self) -> u64;

    fn mint_decimals(&self) -> u8;

    fn mint_is_initialized(&self) -> bool;

    fn mint_freeze_authority(&self) -> Option<Pubkey>;
}

impl<D: ReadonlyAccountData> ReadonlyMintAccount for D {
    fn mint_data_is_valid(&self) -> bool {
        let d = self.data();
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

    fn mint_mint_authority(&self) -> Option<Pubkey> {
        let d = self.data();
        unpack_coption_slice(
            &d[SPL_MINT_MINT_AUTHORITY_OFFSET..SPL_MINT_MINT_AUTHORITY_OFFSET + 4 + PUBKEY_BYTES],
        )
        .map(|b| Pubkey::try_from(b).unwrap())
    }

    fn mint_supply(&self) -> u64 {
        let d = self.data();
        let b: &[u8; 8] = &d[SPL_MINT_SUPPLY_OFFSET..SPL_MINT_SUPPLY_OFFSET + 8]
            .try_into()
            .unwrap();
        u64::from_le_bytes(*b)
    }

    fn mint_decimals(&self) -> u8 {
        let d = self.data();
        d[SPL_MINT_DECIMALS_OFFSET]
    }

    fn mint_is_initialized(&self) -> bool {
        let d = self.data();
        let b = d[SPL_MINT_IS_INITIALIZED_OFFSET];
        match b {
            SPL_MINT_IS_INITIALIZED_FALSE => false,
            SPL_MINT_IS_INITIALIZED_TRUE => true,
            _ => panic!("invalid is_initialized {b:?}"),
        }
    }

    fn mint_freeze_authority(&self) -> Option<Pubkey> {
        let d = self.data();
        unpack_coption_slice(
            &d[SPL_MINT_FREEZE_AUTHORITY_OFFSET
                ..SPL_MINT_FREEZE_AUTHORITY_OFFSET + 4 + PUBKEY_BYTES],
        )
        .map(|b| Pubkey::try_from(b).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;
    use spl_token_2022::{extension::StateWithExtensions, state::Mint};

    use crate::readonly::test_utils::{to_account, valid_coption_discm};

    use super::*;

    proptest! {
        #[test]
        fn mint_readonly_matches_full_deser_invalid(bytes: [u8; SPL_MINT_ACCOUNT_PACKED_LEN]) {
            let account = to_account(&bytes);
            let unpack_res = StateWithExtensions::<Mint>::unpack(&bytes);
            if !account.mint_data_is_valid() {
                prop_assert!(unpack_res.is_err());
            }
        }
    }

    proptest! {
        #[test]
        fn mint_readonly_matches_full_deser_valid(
            mut bytes: [u8; SPL_MINT_ACCOUNT_PACKED_LEN],
            mint_authority_discm in valid_coption_discm(),
            freeze_authority_discm in valid_coption_discm(),
        ) {
            bytes.get_mut(SPL_MINT_MINT_AUTHORITY_OFFSET..SPL_MINT_MINT_AUTHORITY_OFFSET + 4)
                .unwrap()
                .copy_from_slice(&mint_authority_discm);
            bytes.get_mut(SPL_MINT_FREEZE_AUTHORITY_OFFSET..SPL_MINT_FREEZE_AUTHORITY_OFFSET + 4)
                .unwrap()
                .copy_from_slice(&freeze_authority_discm);
            bytes[SPL_MINT_IS_INITIALIZED_OFFSET] = SPL_MINT_IS_INITIALIZED_TRUE;

            let StateWithExtensions { base: expected, .. }
                = StateWithExtensions::<Mint>::unpack(&bytes).unwrap();
            let account = to_account(&bytes);
            prop_assert_eq!(account.mint_mint_authority(), expected.mint_authority.into());
            prop_assert_eq!(account.mint_supply(), expected.supply);
            prop_assert_eq!(account.mint_decimals(), expected.decimals);
            prop_assert_eq!(account.mint_is_initialized(), expected.is_initialized);
            prop_assert_eq!(account.mint_freeze_authority(), expected.freeze_authority.into());
        }
    }
}
