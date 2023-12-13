use solana_sdk::account::Account;

pub trait IntoAccount {
    fn into_account(self) -> Account;
}
