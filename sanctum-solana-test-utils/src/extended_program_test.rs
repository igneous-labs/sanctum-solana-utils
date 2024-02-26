use std::{io::Write, path::Path};

use chrono_humanize::{Accuracy, HumanTime, Tense};
use solana_program::{pubkey::Pubkey, system_program};
use solana_program_test::{find_file, read_file, ProgramTest};
use solana_readonly_account::sdk::KeyedAccount;
use solana_sdk::{
    account::Account,
    bpf_loader_upgradeable::{self, UpgradeableLoaderState},
    rent::Rent,
};

use crate::{test_fixtures_dir, KeyedUiAccount};

/// For nice method syntax on `ProgramTest`
pub trait ExtendedProgramTest {
    fn add_account_chained(self, address: Pubkey, account: Account) -> Self;
    fn add_keyed_account(self, keyed_account: KeyedAccount) -> Self;
    fn add_keyed_ui_account(self, keyed_ui_account: KeyedUiAccount) -> Self;
    fn add_account_from_file<P: AsRef<Path>>(self, json_file_path: P) -> Self;
    fn add_test_fixtures_account<P: AsRef<Path>>(self, relative_json_file_path: P) -> Self;
    fn add_system_account(self, address: Pubkey, lamports: u64) -> Self;

    /// Adds a compiled BPF program as an upgradeable program.
    /// Like [ProgramTest::add_program], the program_name must match `{program_name}.so`
    ///
    /// Works the same way as [ProgramTest::add_program], except:
    /// - sets the program's owner to BpfLoaderUpgradeable instead of BpfLoader
    /// - always equivalent to prefer_bpf = true, only works with compiled .so files
    fn add_upgradeable_program(
        self,
        program_id: Pubkey,
        program_name: &str,
        upgrade_auth_addr: Option<Pubkey>,
        last_upgrade_slot: u64,
    ) -> Self;
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

    fn add_upgradeable_program(
        mut self,
        program_id: Pubkey,
        program_name: &str,
        upgrade_auth_addr: Option<Pubkey>,
        last_upgrade_slot: u64,
    ) -> Self {
        let so_file = format!("{program_name}.so");
        let program_file = find_file(&so_file).unwrap_or_else(|| {
            panic!("Program file data not available for {program_name} ({program_id})")
        });
        let so_prog_data = read_file(&program_file);
        let (prog_data_addr, _bump) =
            Pubkey::find_program_address(&[program_id.as_ref()], &bpf_loader_upgradeable::ID);

        // Copied from:
        // https://docs.rs/solana-program-test/latest/src/solana_program_test/lib.rs.html#630-650
        log::info!(
            "\"{}\" SBF program from {}{}",
            program_name,
            program_file.display(),
            std::fs::metadata(&program_file)
                .map(|metadata| {
                    metadata
                        .modified()
                        .map(|time| {
                            format!(
                                ", modified {}",
                                HumanTime::from(time).to_text_en(Accuracy::Precise, Tense::Past)
                            )
                        })
                        .ok()
                })
                .ok()
                .flatten()
                .unwrap_or_default()
        );

        // add program account
        let mut prog_acc_data = Vec::with_capacity(UpgradeableLoaderState::size_of_program());
        prog_acc_data.write_all(&2u32.to_le_bytes()).unwrap();
        prog_acc_data.write_all(prog_data_addr.as_ref()).unwrap();
        self.add_account(
            program_id,
            Account {
                lamports: Rent::default()
                    .minimum_balance(UpgradeableLoaderState::size_of_program()),
                data: prog_acc_data,
                owner: bpf_loader_upgradeable::ID,
                executable: true,
                rent_epoch: u64::MAX,
            },
        );

        // add program data account
        let mut prog_data_acc_data = Vec::with_capacity(
            UpgradeableLoaderState::size_of_programdata_metadata() + so_prog_data.len(),
        );
        prog_data_acc_data.write_all(&3u32.to_le_bytes()).unwrap();
        prog_data_acc_data
            .write_all(&last_upgrade_slot.to_le_bytes())
            .unwrap();
        match upgrade_auth_addr {
            Some(auth) => {
                prog_data_acc_data.write_all(&[1u8]).unwrap();
                prog_data_acc_data.write_all(auth.as_ref()).unwrap();
            }
            None => {
                prog_data_acc_data.write_all(&[0u8; 33]).unwrap();
            }
        }
        prog_data_acc_data.write_all(&so_prog_data).unwrap();
        self.add_account(
            prog_data_addr,
            Account {
                lamports: Rent::default().minimum_balance(prog_data_acc_data.len()),
                data: prog_data_acc_data,
                owner: bpf_loader_upgradeable::ID,
                executable: false,
                rent_epoch: u64::MAX,
            },
        );

        self
    }
}
