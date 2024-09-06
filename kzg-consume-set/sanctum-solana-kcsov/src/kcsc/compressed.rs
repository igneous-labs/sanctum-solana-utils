//! Compressed commitments for cheaper onchain storage, at the expense of additional compress/decompress syscalls
//!
//! Types in here must be uncompressed to get variants of [`KCSCU`](crate::KCSCU)
//! to perform the actual KZG Consume Set operations.
//!
//! Afterwards, they can be compressed back into these types to store.

use solana_program::alt_bn128::compression::{
    prelude::{G2, G2_COMPRESSED},
    AltBn128CompressionError,
};

use crate::KCSCDecompress;

use super::{KCSCUMut, KCSCUOwned};

/// KZG Consume Set Commitment Compressed
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct KCSCC<'a>(&'a [u8; G2_COMPRESSED]);

impl<'a> KCSCC<'a> {
    #[inline]
    pub const fn new_unchecked(a: &'a [u8; G2_COMPRESSED]) -> Self {
        Self(a)
    }

    #[inline]
    pub const fn as_buf(&self) -> &[u8; G2_COMPRESSED] {
        self.0
    }

    #[inline]
    pub fn decompress(&self) -> Result<KCSCUOwned, AltBn128CompressionError> {
        let mut res = KCSCUOwned::new_unchecked([0; G2]);
        self.decompress_into(res.borrowed_mut())?;
        Ok(res)
    }

    #[inline]
    pub fn decompress_into<'i>(
        &self,
        into: KCSCUMut<'i>,
    ) -> Result<KCSCUMut<'i>, AltBn128CompressionError> {
        KCSCDecompress::new(*self, into).exec()
    }
}

/// Mutable KZG Consume Set Commitment Compressed
#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct KCSCCMut<'a>(pub(crate) &'a mut [u8; G2_COMPRESSED]);

impl<'a> KCSCCMut<'a> {
    #[inline]
    pub fn new_unchecked(a: &'a mut [u8; G2_COMPRESSED]) -> Self {
        Self(a)
    }

    #[inline]
    pub const fn borrowed(&self) -> KCSCC<'_> {
        KCSCC::new_unchecked(self.0)
    }

    #[inline]
    pub fn replace(&mut self, new: KCSCC) -> KCSCCOwned {
        KCSCCOwned::new_unchecked(core::mem::replace(self.0, *new.0))
    }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct KCSCCOwned([u8; G2_COMPRESSED]);

impl KCSCCOwned {
    #[inline]
    pub const fn new_unchecked(a: [u8; G2_COMPRESSED]) -> Self {
        Self(a)
    }

    #[inline]
    pub const fn borrowed(&self) -> KCSCC<'_> {
        KCSCC::new_unchecked(&self.0)
    }

    #[inline]
    pub fn borrowed_mut(&mut self) -> KCSCCMut<'_> {
        KCSCCMut::new_unchecked(&mut self.0)
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::*, rc::Rc};

    use super::*;

    #[test]
    fn simulate_direct_account_data_usage() {
        let mut data = [0u8; G2_COMPRESSED + 10];
        // Rc<RefCell<&mut [u8]>>, just like AccountInfo.data
        let account_info_data = Rc::new(RefCell::new(data.as_mut()));

        {
            let d = account_info_data.try_borrow_mut().unwrap();
            let mut kcscc = RefMut::map(d, |d| {
                <&mut [u8; G2_COMPRESSED]>::try_from(&mut d[..G2_COMPRESSED]).unwrap()
            });
            let a = KCSCCMut::new_unchecked(&mut kcscc);
            a.0[0] = 1;
        }

        assert_eq!(account_info_data.try_borrow().unwrap()[0], 1);
    }
}
