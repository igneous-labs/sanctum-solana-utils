use async_trait::async_trait;
use solana_program::pubkey::Pubkey;
use solana_program_test::BanksClient;
use solana_sdk::account::Account;

#[async_trait]
pub trait ExtendedBanksClient {
    async fn get_account_unwrapped(&mut self, addr: Pubkey) -> Account;

    async fn assert_account_not_exist(&mut self, addr: Pubkey);
}

#[async_trait]
impl ExtendedBanksClient for BanksClient {
    async fn get_account_unwrapped(&mut self, addr: Pubkey) -> Account {
        self.get_account(addr).await.unwrap().unwrap()
    }

    async fn assert_account_not_exist(&mut self, addr: Pubkey) {
        assert!(self.get_account(addr).await.unwrap().is_none())
    }
}
