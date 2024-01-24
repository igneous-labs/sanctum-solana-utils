use async_trait::async_trait;
use borsh::de::BorshDeserialize;
use data_encoding::BASE64;
use solana_program::pubkey::Pubkey;
use solana_program_test::{BanksClient, BanksClientError, BanksTransactionResultWithMetadata};
use solana_sdk::{
    account::Account, transaction::VersionedTransaction, transaction_context::TransactionReturnData,
};

#[async_trait]
pub trait ExtendedBanksClient {
    /// NB: return data is truncated. Probably wanna pass it through zero_padded_return_data() first
    async fn exec_get_return_data<T: Into<VersionedTransaction> + Send>(
        &mut self,
        tx: T,
    ) -> TransactionReturnData;

    async fn get_account_unwrapped(&mut self, addr: Pubkey) -> Account;

    async fn get_account_data(&mut self, addr: Pubkey) -> Vec<u8>;

    async fn get_borsh_account<T: BorshDeserialize>(&mut self, addr: Pubkey) -> T;

    async fn assert_account_not_exist(&mut self, addr: Pubkey);

    /// Execute a base64-encoded legacy or versioned transaction, returning the tx result
    ///
    /// Args:
    /// - `b64_tx` the base64 string, NOT the decoded bytes. If `str` or `String`, use `str.as_bytes()`
    async fn exec_b64_tx(
        &mut self,
        b64_tx: &[u8],
    ) -> Result<BanksTransactionResultWithMetadata, BanksClientError>;
}

#[async_trait]
impl ExtendedBanksClient for BanksClient {
    async fn exec_get_return_data<T: Into<VersionedTransaction> + Send>(
        &mut self,
        tx: T,
    ) -> TransactionReturnData {
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

    async fn exec_b64_tx(
        &mut self,
        b64_tx: &[u8],
    ) -> Result<BanksTransactionResultWithMetadata, BanksClientError> {
        let bytes = BASE64.decode(b64_tx).unwrap();
        let tx: VersionedTransaction = bincode::deserialize(&bytes).unwrap();
        self.process_transaction_with_metadata(tx).await
    }
}

#[cfg(test)]
mod tests {
    use solana_program::{
        message::{v0, VersionedMessage},
        system_instruction,
    };
    use solana_program_test::ProgramTest;
    use solana_sdk::{signer::Signer, transaction::Transaction};

    use super::*;

    #[tokio::test]
    async fn exec_b64_tx_basic() {
        let pt = ProgramTest::default();
        let (mut banks_client, payer, rbh) = pt.start().await;

        let mut test_legacy_tx = Transaction::new_with_payer(
            &[system_instruction::transfer(
                &payer.pubkey(),
                &payer.pubkey(),
                1,
            )],
            Some(&payer.pubkey()),
        );
        test_legacy_tx.sign(&[&payer], rbh);
        let b64_tx = BASE64.encode(&bincode::serialize(&test_legacy_tx).unwrap());
        banks_client.exec_b64_tx(b64_tx.as_bytes()).await.unwrap();

        let test_versioned_tx = VersionedTransaction::try_new(
            VersionedMessage::V0(
                v0::Message::try_compile(
                    &payer.pubkey(),
                    &[system_instruction::transfer(
                        &payer.pubkey(),
                        &payer.pubkey(),
                        2,
                    )],
                    &[],
                    rbh,
                )
                .unwrap(),
            ),
            &[&payer],
        )
        .unwrap();
        let b64_tx = BASE64.encode(&bincode::serialize(&test_versioned_tx).unwrap());
        banks_client.exec_b64_tx(b64_tx.as_bytes()).await.unwrap();
    }
}
