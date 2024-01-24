use std::{error::Error, str::FromStr};

use solana_clap_utils::keypair::signer_from_path;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{NullSigner, Signature},
    signer::{Signer, SignerError},
};

/// Same as [`parse_named_signer`], but with `name` arg just set to "signer"
pub fn parse_signer(arg: &str) -> Result<Box<dyn Signer>, Box<dyn Error>> {
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
/// `Box<dyn Signer>` is not `Clone`, `Send`, or `Sync`, `Box<dyn Error>` is not `Send`, `Sync`, or `'static`,
/// so you can't actually use this fn as a [`clap::builder::TypedValueParser`] in an Args struct.
/// Guess you can type the arg to a `String` first and then run this afterwards.
///
/// See https://docs.rs/solana-clap-utils/latest/src/solana_clap_utils/keypair.rs.html#752-820 for more details.
pub fn parse_named_signer(
    ParseNamedSigner { name, arg }: ParseNamedSigner,
) -> Result<Box<dyn Signer>, Box<dyn Error>> {
    signer_from_path(&clap2::ArgMatches::default(), arg, name, &mut None)
}

/// Source of a pubkey.
#[derive(Debug)]
pub enum PubkeySrc {
    Pubkey(NullSigner),
    Signer(Box<dyn Signer>),
}

impl PubkeySrc {
    pub fn pubkey(&self) -> Pubkey {
        match self {
            Self::Pubkey(ns) => ns.pubkey(),
            Self::Signer(s) => s.pubkey(),
        }
    }

    /// If pubkey, returns a dummy NullSigner
    pub fn signer(&self) -> &dyn Signer {
        match self {
            Self::Pubkey(ns) => ns,
            Self::Signer(s) => s.as_ref(),
        }
    }
}

impl Signer for PubkeySrc {
    fn try_pubkey(&self) -> Result<Pubkey, SignerError> {
        Ok(self.pubkey())
    }

    fn try_sign_message(&self, msg: &[u8]) -> Result<Signature, SignerError> {
        Ok(self.signer().sign_message(msg))
    }

    fn is_interactive(&self) -> bool {
        self.signer().is_interactive()
    }
}

/// Parse either a pubkey or signer.
///
/// This is useful for cmds that take a signer for a checked variant
/// and a raw pubkey without checking that the user has access to the private key for
/// an unchecked variant
///
/// Uses [`parse_signer`] under the hood so all restrictions apply
pub fn parse_pubkey_src(arg: &str) -> Result<PubkeySrc, Box<dyn Error>> {
    if let Ok(pk) = Pubkey::from_str(arg) {
        return Ok(PubkeySrc::Pubkey(NullSigner::new(&pk)));
    }
    let signer = parse_signer(arg)?;
    Ok(PubkeySrc::Signer(signer))
}
