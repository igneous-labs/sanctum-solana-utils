use std::error::Error;

use solana_clap_utils::keypair::signer_from_path;
use solana_sdk::signer::Signer;

/// Same as [`parse_named_signer`], but with `name` arg just set to "signer"
pub fn parse_signer(arg: &str) -> Result<Box<dyn Signer>, Box<dyn Error + 'static>> {
    parse_named_signer(ParseNamedSigner {
        name: "signer",
        arg,
    })
}

#[derive(Clone, Copy, Debug)]
pub struct ParseNamedSigner<'a> {
    pub name: &'a str,
    pub arg: &'a str,
}

/// Parses a signer arg.
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
/// # Panics
/// - if usb://ledger and ledger is not unlocked and on solana app
///
/// # Details
/// - `Box<dyn Signer>` is not `Clone`, `Send`, or `Sync`, `Box<dyn Error>` is not `Send`, `Sync`, or `'static`,
///    so you can't actually use this fn as a [`clap::builder::TypedValueParser`] in an Args struct.
///    Guess you can type the arg to a `String` first and then run this afterwards.
/// - Same thing for `Box<dyn Error>` returned by [`solana_clap_utils::keypair::signer_from_path`],
///   so this messes with usage in async/multithread contexts
///
/// See https://docs.rs/solana-clap-utils/latest/src/solana_clap_utils/keypair.rs.html#752-820 for more details.
pub fn parse_named_signer(
    ParseNamedSigner { name, arg }: ParseNamedSigner,
) -> Result<Box<dyn Signer>, Box<dyn Error + 'static>> {
    signer_from_path(&clap2::ArgMatches::default(), arg, name, &mut None)
}
