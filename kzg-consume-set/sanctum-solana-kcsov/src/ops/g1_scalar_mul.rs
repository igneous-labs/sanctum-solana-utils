use ark_bn254::Fr;
use solana_program::alt_bn128::{compression::prelude::G1, prelude::*, AltBn128Error};

use crate::{fr_to_be, FR, G1_GEN_AFFINE_UNCOMPRESSED_BE};

// DO NOT USE `solana_program::alt_bn128::prelude::ALT_BN128_MULTIPLICATION_INPUT_LEN`, IT SEEMS WRONG - 128 instead of 96
const ALT_BN128_MULTIPLICATION_INPUT_LEN: usize = 96;

#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AltBn128G1ScalarMul([u8; ALT_BN128_MULTIPLICATION_INPUT_LEN]);

impl AltBn128G1ScalarMul {
    #[inline]
    pub const fn new_zeros() -> Self {
        Self([0; ALT_BN128_MULTIPLICATION_INPUT_LEN])
    }

    #[inline]
    pub const fn with_g1_pt(mut self, g1_affine_uncompressed_be: &[u8; G1]) -> Self {
        // TODO: check if sol_memmove syscall uses less CUs, tho that makes it no longer const
        // TODO: replace with array fns once https://github.com/rust-lang/rust/issues/80697 is active
        let mut i = 0;
        while i < G1 {
            self.0[i] = g1_affine_uncompressed_be[i];
            i += 1;
        }
        self
    }

    #[inline]
    pub const fn with_g1_gen(self) -> Self {
        self.with_g1_pt(&G1_GEN_AFFINE_UNCOMPRESSED_BE)
    }

    #[inline]
    pub const fn with_scalar(mut self, scalar_be: &[u8; FR]) -> Self {
        // TODO: check if sol_memmove syscall uses less CUs, tho that makes it no longer const
        // TODO: replace with array fns once https://github.com/rust-lang/rust/issues/80697 is active
        let mut i = 0;
        while i < FR {
            self.0[G1 + i] = scalar_be[i];
            i += 1;
        }
        self
    }

    #[inline]
    pub fn with_fr(self, fr: &Fr) -> Self {
        self.with_scalar(&fr_to_be(fr))
    }

    #[inline]
    pub const fn as_buf(&self) -> &[u8; ALT_BN128_MULTIPLICATION_INPUT_LEN] {
        &self.0
    }

    #[inline]
    pub const fn to_buf(self) -> [u8; ALT_BN128_MULTIPLICATION_INPUT_LEN] {
        self.0
    }

    /// Perform the scalar multiplication operation by calling the `sol_alt_bn128_group_op` syscall
    #[inline]
    pub fn exec(&self) -> Result<[u8; ALT_BN128_MULTIPLICATION_OUTPUT_LEN], AltBn128Error> {
        let mut res = [0u8; ALT_BN128_MULTIPLICATION_OUTPUT_LEN];
        self.exec_into(&mut res)?;
        Ok(res)
    }

    #[inline]
    pub fn exec_into(
        &self,
        into: &mut [u8; ALT_BN128_MULTIPLICATION_OUTPUT_LEN],
    ) -> Result<(), AltBn128Error> {
        #[cfg(not(target_os = "solana"))]
        {
            let v = solana_program::alt_bn128::prelude::alt_bn128_multiplication(self.as_buf())?;
            into.copy_from_slice(&v);
            Ok(())
        }

        #[cfg(target_os = "solana")]
        {
            let result = unsafe {
                solana_program::syscalls::sol_alt_bn128_group_op(
                    ALT_BN128_MUL,
                    self.as_buf() as *const _ as *const u8,
                    ALT_BN128_MULTIPLICATION_INPUT_LEN as u64,
                    into as *mut _ as *mut u8,
                )
            };

            match result {
                0 => Ok(()),
                // since input lengths are valid,
                // the only way this syscall fails is if either one of the
                // inputs is invalid
                _ => Err(AltBn128Error::InvalidInputData),
            }
        }
    }
}
