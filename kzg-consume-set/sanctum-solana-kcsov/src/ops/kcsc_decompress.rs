use solana_program::alt_bn128::compression::AltBn128CompressionError;

use crate::{KCSCUMut, KCSCC};

#[derive(Debug, PartialEq, Eq)]
pub struct KCSCDecompress<'f, 'i> {
    from: KCSCC<'f>,
    into: KCSCUMut<'i>,
}

impl<'f, 'i> KCSCDecompress<'f, 'i> {
    #[inline]
    pub const fn new(from: KCSCC<'f>, into: KCSCUMut<'i>) -> Self {
        Self { from, into }
    }

    #[inline]
    pub fn exec(self) -> Result<KCSCUMut<'i>, AltBn128CompressionError> {
        #[cfg(not(target_os = "solana"))]
        {
            panic!("only available on target_os = 'solana'")
        }

        #[cfg(target_os = "solana")]
        {
            use solana_program::alt_bn128::compression::prelude::*;

            let result = unsafe {
                solana_program::syscalls::sol_alt_bn128_compression(
                    ALT_BN128_G2_DECOMPRESS,
                    self.from.as_buf() as *const _ as *const u8,
                    G2 as u64,
                    self.into.0 as *mut _ as *mut u8,
                )
            };
            match result {
                0 => Ok(self.into),
                _ => Err(AltBn128CompressionError::UnexpectedError),
            }
        }
    }
}
