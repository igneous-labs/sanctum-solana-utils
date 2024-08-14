use solana_program::alt_bn128::{compression::prelude::G1, prelude::*, AltBn128Error};

#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AltBn128G1Add([u8; ALT_BN128_ADDITION_INPUT_LEN]);

impl AltBn128G1Add {
    #[inline]
    pub const fn new_zeros() -> Self {
        Self([0; ALT_BN128_ADDITION_INPUT_LEN])
    }

    #[inline]
    pub const fn with_lhs(mut self, g1_affine_uncompressed_be: &[u8; G1]) -> Self {
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
    pub const fn with_rhs(mut self, g1_affine_uncompressed_be: &[u8; G1]) -> Self {
        // TODO: check if sol_memmove syscall uses less CUs, tho that makes it no longer const
        // TODO: replace with array fns once https://github.com/rust-lang/rust/issues/80697 is active
        let mut i = 0;
        while i < G1 {
            self.0[G1 + i] = g1_affine_uncompressed_be[i];
            i += 1;
        }
        self
    }

    #[inline]
    pub const fn as_buf(&self) -> &[u8; ALT_BN128_ADDITION_INPUT_LEN] {
        &self.0
    }

    #[inline]
    pub const fn to_buf(self) -> [u8; ALT_BN128_ADDITION_INPUT_LEN] {
        self.0
    }

    /// Perform the scalar multiplication operation by calling the `sol_alt_bn128_group_op` syscall
    #[inline]
    pub fn exec(&self) -> Result<[u8; ALT_BN128_ADDITION_OUTPUT_LEN], AltBn128Error> {
        let mut res = [0u8; ALT_BN128_ADDITION_OUTPUT_LEN];
        self.exec_into(&mut res)?;
        Ok(res)
    }

    #[inline]
    pub fn exec_into(
        &self,
        into: &mut [u8; ALT_BN128_ADDITION_OUTPUT_LEN],
    ) -> Result<(), AltBn128Error> {
        #[cfg(not(target_os = "solana"))]
        {
            let _ = into;
            panic!("only available on target_os = 'solana'")
        }

        #[cfg(target_os = "solana")]
        {
            let result = unsafe {
                solana_program::syscalls::sol_alt_bn128_group_op(
                    ALT_BN128_ADD,
                    self.as_buf() as *const _ as *const u8,
                    ALT_BN128_ADDITION_INPUT_LEN as u64,
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
