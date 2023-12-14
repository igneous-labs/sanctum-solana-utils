use solana_program::{program_option::COption, program_pack::Pack, pubkey::Pubkey};
use solana_readonly_account::sdk::KeyedAccount;
use solana_sdk::account::Account;
use spl_token::state::Mint;

use crate::{est_rent_exempt_lamports, ExtendedProgramTest, IntoAccount};

use super::{MockMintArgs, MockTokenAccountArgs};

pub trait TokenkegProgramTest {
    fn add_tokenkeg_account(self, addr: Pubkey, account: spl_token::state::Account) -> Self;
    fn add_tokenkeg_account_from_args(self, addr: Pubkey, args: MockTokenAccountArgs) -> Self;
    fn add_tokenkeg_mint_account(self, addr: Pubkey, mint: Mint) -> Self;
    fn add_tokenkeg_mint_from_args(self, addr: Pubkey, args: MockMintArgs) -> Self;
}

impl<T: ExtendedProgramTest> TokenkegProgramTest for T {
    fn add_tokenkeg_account(self, addr: Pubkey, account: spl_token::state::Account) -> Self {
        self.add_keyed_account(KeyedAccount {
            pubkey: addr,
            account: account.into_account(),
        })
    }

    fn add_tokenkeg_account_from_args(self, addr: Pubkey, args: MockTokenAccountArgs) -> Self {
        self.add_tokenkeg_account(addr, mock_tokenkeg_account(args))
    }

    fn add_tokenkeg_mint_account(self, addr: Pubkey, mint: Mint) -> Self {
        self.add_keyed_account(KeyedAccount {
            pubkey: addr,
            account: mint.into_account(),
        })
    }

    fn add_tokenkeg_mint_from_args(self, addr: Pubkey, args: MockMintArgs) -> Self {
        self.add_tokenkeg_mint_account(addr, mock_tokenkeg_mint(args))
    }
}

pub const TOKENKEG_ACC_RENT_EXEMPT_LAMPORTS: u64 =
    est_rent_exempt_lamports(spl_token::state::Account::LEN);

pub fn mock_tokenkeg_account(
    MockTokenAccountArgs {
        mint,
        authority,
        amount,
    }: MockTokenAccountArgs,
) -> spl_token::state::Account {
    let is_native = mint == spl_token::native_mint::ID;
    spl_token::state::Account {
        mint,
        owner: authority,
        amount,
        delegate: COption::None,
        state: spl_token::state::AccountState::Initialized,
        is_native: if is_native {
            COption::Some(TOKENKEG_ACC_RENT_EXEMPT_LAMPORTS)
        } else {
            COption::None
        },
        delegated_amount: 0,
        close_authority: COption::None,
    }
}

impl IntoAccount for spl_token::state::Account {
    fn into_account(self) -> Account {
        let mut data = vec![0u8; spl_token::state::Account::LEN];
        let mut lamports = TOKENKEG_ACC_RENT_EXEMPT_LAMPORTS;
        if self.is_native.is_some() {
            lamports += self.amount;
        }
        spl_token::state::Account::pack(self, &mut data).unwrap();
        Account {
            lamports,
            data,
            owner: spl_token::ID,
            executable: false,
            rent_epoch: u64::MAX,
        }
    }
}

pub fn mock_tokenkeg_mint(
    MockMintArgs {
        mint_authority,
        freeze_authority,
        supply,
        decimals,
    }: MockMintArgs,
) -> Mint {
    Mint {
        mint_authority: COption::from(mint_authority),
        supply,
        decimals,
        is_initialized: true,
        freeze_authority: COption::from(freeze_authority),
    }
}

impl IntoAccount for Mint {
    fn into_account(self) -> Account {
        let mut data = vec![0u8; Mint::LEN];
        Mint::pack(self, &mut data).unwrap();
        Account {
            lamports: est_rent_exempt_lamports(Mint::LEN),
            data,
            owner: spl_token::ID,
            executable: false,
            rent_epoch: u64::MAX,
        }
    }
}
