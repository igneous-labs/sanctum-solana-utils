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
        let Self { from, mut into } = self;

        #[cfg(not(target_os = "solana"))]
        {
            let arr = solana_program::alt_bn128::compression::prelude::alt_bn128_g2_decompress(
                from.as_buf(),
            )?;
            into.replace(crate::KCSCU::new_unchecked(&arr));
            Ok(into)
        }

        #[cfg(target_os = "solana")]
        {
            use solana_program::alt_bn128::compression::prelude::*;

            let result = unsafe {
                solana_program::syscalls::sol_alt_bn128_compression(
                    ALT_BN128_G2_DECOMPRESS,
                    from.as_buf() as *const _ as *const u8,
                    G2 as u64,
                    into.0 as *mut _ as *mut u8,
                )
            };
            match result {
                0 => Ok(into),
                _ => Err(AltBn128CompressionError::UnexpectedError),
            }
        }
    }
}
