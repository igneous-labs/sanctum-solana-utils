use std::fmt::Display;

use async_trait::async_trait;
use solana_sdk::transaction::Transaction;

/// A flag for specifying whether to send transactions to
/// the network or just simulate them
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TxSendMode {
    SimulateOnly,
    SendActual,
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
            true => Self::SimulateOnly,
            false => Self::SendActual,
        }
    }
}

pub trait TxSendingRpcClient {
    /// Sends or simulates the given transaction, outputting the following to stdout:
    /// - simulation results if `send_mode == TxSendMode::SimulateOnly`
    /// - transaction signature if `send_mode == TxSendMode::SendActual`
    fn send_or_sim_tx(&self, tx: &Transaction, send_mode: TxSendMode);
}

impl TxSendingRpcClient for solana_client::rpc_client::RpcClient {
    fn send_or_sim_tx(&self, tx: &Transaction, send_mode: TxSendMode) {
        match send_mode {
            TxSendMode::SendActual => {
                let signature = self.send_and_confirm_transaction_with_spinner(tx).unwrap();
                println!("Signature: {}", signature);
            }
            TxSendMode::SimulateOnly => {
                let result = self.simulate_transaction(tx).unwrap();
                println!("Simulate result: {:?}", result);
            }
        }
    }
}

#[async_trait]
pub trait TxSendingNonblockingRpcClient {
    /// Sends or simulates the given transaction, outputting the following to stdout:
    /// - simulation results if `send_mode == TxSendMode::SimulateOnly`
    /// - transaction signature if `send_mode == TxSendMode::SendActual`
    async fn send_or_sim_tx(&self, tx: &Transaction, send_mode: TxSendMode);
}

#[async_trait]
impl TxSendingNonblockingRpcClient for solana_client::nonblocking::rpc_client::RpcClient {
    /// Sends or simulates the given transaction, outputting the following to stdout:
    /// - simulation results if `send_mode == TxSendMode::SimulateOnly`
    /// - transaction signature if `send_mode == TxSendMode::SendActual`
    async fn send_or_sim_tx(&self, tx: &Transaction, send_mode: TxSendMode) {
        match send_mode {
            TxSendMode::SendActual => {
                let signature = self
                    .send_and_confirm_transaction_with_spinner(tx)
                    .await
                    .unwrap();
                println!("Signature: {}", signature);
            }
            TxSendMode::SimulateOnly => {
                let result = self.simulate_transaction(tx).await.unwrap();
                println!("Simulate result: {:?}", result);
            }
        }
    }
}
