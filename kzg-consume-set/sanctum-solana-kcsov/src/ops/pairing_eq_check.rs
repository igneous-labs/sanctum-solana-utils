use solana_program::alt_bn128::{
    compression::prelude::{G1, G2},
    prelude::*,
    AltBn128Error,
};

use crate::{u256_be_sub, FQ, G1_GEN_AFFINE_UNCOMPRESSED_BE, G2_GEN_AFFINE_UNCOMPRESSED_BE, Q_BE};

pub const ALT_BN128_PAIRING_EQ_CHECK_LEN: usize = 2 * ALT_BN128_PAIRING_ELEMENT_LEN;

/// Verifies that e(g1a, g2a) = e(g1b, g2b) by verifying that
/// e(-g1a, g2a) = e(g1b, g2b) using the
/// verify e(w, x) * e(y, z) * ... = 1 syscall
///
/// Because
/// e(g1a, g2a) = e(g1b, g2b)
/// 1 = -e(g1a, g2a) * e(g1b, g2b)
/// 1 = e(-g1a, g2a) * e(g1b, g2b) (bilinearity)
#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AltBn128G1G2PairingEqCheck([u8; ALT_BN128_PAIRING_EQ_CHECK_LEN]);

impl AltBn128G1G2PairingEqCheck {
    #[inline]
    pub const fn new_zeroes() -> Self {
        Self([0; ALT_BN128_PAIRING_EQ_CHECK_LEN])
    }

    #[inline]
    pub const fn with_g1a(mut self, g1_affine_uncompressed_be: &[u8; G1]) -> Self {
        // TODO: check if sol_memmove syscall uses less CUs, tho that makes it no longer const
        // TODO: replace with array fns once https://github.com/rust-lang/rust/issues/80697 is active

        // fill in x
        let mut i = 0;
        while i < FQ {
            self.0[i] = g1_affine_uncompressed_be[i];
            i += 1;
        }

        // negate and fill in y
        // TODO: verify that this unsafe block isnt UB
        let y: &[u8; FQ] = unsafe {
            let start_ptr = &g1_affine_uncompressed_be[FQ] as *const u8;
            let start_ptr = start_ptr.cast::<[u8; FQ]>();
            &*start_ptr
        };
        let y = u256_be_sub(&Q_BE, y);
        let mut i = 0;
        while i < FQ {
            self.0[FQ + i] = y[i];
            i += 1;
        }

        self
    }

    #[inline]
    pub const fn with_g1a_g1_gen(self) -> Self {
        self.with_g1a(&G1_GEN_AFFINE_UNCOMPRESSED_BE)
    }

    #[inline]
    pub const fn with_g2a(mut self, g2_affine_uncompressed_be: &[u8; G2]) -> Self {
        // TODO: check if sol_memmove syscall uses less CUs, tho that makes it no longer const
        // TODO: replace with array fns once https://github.com/rust-lang/rust/issues/80697 is active
        let mut i = 0;
        while i < G2 {
            self.0[G1 + i] = g2_affine_uncompressed_be[i];
            i += 1;
        }
        self
    }

    #[inline]
    pub const fn with_g2a_gen(self) -> Self {
        self.with_g2a(&G2_GEN_AFFINE_UNCOMPRESSED_BE)
    }

    #[inline]
    pub const fn with_g1b(mut self, g1_affine_uncompressed_be: &[u8; G1]) -> Self {
        // TODO: check if sol_memmove syscall uses less CUs, tho that makes it no longer const
        // TODO: replace with array fns once https://github.com/rust-lang/rust/issues/80697 is active
        let mut i = 0;
        while i < G1 {
            self.0[ALT_BN128_PAIRING_ELEMENT_LEN + i] = g1_affine_uncompressed_be[i];
            i += 1;
        }
        self
    }

    #[inline]
    pub const fn with_g1b_g1_gen(self) -> Self {
        self.with_g1b(&G1_GEN_AFFINE_UNCOMPRESSED_BE)
    }

    #[inline]
    pub const fn with_g2b(mut self, g2_affine_uncompressed_be: &[u8; G2]) -> Self {
        // TODO: check if sol_memmove syscall uses less CUs, tho that makes it no longer const
        // TODO: replace with array fns once https://github.com/rust-lang/rust/issues/80697 is active
        let mut i = 0;
        while i < G2 {
            self.0[ALT_BN128_PAIRING_ELEMENT_LEN + G1 + i] = g2_affine_uncompressed_be[i];
            i += 1;
        }
        self
    }

    #[inline]
    pub const fn with_g2b_gen(self) -> Self {
        self.with_g2b(&G2_GEN_AFFINE_UNCOMPRESSED_BE)
    }

    #[inline]
    pub const fn as_buf(&self) -> &[u8; ALT_BN128_PAIRING_EQ_CHECK_LEN] {
        &self.0
    }

    #[inline]
    pub const fn to_buf(self) -> [u8; ALT_BN128_PAIRING_EQ_CHECK_LEN] {
        self.0
    }

    /// Perform the pairing operation by calling the `sol_alt_bn128_group_op` syscall
    #[inline]
    pub fn exec(&self) -> Result<bool, AltBn128Error> {
        #[cfg(not(target_os = "solana"))]
        {
            use ark_bn254::{G1Affine, G2Affine};
            use ark_ff::Field;
            use ark_serialize::CanonicalDeserialize;

            let [g1a, g1b] = [0, ALT_BN128_PAIRING_ELEMENT_LEN].map(|start| {
                let mut g1 = [0u8; G1];
                g1.copy_from_slice(&self.0[start..start + G1]);
                g1[..G1 / 2].reverse();
                g1[G1 / 2..].reverse();
                G1Affine::deserialize_uncompressed_unchecked(g1.as_slice())
                    .map_err(|_e| AltBn128Error::InvalidInputData)
            });
            let g1a = g1a?;
            let g1b = g1b?;

            let [g2a, g2b] = [G1, ALT_BN128_PAIRING_ELEMENT_LEN + G1].map(|start| {
                let mut g2 = [0u8; G2];
                g2.copy_from_slice(&self.0[start..start + G2]);
                g2[..G2 / 2].reverse();
                g2[G2 / 2..].reverse();
                G2Affine::deserialize_uncompressed_unchecked(g2.as_slice())
                    .map_err(|_e| AltBn128Error::InvalidInputData)
            });
            let g2a = g2a?;
            let g2b = g2b?;

            let res = <ark_bn254::Bn254 as ark_ec::pairing::Pairing>::multi_pairing(
                [g1a, g1b],
                [g2a, g2b],
            );
            Ok(res.0 == ark_bn254::Fq12::ONE)
        }

        #[cfg(target_os = "solana")]
        {
            const ONE_U256_BE: [u8; 32] = {
                let mut res = [0u8; 32];
                res[31] = 1;
                res
            };

            let mut res = [0; ALT_BN128_PAIRING_OUTPUT_LEN];
            let result = unsafe {
                solana_program::syscalls::sol_alt_bn128_group_op(
                    ALT_BN128_PAIRING,
                    self.as_buf() as *const _ as *const u8,
                    ALT_BN128_PAIRING_EQ_CHECK_LEN as u64,
                    &mut res as *mut _ as *mut u8,
                )
            };

            match result {
                0 => Ok(res == ONE_U256_BE),
                // since input lengths are valid,
                // the only way this syscall fails is if either one of the
                // uncompressed points is not a valid group elem of
                // its respective curve
                _ => Err(AltBn128Error::InvalidInputData),
            }
        }
    }
}
