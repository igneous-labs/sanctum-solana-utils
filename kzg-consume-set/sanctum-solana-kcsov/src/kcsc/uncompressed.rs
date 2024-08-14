use core::fmt::Display;

use solana_program::alt_bn128::{
    compression::{
        prelude::{G1, G2, G2_COMPRESSED},
        AltBn128CompressionError,
    },
    prelude::*,
    AltBn128Error,
};

use crate::{AltBn128G1G2Pairing, KCSCCompress, G2_GEN_AFFINE_UNCOMPRESSED_BE};

use super::{KCSCCMut, KCSCCOwned};

/// KZG Consume Set Commitment Uncompressed.
///
/// This is one of the main types to work with for the final proof verification.
/// The other types can be converted into this.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct KCSCU<'a>(&'a [u8; G2]);

impl<'a> KCSCU<'a> {
    #[inline]
    pub const fn new_unchecked(a: &'a [u8; G2]) -> Self {
        Self(a)
    }

    #[inline]
    pub const fn as_buf(&self) -> &[u8; G2] {
        self.0
    }

    /// Returns the pairing result to compare against
    /// $e(Z(\tau), \pi)$ to verify that all the roots of $Z(x)$
    /// are part of the committed set
    #[inline]
    pub fn expected_pairing(&self) -> Result<[u8; ALT_BN128_PAIRING_OUTPUT_LEN], AltBn128Error> {
        const INPUT: AltBn128G1G2Pairing = AltBn128G1G2Pairing::new_zeroes().with_g1_gen();
        INPUT.with_g2_pt(self.0).exec()
    }

    #[inline]
    pub fn compress(&self) -> Result<KCSCCOwned, AltBn128CompressionError> {
        let mut res = KCSCCOwned::new_unchecked([0; G2_COMPRESSED]);
        self.compress_into(res.borrowed_mut())?;
        Ok(res)
    }

    #[inline]
    pub fn compress_into<'i>(
        &self,
        into: KCSCCMut<'i>,
    ) -> Result<KCSCCMut<'i>, AltBn128CompressionError> {
        KCSCCompress::new(*self, into).exec()
    }

    /// Returns if the committed set is empty
    #[inline]
    pub const fn is_empty(&self) -> bool {
        let mut res = true;
        let mut i = 0;
        while i < G2 {
            res &= self.0[i] == G2_GEN_AFFINE_UNCOMPRESSED_BE[i];
            i += 1
        }
        res
    }

    //// Returns `true` if polynomial $p(x)$ with proof = `poly_proof` is a factor of the committed polynomial
    /// i.e. if the roots of p(x) are members of the committed set.
    ///
    /// `poly_proof` = $\frac{p(\tau)}{z(\tau)}G2$, single G2 point affine uncompressed big-endian
    ///
    /// `z_tau_g1` = $z(\tau)G1$, single G1 point affine uncompressed big-endian.
    #[inline]
    pub fn is_poly_factor(
        &self,
        poly_proof: &[u8; G2],
        z_tau_g1: &[u8; G1],
    ) -> Result<bool, AltBn128Error> {
        let expected = self.expected_pairing()?;
        let calculated = crate::AltBn128G1G2Pairing::new_zeroes()
            .with_g1_pt(z_tau_g1)
            .with_g2_pt(poly_proof)
            .exec()?;
        Ok(expected == calculated)
    }
}

/// Mutable KZG Consume Set Commitment Uncompressed
///
/// This is one of the main types to work with for the final proof verification.
/// The other types can be converted into this.
#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct KCSCUMut<'a>(pub(crate) &'a mut [u8; G2]);

#[derive(Debug, PartialEq, Eq)]
pub enum KCSConsumeError {
    InvalidProof,
    Syscall(AltBn128Error),
}

impl From<AltBn128Error> for KCSConsumeError {
    fn from(value: AltBn128Error) -> Self {
        Self::Syscall(value)
    }
}

impl Display for KCSConsumeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidProof => f.write_str("InvalidProof"),
            Self::Syscall(e) => write!(f, "{}", e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for KCSConsumeError {}

impl<'a> KCSCUMut<'a> {
    #[inline]
    pub fn new_unchecked(a: &'a mut [u8; G2]) -> Self {
        Self(a)
    }

    #[inline]
    pub const fn borrowed(&self) -> KCSCU<'_> {
        KCSCU::new_unchecked(self.0)
    }

    #[inline]
    pub fn replace(&mut self, new: KCSCU) -> KCSCUOwned {
        KCSCUOwned::new_unchecked(core::mem::replace(self.0, *new.0))
    }

    /// Verifies that the roots of polynomial $p(x)$ are indeed
    /// members of the committed set and then updates `self`
    /// to represent the new proof of the new set with these roots removed.
    #[inline]
    pub fn consume_poly(
        &mut self,
        poly_proof: &[u8; G2],
        z_tau_g1: &[u8; G1],
    ) -> Result<(), KCSConsumeError> {
        match self.borrowed().is_poly_factor(poly_proof, z_tau_g1)? {
            true => {
                *self.0 = *poly_proof;
                Ok(())
            }
            false => Err(KCSConsumeError::InvalidProof),
        }
    }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct KCSCUOwned([u8; G2]);

impl KCSCUOwned {
    #[inline]
    pub const fn new_unchecked(a: [u8; G2]) -> Self {
        Self(a)
    }

    #[inline]
    pub const fn borrowed(&self) -> KCSCU<'_> {
        KCSCU::new_unchecked(&self.0)
    }

    #[inline]
    pub fn borrowed_mut(&mut self) -> KCSCUMut<'_> {
        KCSCUMut::new_unchecked(&mut self.0)
    }
}
