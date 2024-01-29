use crate::{FeeRatio, MathError, U64FeeRatio, U64Ratio, BPS_DENOMINATOR};

/// A BPS fee rate applied to a token amount.
///
/// `fee_charged = amt * fee_num / 10_000`
///
/// `amt_after_fee = amt - fee_charged`.
///
/// Invariant: numerator must be <= 10_000 (fee_num <= fee_denom).
/// Fields are private to guarantee this invariant
///
/// Must use with [`crate::CeilDiv`] or [`crate::FloorDiv`]
/// which determines how `/ fee_denom` is performed.
///
/// Works the same way as [`U64FeeRatio`], with `fee_denom` set to [`BPS_DENOMINATOR`]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
pub struct U64BpsFee(u16);

#[cfg(feature = "borsh")]
pub const U64_BPS_FEE_CEIL_BORSH_SER_LEN: usize = 2;

impl U64BpsFee {
    pub const ZERO: Self = Self(0);

    pub const MAX: Self = Self(BPS_DENOMINATOR);

    /// Errors if `bps > BPS_DENOMINATOR`
    #[inline]
    pub const fn try_new(bps: u16) -> Result<Self, MathError> {
        if bps > BPS_DENOMINATOR {
            Err(MathError)
        } else {
            Ok(Self(bps))
        }
    }

    /// Panics if `bps > BPS_DENOMINATOR`
    #[inline]
    pub const fn new_unchecked(bps: u16) -> Self {
        match Self::try_new(bps) {
            Ok(s) => s,
            Err(_e) => panic!("bps > 10_000"),
        }
    }

    #[inline]
    pub fn try_to_u64_fee_ratio(&self) -> Result<U64FeeRatio<u16, u16>, MathError> {
        U64FeeRatio::try_from_fee_num_and_denom(self.0, BPS_DENOMINATOR)
    }
}

impl FeeRatio for U64BpsFee {
    type N = u16;

    type D = u16;

    #[inline]
    fn to_u64_ratio(&self) -> U64Ratio<Self::N, Self::D> {
        U64Ratio {
            num: self.0,
            denom: BPS_DENOMINATOR,
        }
    }

    #[inline]
    fn fee_num(&self) -> Self::N {
        self.0
    }

    #[inline]
    fn fee_denom(&self) -> Self::D {
        BPS_DENOMINATOR
    }
}

#[cfg(all(test, feature = "std"))]
pub(crate) mod bps_fee_test_utils {
    use proptest::prelude::*;

    use super::*;

    prop_compose! {
        pub fn invalid_bps_fee()
            (bps in 10_001..=u16::MAX) -> U64BpsFee {
                U64BpsFee(bps)
            }
    }

    prop_compose! {
        pub fn valid_bps_fee()
            (bps in 0..=BPS_DENOMINATOR) -> U64BpsFee {
                U64BpsFee(bps)
            }
    }

    prop_compose! {
        pub fn valid_nonzero_bps_fee()
            (bps in 1..=BPS_DENOMINATOR) -> U64BpsFee {
                U64BpsFee(bps)
            }
    }

    prop_compose! {
        pub fn valid_nonmax_bps_fee()
            (bps in 0..BPS_DENOMINATOR) -> U64BpsFee {
                U64BpsFee(bps)
            }
    }
}

#[cfg(all(test, feature = "std"))]
mod tests {
    use proptest::prelude::*;

    use crate::{bps_fee_test_utils::*, FeeRatioValid};

    proptest! {
        #[test]
        fn correct_valid_conditions(valid in valid_bps_fee()) {
            prop_assert!(valid.is_valid());
            prop_assert!(valid.validate().is_ok());
        }
    }

    proptest! {
        #[test]
        fn correct_invalid_conditions(invalid in invalid_bps_fee()) {
            prop_assert!(!invalid.is_valid());
            prop_assert!(invalid.validate().is_err());
        }
    }
}
