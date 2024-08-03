use std::fmt::Display;

use async_trait::async_trait;
use data_encoding::BASE64;
use solana_client::{
    rpc_client::SerializableTransaction,
    rpc_config::{RpcSendTransactionConfig, RpcSimulateTransactionConfig},
};
use solana_rpc_client_api::client_error::Error as ClientError;
use solana_sdk::{
    commitment_config::{CommitmentConfig, CommitmentLevel},
    hash::Hash,
};
use solana_transaction_status::UiTransactionEncoding;

/// Enum for specifying how to handle transactions output.
/// - `SendActual` sends the actual transaction to the cluster
/// - `SimOnly` simulates the transaction against the cluster
/// - `DumpMsg` outputs base64 encoded serialized transaction to stdout for use with multisigs, explorer inspectors, or piping into other applications
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
pub enum TxSendMode {
    SendActual,
    SimOnly,
    DumpMsg,
}

impl Default for TxSendMode {
    fn default() -> Self {
        Self::SendActual
    }
}

impl Display for TxSendMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl TxSendMode {
    /// Creates the enum from a `should_dry_run` boolean flag.
    ///
    /// clap v4 ONLY: This can be used with `clap`'s [`TypedValueParser::map`](https://docs.rs/clap/latest/clap/builder/trait.TypedValueParser.html#method.map),
    /// [`BoolishValueParser`](https://docs.rs/clap/latest/clap/builder/struct.BoolishValueParser.html),
    /// and [`ArgAction::SetTrue`](https://docs.rs/clap/latest/clap/builder/enum.ArgAction.html#variant.SetTrue) to parse a `--dry-run` flag arg
    ///
    /// # Example:
    ///
    /// ```rust ignore
    /// use clap4::{builder::{BoolishValueParser, TypedValueParser}, Parser, ArgAction};
    /// use sanctum_solana_cli_utils::parse_solana_cli_config_from_path;
    ///
    /// #[derive(Parser, Debug)]
    /// #[command(author, version, about)]
    /// pub struct Args {
    ///     #[arg(
    ///         long,
    ///         short,
    ///         help = "only simulate any transactions instead of sending them",
    ///         action = ArgAction::SetTrue,
    ///         value_parser = BoolishValueParser::new().map(TxSendMode::from_should_dry_run)
    ///     )]
    ///     pub dry_run: TxSendMode,
    /// }
    /// ```
    pub fn from_should_dry_run(should_dry_run: bool) -> Self {
        match should_dry_run {
            true => Self::SimOnly,
            false => Self::SendActual,
        }
    }
}

/// This struct combines the common fields of
/// `RpcSendTransactionConfig` and `RpcSimulateTransactionConfig`,
/// while omitting some fields that are deemed to be not important for user config
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct HandleTxArgs {
    pub sig_verify: bool,
    pub skip_preflight: bool,
    pub replace_recent_blockhash: bool,
    pub sim_against_commitment: Option<CommitmentLevel>,
    pub tx_cfm_commitment: Option<CommitmentLevel>,
    pub min_context_slot: Option<u64>,
    pub max_retries: Option<usize>,
    pub inner_instructions: bool,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RecentBlockhash {
    pub hash: Hash,
    pub last_valid_blockheight: u64,
}

impl HandleTxArgs {
    /// A sane default for CLIs. Combination of [`Self::cu_sim()`] and [`Self::optimal_tx_send(CommitmentLevel::Confirmed)`]
    pub const fn cli_default() -> Self {
        Self {
            sig_verify: false,
            replace_recent_blockhash: true,
            sim_against_commitment: Some(CommitmentLevel::Processed),
            tx_cfm_commitment: Some(CommitmentLevel::Confirmed),
            skip_preflight: true,
            max_retries: None,
            min_context_slot: None,
            inner_instructions: false,
        }
    }

    /// The args used for simulating transactions to estimate compute units
    pub const fn cu_sim() -> Self {
        Self {
            sig_verify: false,
            replace_recent_blockhash: true,
            sim_against_commitment: Some(CommitmentLevel::Processed),
            // default / dont-cares
            tx_cfm_commitment: None,
            skip_preflight: false,
            min_context_slot: None,
            max_retries: None,
            inner_instructions: false,
        }
    }

    pub const fn optimal_tx_send(tx_cfm_commitment: CommitmentLevel) -> Self {
        Self {
            skip_preflight: true,
            max_retries: None, // use default behavior of having RPC fwd tx twice every sec until it expires or is confirmed
            tx_cfm_commitment: Some(tx_cfm_commitment),
            // dont-cares
            sim_against_commitment: None,
            sig_verify: false,
            replace_recent_blockhash: false,
            min_context_slot: None,
            inner_instructions: false,
        }
    }
}

impl From<HandleTxArgs> for RpcSimulateTransactionConfig {
    fn from(
        HandleTxArgs {
            sig_verify,
            replace_recent_blockhash,
            sim_against_commitment,
            min_context_slot,
            inner_instructions,
            ..
        }: HandleTxArgs,
    ) -> Self {
        Self {
            sig_verify,
            replace_recent_blockhash,
            commitment: sim_against_commitment.map(|commitment| CommitmentConfig { commitment }),
            encoding: Some(UiTransactionEncoding::Base64),
            accounts: None,
            min_context_slot,
            inner_instructions,
        }
    }
}

impl From<HandleTxArgs> for RpcSendTransactionConfig {
    fn from(
        HandleTxArgs {
            skip_preflight,
            min_context_slot,
            max_retries,
            ..
        }: HandleTxArgs,
    ) -> Self {
        Self {
            skip_preflight,
            // should always be using getLatestBlockhash fetched with Confirmed commitment
            // https://solana.com/docs/core/transactions/confirmation#use-an-appropriate-preflight-commitment-level
            preflight_commitment: Some(CommitmentLevel::Confirmed),
            encoding: Some(UiTransactionEncoding::Base64),
            max_retries,
            min_context_slot,
        }
    }
}

pub trait TxSendingRpcClient {
    /// Get blockhash with confirmed commitment. Optimal for transaction sending.
    fn get_confirmed_blockhash(&self) -> Result<RecentBlockhash, ClientError>;

    /// Handles the given transaction, outputting the following to stdout:
    /// - simulation results if `send_mode == TxSendMode::SimOnly`
    /// - transaction signature if `send_mode == TxSendMode::SendActual`
    /// - base64 encoded serialized tx if `send_mode == TxSendMode::DumpMsg`
    fn handle_tx<T: SerializableTransaction>(
        &self,
        tx: &T,
        send_mode: TxSendMode,
        args: HandleTxArgs,
    ) -> Result<(), ClientError>;
}

impl TxSendingRpcClient for solana_client::rpc_client::RpcClient {
    fn get_confirmed_blockhash(&self) -> Result<RecentBlockhash, ClientError> {
        let (hash, last_valid_blockheight) =
            self.get_latest_blockhash_with_commitment(CommitmentConfig {
                commitment: CommitmentLevel::Confirmed,
            })?;
        Ok(RecentBlockhash {
            hash,
            last_valid_blockheight,
        })
    }

    fn handle_tx<T: SerializableTransaction>(
        &self,
        tx: &T,
        send_mode: TxSendMode,
        mut args: HandleTxArgs,
    ) -> Result<(), ClientError> {
        let [tx_cfm_commitment, _sim_against_commitment] = [
            &mut args.tx_cfm_commitment,
            &mut args.sim_against_commitment,
        ]
        .map(|mut_ref| match mut_ref.as_mut() {
            Some(c) => *c,
            None => {
                let commitment = self.commitment().commitment;
                mut_ref.replace(commitment);
                commitment
            }
        });
        match send_mode {
            TxSendMode::SendActual => {
                let signature = self.send_and_confirm_transaction_with_spinner_and_config(
                    tx,
                    CommitmentConfig {
                        commitment: tx_cfm_commitment,
                    },
                    args.into(),
                )?;
                eprintln!("Signature: {}", signature);
            }
            TxSendMode::SimOnly => {
                let result = self.simulate_transaction_with_config(tx, args.into())?;
                eprintln!("Simulate result: {:#?}", result);
            }
            TxSendMode::DumpMsg => {
                // somehow `BASE64.encode(&tx.message_data())` as suggested by all the explorers
                // results in a different output that cannot be handled by their inspectors lmao
                println!("{}", BASE64.encode(&bincode::serialize(&tx).unwrap()))
            }
        };
        Ok(())
    }
}

#[async_trait]
pub trait TxSendingNonblockingRpcClient {
    /// Get blockhash with confirmed commitment. Optimal for transaction sending.
    async fn get_confirmed_blockhash(&self) -> Result<RecentBlockhash, ClientError>;

    /// Handles the given transaction, outputting the following to stdout:
    /// - simulation results if `send_mode == TxSendMode::SimOnly`
    /// - transaction signature if `send_mode == TxSendMode::SendActual`
    /// - base64 encoded serialized tx if `send_mode == TxSendMode::DumpMsg`
    async fn handle_tx<T: SerializableTransaction + Sync>(
        &self,
        tx: &T,
        send_mode: TxSendMode,
        args: HandleTxArgs,
    ) -> Result<(), ClientError>;
}

#[async_trait]
impl TxSendingNonblockingRpcClient for solana_client::nonblocking::rpc_client::RpcClient {
    async fn get_confirmed_blockhash(&self) -> Result<RecentBlockhash, ClientError> {
        let (hash, last_valid_blockheight) = self
            .get_latest_blockhash_with_commitment(CommitmentConfig {
                commitment: CommitmentLevel::Confirmed,
            })
            .await?;
        Ok(RecentBlockhash {
            hash,
            last_valid_blockheight,
        })
    }

    async fn handle_tx<T: SerializableTransaction + Sync>(
        &self,
        tx: &T,
        send_mode: TxSendMode,
        mut args: HandleTxArgs,
    ) -> Result<(), ClientError> {
        let [tx_cfm_commitment, _sim_against_commitment] = [
            &mut args.tx_cfm_commitment,
            &mut args.sim_against_commitment,
        ]
        .map(|mut_ref| match mut_ref.as_mut() {
            Some(c) => *c,
            None => {
                let commitment = self.commitment().commitment;
                mut_ref.replace(commitment);
                commitment
            }
        });
        match send_mode {
            TxSendMode::SendActual => {
                let signature = self
                    .send_and_confirm_transaction_with_spinner_and_config(
                        tx,
                        CommitmentConfig {
                            commitment: tx_cfm_commitment,
                        },
                        args.into(),
                    )
                    .await?;
                eprintln!("Signature: {}", signature);
            }
            TxSendMode::SimOnly => {
                let result = self
                    .simulate_transaction_with_config(tx, args.into())
                    .await?;
                eprintln!("Simulate result: {:#?}", result);
            }
            TxSendMode::DumpMsg => {
                // somehow `BASE64.encode(&tx.message_data())` as suggested by all the explorers
                // results in a different output that cannot be handled by their inspectors lmao
                println!("{}", BASE64.encode(&bincode::serialize(&tx).unwrap()))
            }
        };
        Ok(())
    }
}
