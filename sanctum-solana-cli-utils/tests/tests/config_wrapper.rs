use clap::{builder::ValueParser, Parser};
use sanctum_solana_cli_utils::ConfigWrapper;

#[test]
fn parse_solana_cli_config() {
    #[derive(Parser, Debug)]
    #[command(author, version, about)]
    struct Args {
        #[arg(
             long,
             short,
             help = "path to solana CLI config",
             default_value = "",
             value_parser = ValueParser::new(ConfigWrapper::parse_from_path)
         )]
        pub config: ConfigWrapper,
    }

    // panics if user doesn't have solana CLI installed
    let Args { config } = Args::parse_from(["cmd"]);
    // make sure getters work
    config.nonblocking_rpc_client();
    config.rpc_client();
    config.signer();
}
