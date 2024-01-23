use std::collections::HashMap;

use solana_cli_config::Config;
use solana_sdk::signature::Keypair;
use tempfile::NamedTempFile;

/// Contained `NamedTempFile`s are deleted when dropped
pub struct TempCliConfig {
    keypair: NamedTempFile,
    config: NamedTempFile,
}

impl TempCliConfig {
    pub fn from_keypair_and_rpc_url(keypair: &Keypair, json_rpc_url: String) -> Self {
        let kp_bytes = keypair.to_bytes();
        let keypair = NamedTempFile::new().unwrap();
        serde_json::to_writer(keypair.as_file(), kp_bytes.as_ref()).unwrap();

        let config = NamedTempFile::new().unwrap();
        serde_yaml::to_writer(
            config.as_file(),
            &Config {
                json_rpc_url,
                websocket_url: "".to_owned(),
                keypair_path: keypair.path().to_str().unwrap().to_owned(),
                address_labels: HashMap::new(),
                commitment: "confirmed".to_owned(),
            },
        )
        .unwrap();
        Self { keypair, config }
    }

    /// Config will point to `http://127.0.0.1:{port}`
    pub fn from_keypair_and_local_port(keypair: &Keypair, port: u16) -> Self {
        Self::from_keypair_and_rpc_url(keypair, format!("http://127.0.0.1:{port}"))
    }

    // Access keypair and config via getters to ensure this struct is never
    // destructured, since destructuring can drop unused fields => delete file

    pub fn keypair(&self) -> &NamedTempFile {
        &self.keypair
    }

    pub fn config(&self) -> &NamedTempFile {
        &self.config
    }
}

#[cfg(test)]
mod tests {
    use sanctum_solana_cli_utils::ConfigWrapper;
    use solana_sdk::signer::Signer;

    use super::*;

    #[test]
    fn temp_cli_config_basic() {
        let port = 12321;
        let kp = Keypair::new();
        let tcc = TempCliConfig::from_keypair_and_local_port(&kp, 12321);
        let config = ConfigWrapper::parse_from_path(tcc.config().path().to_str().unwrap()).unwrap();
        assert_eq!(
            config.as_ref().json_rpc_url,
            format!("http://127.0.0.1:{port}")
        );
        assert_eq!(config.signer().pubkey(), kp.pubkey());
    }
}
