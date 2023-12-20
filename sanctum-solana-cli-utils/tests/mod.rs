use clap::{
    builder::{BoolishValueParser, TypedValueParser, ValueParser},
    ArgAction, Parser,
};
use sanctum_solana_cli_utils::{ConfigWrapper, TxSendMode};

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
    Args::parse_from(["cmd"]);
}

#[test]
fn parse_tx_send_mode_from_dry_run() {
    #[derive(Parser, Debug)]
    #[command(author, version, about)]
    struct Args {
        #[arg(
            long,
            short,
            help = "only simulate any transactions instead of sending them",
            action = ArgAction::SetTrue,
            value_parser = BoolishValueParser::new().map(TxSendMode::from_should_dry_run)
        )]
        pub dry_run: TxSendMode,
    }

    assert_eq!(
        Args::parse_from(["cmd", "--dry-run"]).dry_run,
        TxSendMode::SimulateOnly
    );
    assert_eq!(Args::parse_from(["cmd"]).dry_run, TxSendMode::SendActual);
}
