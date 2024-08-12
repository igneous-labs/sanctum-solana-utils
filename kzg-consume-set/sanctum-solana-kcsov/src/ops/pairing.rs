use solana_program::alt_bn128::{
    compression::prelude::{G1, G2},
    prelude::*,
    AltBn128Error,
};

use crate::{G1_GEN_AFFINE_UNCOMPRESSED_BE, G2_GEN_AFFINE_UNCOMPRESSED_BE};

#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AltBn128G1G2Pairing([u8; ALT_BN128_PAIRING_ELEMENT_LEN]);

impl AltBn128G1G2Pairing {
    #[inline]
    pub const fn new_zeroes() -> Self {
        Self([0; ALT_BN128_PAIRING_ELEMENT_LEN])
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
    pub const fn with_g2_pt(mut self, g2_affine_uncompressed_be: &[u8; G2]) -> Self {
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
    pub const fn with_g2_gen(self) -> Self {
        self.with_g2_pt(&G2_GEN_AFFINE_UNCOMPRESSED_BE)
    }

    #[inline]
    pub const fn as_buf(&self) -> &[u8; ALT_BN128_PAIRING_ELEMENT_LEN] {
        &self.0
    }

    #[inline]
    pub const fn to_buf(self) -> [u8; ALT_BN128_PAIRING_ELEMENT_LEN] {
        self.0
    }

    /// Perform the pairing operation by calling the `sol_alt_bn128_group_op` syscall
    #[inline]
    pub fn exec(&self) -> Result<[u8; ALT_BN128_PAIRING_OUTPUT_LEN], AltBn128Error> {
        #[cfg(not(target_os = "solana"))]
        {
            panic!("only available on target_os = 'solana'")
        }

        #[cfg(target_os = "solana")]
        {
            let mut result_buffer = [0u8; ALT_BN128_PAIRING_OUTPUT_LEN];
            let result = unsafe {
                solana_program::syscalls::sol_alt_bn128_group_op(
                    ALT_BN128_PAIRING,
                    self.as_buf() as *const _ as *const u8,
                    ALT_BN128_PAIRING_ELEMENT_LEN as u64,
                    &mut result_buffer as *mut _ as *mut u8,
                )
            };

            match result {
                0 => Ok(result_buffer),
                // since input lengths are valid,
                // the only way this syscall fails is if either one of the
                // uncompressed points is not a valid group elem of
                // its respective curve
                _ => Err(AltBn128Error::InvalidInputData),
            }
        }
    }
}
