use proptest::{
    prelude::prop_compose,
    strategy::{Just, Strategy, Union},
};
use spl_token::state::{AccountState, Mint};

use crate::proptest_utils::{coption_pubkey, coption_u64, pubkey};

pub fn token_account_state() -> impl Strategy<Value = AccountState> {
    Union::new([
        Just(AccountState::Uninitialized),
        Just(AccountState::Initialized),
        Just(AccountState::Frozen),
    ])
}

prop_compose! {
    pub fn tokenkeg_account()
        (
            amount: u64,
            delegated_amount: u64,
            mint in pubkey(),
            owner in pubkey(),
            state in token_account_state(),
            delegate in coption_pubkey(),
            close_authority in coption_pubkey(),
            is_native in coption_u64(),
        ) -> spl_token::state::Account {
            spl_token::state::Account {
                mint,
                owner,
                amount,
                delegate,
                state,
                is_native,
                delegated_amount,
                close_authority,
            }
        }
}

prop_compose! {
    pub fn tokenkeg_mint()
        (
            mint_authority in coption_pubkey(),
            supply: u64,
            decimals: u8,
            is_initialized: bool,
            freeze_authority in coption_pubkey(),
        ) -> Mint {
            Mint {
                mint_authority,
                supply,
                decimals,
                is_initialized,
                freeze_authority,
            }
        }
}
