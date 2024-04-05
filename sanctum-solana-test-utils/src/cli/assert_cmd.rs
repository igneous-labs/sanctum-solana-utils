use std::process::Output;

use assert_cmd::Command;
use async_trait::async_trait;
use solana_program_test::{BanksClientError, BanksTransactionResultWithMetadata};

use crate::ExtendedBanksClient;

#[async_trait]
pub trait ExtendedCommand {
    /// Executes a sequence of newline separated base64-encoded transactions from an [`assert_cmd::Command`]'s stdout,
    /// eprinting stderr if the Command did not exit with status 0
    async fn exec_b64_txs(
        &mut self,
        bc: impl ExtendedBanksClient + Send,
    ) -> Vec<Result<BanksTransactionResultWithMetadata, BanksClientError>>;
}

#[async_trait]
impl ExtendedCommand for Command {
    async fn exec_b64_txs(
        &mut self,
        mut bc: impl ExtendedBanksClient + Send,
    ) -> Vec<Result<BanksTransactionResultWithMetadata, BanksClientError>> {
        let Output {
            stdout,
            status,
            stderr,
        } = self.output().unwrap();
        assert!(
            status.success(),
            "{}",
            std::str::from_utf8(&stderr).unwrap()
        );
        let stdout = std::str::from_utf8(&stdout).unwrap();
        // run txs in sequence, waiting on result of the prev before exec-ing next
        let mut res = vec![];
        for b64 in stdout.split('\n') {
            if !b64.is_empty() {
                res.push(bc.exec_b64_tx(b64.as_bytes()).await);
            }
        }
        res
    }
}

/// To be used with result returned from [`ExtendedCommand::exec_b64_txs`]
pub fn assert_all_txs_success_nonempty(
    exec_res: &[Result<BanksTransactionResultWithMetadata, BanksClientError>],
) {
    if exec_res.is_empty() {
        panic!("exec_res is empty");
    }
    for res in exec_res {
        res.as_ref().unwrap().result.as_ref().unwrap();
    }
}
