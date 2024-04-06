use std::{error::Error, str::FromStr};

use solana_sdk::{
    pubkey::Pubkey,
    signature::{NullSigner, Signature},
    signer::{Signer, SignerError},
};

use crate::parse_signer;

/// Source of a pubkey.
#[derive(Debug)]
pub enum PubkeySrc {
    Pubkey(NullSigner),
    Signer(Box<dyn Signer>),
}

impl PubkeySrc {
    pub fn parse(arg: &str) -> Result<Self, Box<dyn Error + 'static>> {
        if let Ok(pk) = Pubkey::from_str(arg) {
            return Ok(Self::Pubkey(NullSigner::new(&pk)));
        }
        let signer = parse_signer(arg)?;
        Ok(Self::Signer(signer))
    }

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
