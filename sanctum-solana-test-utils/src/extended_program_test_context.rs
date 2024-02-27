use std::io::Write;

use async_trait::async_trait;
use solana_program_test::ProgramTestContext;
use solana_sdk::{
    account::{Account, ReadableAccount},
    bpf_loader_upgradeable::{self, UpgradeableLoaderState},
    pubkey::Pubkey,
    signature::Keypair,
    signer::Signer,
    transaction::Transaction,
};

use crate::{default_rent_exempt_lamports, load_program_so, ExtendedBanksClient};

/// Payer should have enough SOL for 2 transactions
#[derive(Debug, Copy, Clone)]
pub struct UpgradeProgramSigners<'a> {
    pub upgrade_auth: &'a Keypair,
    pub payer: &'a Keypair,
}

impl UpgradeProgramSigners<'_> {
    pub fn to_signers(&self) -> Vec<&Keypair> {
        let mut res = vec![self.payer, self.upgrade_auth];
        res.dedup();
        res
    }
}

/// For nice method syntax on [`ProgramTestContext`]
#[async_trait]
pub trait ExtendedProgramTestContext {
    /// Upgrades the program at `program_id` with the executable data in `{program_name}.so`.
    ///
    /// NB: this method calls [`ProgramTestContext::warp_forward_force_reward_interval_end`] several times,
    /// advancing the blocks of the test chain. This will affect tests that rely on sysvar::Clock being in a specific state,
    /// so be sure to make any modifications to sysvar::Clock only after calling this
    async fn upgrade_program(
        &mut self,
        program_id: Pubkey,
        program_name: &str,
        signers: UpgradeProgramSigners<'async_trait>,
    ) -> &mut Self;
}

#[async_trait]
impl ExtendedProgramTestContext for ProgramTestContext {
    async fn upgrade_program(
        &mut self,
        program_id: Pubkey,
        program_name: &str,
        signers: UpgradeProgramSigners<'async_trait>,
    ) -> &mut Self {
        let (prog_data_addr, _bump) =
            Pubkey::find_program_address(&[program_id.as_ref()], &bpf_loader_upgradeable::ID);

        let so_prog_data = load_program_so(program_name);

        // Must do program upgrades via an upgrade transaction, cannot just do
        // ctx.set_account() on prog_data_addr, else old program will not be replaced

        // Create a random buffer account to hold the new program data
        let buffer_addr = Pubkey::new_unique();
        let mut buffer_acc_data = Vec::with_capacity(
            UpgradeableLoaderState::size_of_buffer_metadata() + so_prog_data.len(),
        );
        buffer_acc_data.write_all(&1u32.to_le_bytes()).unwrap();
        buffer_acc_data.write_all(&[1u8]).unwrap();
        buffer_acc_data
            .write_all(signers.upgrade_auth.pubkey().as_ref())
            .unwrap();
        buffer_acc_data.write_all(&so_prog_data).unwrap();
        self.set_account(
            &buffer_addr,
            &Account {
                lamports: default_rent_exempt_lamports(buffer_acc_data.len()),
                data: buffer_acc_data,
                owner: bpf_loader_upgradeable::ID,
                executable: false,
                rent_epoch: u64::MAX,
            }
            .to_account_shared_data(),
        );

        // Send a tx to extend prog data if required
        let new_prog_data_len =
            so_prog_data.len() + UpgradeableLoaderState::size_of_programdata_metadata();
        let old_prog_data_len = self
            .banks_client
            .get_account_data(prog_data_addr)
            .await
            .len();
        let extend_by = new_prog_data_len.saturating_sub(old_prog_data_len);
        if extend_by > 0 {
            // must warp forward by 1 slot or will get
            // `Program was extended in this block already` error
            self.warp_forward_force_reward_interval_end().unwrap();
            let mut tx = Transaction::new_with_payer(
                &[bpf_loader_upgradeable::extend_program(
                    &program_id,
                    Some(&signers.upgrade_auth.pubkey()),
                    extend_by.try_into().unwrap(),
                )],
                Some(&signers.payer.pubkey()),
            );
            tx.sign(&signers.to_signers(), self.last_blockhash);
            self.banks_client.process_transaction(tx).await.unwrap();
        }

        // Send the upgrade tx.

        // must warp forward by 1 slot again. Otherwise, if extend tx was sent in same slot or if program was deployed
        // (ProgramTest::start_with_context()) in the same slot, the tx will throw
        // `Program was deployed in this block already` error
        self.warp_forward_force_reward_interval_end().unwrap();
        let mut tx = Transaction::new_with_payer(
            &[bpf_loader_upgradeable::upgrade(
                &program_id,
                &buffer_addr,
                &signers.upgrade_auth.pubkey(),
                &signers.upgrade_auth.pubkey(), // spill back to upgrade_auth
            )],
            Some(&signers.payer.pubkey()),
        );
        tx.sign(&signers.to_signers(), self.last_blockhash);
        self.banks_client.process_transaction(tx).await.unwrap();

        // Need to warp forward by 1 slot one last time because newly upgraded programs are not visible
        // on their slot of deployment:
        // https://github.com/solana-labs/solana/blob/cd4cf814fc2ffb84d6165231d2578cd7c6a25dcb/programs/loader-v4/src/lib.rs#L604
        self.warp_forward_force_reward_interval_end().unwrap();

        self
    }
}

/*
#[cfg(test)]
mod tests {
    use solana_program_test::ProgramTest;

    use super::*;

    #[tokio::test]
    async fn upgrade_program_comptime_lifetime_check() {
        let pt = ProgramTest::default();
        let mut ctx = pt.start_with_context().await;
        let payer = Keypair::new();
        let upgrade_auth = Keypair::new();
        ctx.upgrade_program(
            Pubkey::new_unique(),
            "non_existent_program",
            UpgradeProgramSigners {
                upgrade_auth: &upgrade_auth,
                payer: &payer,
            },
        )
        .await;
    }
}
*/
