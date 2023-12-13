use async_trait::async_trait;
use borsh::de::BorshDeserialize;
use solana_program::pubkey::Pubkey;
use solana_program_test::{BanksClient, BanksTransactionResultWithMetadata};
use solana_sdk::{
    account::Account, transaction::Transaction, transaction_context::TransactionReturnData,
};

#[async_trait]
pub trait ExtendedBanksClient {
    /// NB: return data is truncated. Probably wanna pass it through zero_padded_return_data() first
    async fn exec_get_return_data(&mut self, tx: Transaction) -> TransactionReturnData;

    async fn get_account_unwrapped(&mut self, addr: Pubkey) -> Account;

    async fn get_account_data(&mut self, addr: Pubkey) -> Vec<u8>;

    async fn get_borsh_account<T: BorshDeserialize>(&mut self, addr: Pubkey) -> T;

    async fn assert_account_not_exist(&mut self, addr: Pubkey);
}

#[async_trait]
impl ExtendedBanksClient for BanksClient {
    async fn exec_get_return_data(&mut self, tx: Transaction) -> TransactionReturnData {
        let BanksTransactionResultWithMetadata { result, metadata } =
            self.process_transaction_with_metadata(tx).await.unwrap();
        result.unwrap(); // check result ok
        metadata.unwrap().return_data.unwrap()
    }

    async fn get_account_unwrapped(&mut self, addr: Pubkey) -> Account {
        self.get_account(addr).await.unwrap().unwrap()
    }

    async fn get_account_data(&mut self, addr: Pubkey) -> Vec<u8> {
        self.get_account(addr).await.unwrap().unwrap().data
    }

    async fn get_borsh_account<T: BorshDeserialize>(&mut self, addr: Pubkey) -> T {
        // get_account_data_with_borsh() has some issues with trait bounds
        let data = self.get_account_data(addr).await;
        T::deserialize(&mut data.as_ref()).unwrap()
    }

    async fn assert_account_not_exist(&mut self, addr: Pubkey) {
        assert!(self.get_account(addr).await.unwrap().is_none())
    }
}
