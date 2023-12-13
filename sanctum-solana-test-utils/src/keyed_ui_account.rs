use std::{fs::File, path::Path, str::FromStr};

use serde::{Deserialize, Serialize};
use solana_account_decoder::UiAccount;
use solana_program::pubkey::Pubkey;
use solana_readonly_account::sdk::KeyedAccount;

/// This is the json format of
/// `solana account -o <FILENAME>.json --output json <ACCOUNT-PUBKEY>`
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KeyedUiAccount {
    pub pubkey: String,
    pub account: UiAccount,
}

impl KeyedUiAccount {
    pub fn from_file<P: AsRef<Path>>(json_file_path: P) -> Self {
        let mut file = File::open(json_file_path).unwrap();
        serde_json::from_reader(&mut file).unwrap()
    }

    pub fn to_keyed_account(&self) -> KeyedAccount {
        KeyedAccount {
            pubkey: Pubkey::from_str(&self.pubkey).unwrap(),
            account: self.account.decode().unwrap(),
        }
    }
}
