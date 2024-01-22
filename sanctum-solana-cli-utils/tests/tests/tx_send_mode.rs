use clap::{
    builder::{BoolishValueParser, TypedValueParser, ValueParser},
    ArgAction, Parser,
};
use sanctum_solana_cli_utils::{
    ConfigWrapper, TxSendMode, TxSendingNonblockingRpcClient, TxSendingRpcClient,
};
use solana_sdk::{
    message::VersionedMessage, signature::Keypair, signer::Signer, system_instruction,
    transaction::Transaction,
};

#[cfg(feature = "clap")]
#[test]
fn parse_tx_send_mode_enum() {
    use clap::Parser;
    use sanctum_solana_cli_utils::TxSendMode;

    #[derive(Parser, Debug)]
    #[command(author, version, about)]
    struct Args {
        #[arg(
             long,
             short,
             help = "transaction send mode",
             default_value_t = TxSendMode::default(),
             value_enum,
         )]
        pub send_mode: TxSendMode,
    }

    assert_eq!(Args::parse_from(["cmd"]).send_mode, TxSendMode::default());
    for (arg, actual) in [
        ("send-actual", TxSendMode::SendActual),
        ("sim-only", TxSendMode::SimOnly),
        ("dump-msg", TxSendMode::DumpMsg),
    ] {
        assert_eq!(
            Args::parse_from(["cmd", "--send-mode", arg]).send_mode,
            actual
        );
    }
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
        TxSendMode::SimOnly
    );
    assert_eq!(Args::parse_from(["cmd"]).dry_run, TxSendMode::SendActual);
}

#[cfg(feature = "clap")]
#[tokio::test]
async fn base64_tx_encode() {
    use solana_sdk::{message::v0::Message, transaction::VersionedTransaction};

    #[derive(Parser, Debug)]
    #[command(author, version, about)]
    struct Args {
        #[arg(
             long,
             short,
             default_value = "",
             value_parser = ValueParser::new(ConfigWrapper::parse_from_path)
         )]
        pub config: ConfigWrapper,
    }

    // panics if user doesn't have solana CLI installed
    let Args { config } = Args::parse_from(["cmd"]);

    let payer = Keypair::new();
    let ix = system_instruction::transfer(&payer.pubkey(), &payer.pubkey(), 1);
    let test_legacy_tx = Transaction::new_with_payer(&[ix.clone()], Some(&payer.pubkey()));

    let test_versioned_tx = VersionedTransaction {
        message: VersionedMessage::V0(
            Message::try_compile(&payer.pubkey(), &[ix], &[], Default::default()).unwrap(),
        ),
        signatures: vec![],
    };

    // set `-- --nocapture` and paste stdout outputs into inspector
    // to make sure they work properly
    config
        .rpc_client()
        .handle_tx(&test_legacy_tx, TxSendMode::DumpMsg);
    config
        .nonblocking_rpc_client()
        .handle_tx(&test_legacy_tx, TxSendMode::DumpMsg)
        .await;
    config
        .rpc_client()
        .handle_tx(&test_versioned_tx, TxSendMode::DumpMsg);
    config
        .nonblocking_rpc_client()
        .handle_tx(&test_versioned_tx, TxSendMode::DumpMsg)
        .await;
}
