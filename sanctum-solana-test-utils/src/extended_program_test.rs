use std::path::Path;

use solana_program::{pubkey::Pubkey, system_program};
use solana_program_test::ProgramTest;
use solana_readonly_account::sdk::KeyedAccount;
use solana_sdk::account::Account;

use crate::{test_fixtures_dir, KeyedUiAccount};

/// For nice method syntax on `ProgramTest`
pub trait ExtendedProgramTest {
    fn add_account_chained(self, address: Pubkey, account: Account) -> Self;
    fn add_keyed_account(self, keyed_account: KeyedAccount) -> Self;
    fn add_keyed_ui_account(self, keyed_ui_account: KeyedUiAccount) -> Self;
    fn add_account_from_file<P: AsRef<Path>>(self, json_file_path: P) -> Self;
    fn add_test_fixtures_account<P: AsRef<Path>>(self, relative_json_file_path: P) -> Self;
    fn add_system_account(self, address: Pubkey, lamports: u64) -> Self;
}

impl ExtendedProgramTest for ProgramTest {
    fn add_account_chained(mut self, address: Pubkey, account: Account) -> Self {
        self.add_account(address, account);
        self
    }

    fn add_keyed_account(self, KeyedAccount { pubkey, account }: KeyedAccount) -> Self {
        self.add_account_chained(pubkey, account)
    }

    fn add_keyed_ui_account(self, keyed_ui_account: KeyedUiAccount) -> Self {
        self.add_keyed_account(keyed_ui_account.to_keyed_account())
    }

    fn add_account_from_file<P: AsRef<Path>>(self, json_file_path: P) -> Self {
        self.add_keyed_ui_account(KeyedUiAccount::from_file(json_file_path))
    }

    /// Adds a KeyedUiAccount from `<test_fixtures_dir()>/relative_json_file_path`
    fn add_test_fixtures_account<P: AsRef<Path>>(self, relative_json_file_path: P) -> Self {
        self.add_account_from_file(test_fixtures_dir().join(relative_json_file_path))
    }

    fn add_system_account(self, address: Pubkey, lamports: u64) -> Self {
        self.add_account_chained(
            address,
            Account {
                lamports,
                data: Vec::new(),
                owner: system_program::ID,
                executable: false,
                rent_epoch: u64::MAX,
            },
        )
    }
}
