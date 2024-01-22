use std::fmt::Display;

use async_trait::async_trait;
use data_encoding::BASE64;
use solana_client::rpc_client::SerializableTransaction;

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

pub trait TxSendingRpcClient {
    /// Handles the given transaction, outputting the following to stdout:
    /// - simulation results if `send_mode == TxSendMode::SimOnly`
    /// - transaction signature if `send_mode == TxSendMode::SendActual`
    /// - base64 encoded serialized tx if `send_mode == TxSendMode::DumpMsg`
    fn handle_tx<T: SerializableTransaction>(&self, tx: &T, send_mode: TxSendMode);
}

impl TxSendingRpcClient for solana_client::rpc_client::RpcClient {
    fn handle_tx<T: SerializableTransaction>(&self, tx: &T, send_mode: TxSendMode) {
        match send_mode {
            TxSendMode::SendActual => {
                let signature = self.send_and_confirm_transaction_with_spinner(tx).unwrap();
                eprintln!("Signature: {}", signature);
            }
            TxSendMode::SimOnly => {
                let result = self.simulate_transaction(tx).unwrap();
                eprintln!("Simulate result: {:?}", result);
            }
            TxSendMode::DumpMsg => {
                // somehow `BASE64.encode(&tx.message_data())` as suggested by all the explorers
                // results in a different output that cannot be handled by their inspectors lmao
                println!("{}", BASE64.encode(&bincode::serialize(&tx).unwrap()))
            }
        }
    }
}

#[async_trait]
pub trait TxSendingNonblockingRpcClient {
    /// Handles the given transaction, outputting the following to stdout:
    /// - simulation results if `send_mode == TxSendMode::SimOnly`
    /// - transaction signature if `send_mode == TxSendMode::SendActual`
    /// - base64 encoded serialized tx if `send_mode == TxSendMode::DumpMsg`
    async fn handle_tx<T: SerializableTransaction + Sync>(&self, tx: &T, send_mode: TxSendMode);
}

#[async_trait]
impl TxSendingNonblockingRpcClient for solana_client::nonblocking::rpc_client::RpcClient {
    async fn handle_tx<T: SerializableTransaction + Sync>(&self, tx: &T, send_mode: TxSendMode) {
        match send_mode {
            TxSendMode::SendActual => {
                let signature = self
                    .send_and_confirm_transaction_with_spinner(tx)
                    .await
                    .unwrap();
                eprintln!("Signature: {}", signature);
            }
            TxSendMode::SimOnly => {
                let result = self.simulate_transaction(tx).await.unwrap();
                eprintln!("Simulate result: {:?}", result);
            }
            TxSendMode::DumpMsg => {
                // somehow `BASE64.encode(&tx.message_data())` as suggested by all the explorers
                // results in a different output that cannot be handled by their inspectors lmao
                println!("{}", BASE64.encode(&bincode::serialize(&tx).unwrap()))
            }
        }
    }
}
