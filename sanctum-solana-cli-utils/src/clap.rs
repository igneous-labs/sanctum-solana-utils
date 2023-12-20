//! TODO: deprecate this once solana upgrades `solana-clap-utils` to newer versions of clap
//! Stuff in here enables the usage of `solana-clap-utils` with `clap >= 3.0` instead of `clap ^2.0`

use solana_clap_utils::keypair::signer_from_path;
use solana_cli_config::{Config, CONFIG_FILE};
use solana_sdk::{
    commitment_config::{CommitmentConfig, CommitmentLevel},
    signer::Signer,
};
use std::{fmt::Display, io, str::FromStr};

/// Newtype to make `solana_cli_config::Config` compatible with clap >= 3.0
/// by implementing Clone on
#[derive(Debug, PartialEq, Eq)]
pub struct ConfigWrapper(Config);

impl Clone for ConfigWrapper {
    fn clone(&self) -> Self {
        Self(Config {
            json_rpc_url: self.0.json_rpc_url.clone(),
            websocket_url: self.0.websocket_url.clone(),
            keypair_path: self.0.keypair_path.clone(),
            address_labels: self.0.address_labels.clone(),
            commitment: self.0.commitment.clone(),
        })
    }
}

impl Display for ConfigWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl AsRef<Config> for ConfigWrapper {
    fn as_ref(&self) -> &Config {
        &self.0
    }
}

impl ConfigWrapper {
    /// Creates a synchronous `RpcClient` from the config's
    /// `json_rpc_url` and `commitment`
    pub fn rpc_client(&self) -> solana_client::rpc_client::RpcClient {
        solana_client::rpc_client::RpcClient::new_with_commitment(
            &self.0.json_rpc_url,
            CommitmentConfig {
                commitment: CommitmentLevel::from_str(&self.0.commitment).unwrap(),
            },
        )
    }

    /// Creates an asynchronous `nonblocking::RpcClient` from the config's
    /// `json_rpc_url` and `commitment`
    pub fn nonblocking_rpc_client(&self) -> solana_client::nonblocking::rpc_client::RpcClient {
        solana_client::nonblocking::rpc_client::RpcClient::new_with_commitment(
            self.0.json_rpc_url.clone(),
            CommitmentConfig {
                commitment: CommitmentLevel::from_str(&self.0.commitment).unwrap(),
            },
        )
    }

    /// Loads the wallet specified by the cli config.
    ///
    /// # Supports:
    /// - file system keypair files
    /// - SignerSourceKind::Usb (usb://ledger) without `confirm_key`
    ///
    /// # Does NOT support:
    /// - SignerSourceKind::Prompt with skip seed phrase validation
    /// - SignerSourceKind::Usb (usb://ledger) with `confirm_key`
    /// - SignerSourceKind::Pubkey
    ///
    /// See https://docs.rs/solana-clap-utils/latest/src/solana_clap_utils/keypair.rs.html#752-820 for more details.
    ///
    /// # Panics
    /// - if usb://ledger and ledger is not unlocked and on solana app
    pub fn signer(&self) -> Box<dyn Signer> {
        signer_from_path(
            &clap2::ArgMatches::default(),
            &self.0.keypair_path,
            "wallet",
            &mut None,
        )
        .unwrap()
    }

    /// parser fn that can be used in clap derive args structs.
    ///
    /// Uses default `solana_cli_config::CONFIG_FILE` (`~/.config/solana/cli/config.yml`)
    /// if `path` is the empty string.
    ///
    /// # Example:
    ///
    /// ```rust ignore
    /// use clap4::{builder::ValueParser, Parser};
    /// use sanctum_solana_cli_utils::ConfigWrapper;
    ///
    /// #[derive(Parser, Debug)]
    /// #[command(author, version, about)]
    /// pub struct Args {
    ///     #[arg(
    ///         long,
    ///         short,
    ///         help = "path to solana CLI config",
    ///         default_value = "",
    ///         value_parser = ValueParser::new(ConfigWrapper::parse_from_path)
    ///     )]
    ///     pub config: ConfigWrapper,
    /// }
    /// ```
    pub fn parse_from_path(path: &str) -> Result<Self, io::Error> {
        let p = if path.is_empty() {
            CONFIG_FILE.as_ref().ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::Other,
                    "Solana CONFIG_FILE could not identify the user's home directory",
                )
            })?
        } else {
            path
        };
        Ok(Self(Config::load(p)?))
    }
}
