use solana_program::alt_bn128::compression::AltBn128CompressionError;

use crate::{KCSCCMut, KCSCU};

#[derive(Debug, PartialEq, Eq)]
pub struct KCSCCompress<'f, 'i> {
    from: KCSCU<'f>,
    into: KCSCCMut<'i>,
}

impl<'f, 'i> KCSCCompress<'f, 'i> {
    #[inline]
    pub const fn new(from: KCSCU<'f>, into: KCSCCMut<'i>) -> Self {
        Self { from, into }
    }

    #[inline]
    pub fn exec(self) -> Result<KCSCCMut<'i>, AltBn128CompressionError> {
        #[cfg(not(target_os = "solana"))]
        {
            let Self { from, mut into } = self;
            let arr = solana_program::alt_bn128::compression::prelude::alt_bn128_g2_compress(
                from.as_buf(),
            )?;
            into.replace(crate::KCSCC::new_unchecked(&arr));
            Ok(into)
        }

        #[cfg(target_os = "solana")]
        {
            use solana_program::alt_bn128::compression::prelude::*;

            let Self { from, into } = self;
            let result = unsafe {
                solana_program::syscalls::sol_alt_bn128_compression(
                    ALT_BN128_G2_COMPRESS,
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
