use core::cmp::Ordering;

use crate::{FeeRatio, MathError, U64Ratio};

/// A fee ratio applied directly to a token amount.
/// A zero `fee_denom` is treated as a 0.
///
/// `fee_charged = amt * fee_num / fee_denom`
///
/// `amt_after_fee = amt - fee_charged`.
///
/// Invariant: must be <= 1.0 (fee_num <= fee_denom).
/// Fields are private to guarantee this invariant
///
/// Must use with [`crate::CeilDiv`] or [`crate::FloorDiv`]
/// which determines how `/ fee_denom` is performed
#[derive(Debug, Copy, Clone, Default, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
pub struct U64FeeRatio<N, D> {
    fee_num: N,
    fee_denom: D,
}

impl<N: Copy + Into<u128>, D: Copy + Into<u128>> U64FeeRatio<N, D> {
    pub fn try_from_fee_num_and_denom(fee_num: N, fee_denom: D) -> Result<Self, MathError> {
        if fee_num.into() > fee_denom.into() {
            Err(MathError)
        } else {
            Ok(Self { fee_num, fee_denom })
        }
    }
}

impl<N: Copy + Into<u128>, D: Copy + Into<u128>> FeeRatio for U64FeeRatio<N, D> {
    type N = N;

    type D = D;

    fn to_u64_ratio(&self) -> U64Ratio<Self::N, Self::D> {
        U64Ratio {
            num: self.fee_num,
            denom: self.fee_denom,
        }
    }

    fn fee_num(&self) -> Self::N {
        self.fee_num
    }

    fn fee_denom(&self) -> Self::D {
        self.fee_denom
    }
}

impl<
        LN: Copy + Into<u128>,
        LD: Copy + Into<u128>,
        RN: Copy + Into<u128>,
        RD: Copy + Into<u128>,
    > PartialEq<U64FeeRatio<RN, RD>> for U64FeeRatio<LN, LD>
{
    fn eq(&self, rhs: &U64FeeRatio<RN, RD>) -> bool {
        self.to_u64_ratio().eq(&rhs.to_u64_ratio())
    }
}

impl<N: Copy + Into<u128>, D: Copy + Into<u128>> Eq for U64FeeRatio<N, D> {}

impl<
        LN: Copy + Into<u128>,
        LD: Copy + Into<u128>,
        RN: Copy + Into<u128>,
        RD: Copy + Into<u128>,
    > PartialOrd<U64FeeRatio<RN, RD>> for U64FeeRatio<LN, LD>
{
    fn partial_cmp(&self, rhs: &U64FeeRatio<RN, RD>) -> Option<Ordering> {
        self.to_u64_ratio().partial_cmp(&rhs.to_u64_ratio())
    }
}

impl<N: Copy + Into<u128>, D: Copy + Into<u128>> Ord for U64FeeRatio<N, D> {
    fn cmp(&self, rhs: &Self) -> Ordering {
        self.to_u64_ratio().cmp(&rhs.to_u64_ratio())
    }
}

#[cfg(all(test, feature = "std"))]
pub(crate) mod fee_ratio_test_utils {
    use proptest::prelude::*;

    use super::*;

    prop_compose! {
        pub fn valid_fee_ratio()
            (fee_denom in any::<u64>())
            (fee_num in 0..=fee_denom, fee_denom in Just(fee_denom)) -> U64FeeRatio<u64, u64> {
                U64FeeRatio { fee_num, fee_denom }
            }
    }

    prop_compose! {
        pub fn valid_nonzero_fee_ratio()
            (fee_denom in any::<u64>())
            (fee_num in 1..=fee_denom, fee_denom in Just(fee_denom)) -> U64FeeRatio<u64, u64> {
                U64FeeRatio { fee_num, fee_denom }
            }
    }

    prop_compose! {
        pub fn valid_nonmax_fee_ratio()
            (fee_denom in any::<u64>())
            (fee_num in 0..fee_denom, fee_denom in Just(fee_denom)) -> U64FeeRatio<u64, u64> {
                U64FeeRatio { fee_num, fee_denom }
            }
    }

    prop_compose! {
        pub fn valid_max_fee_ratio()
            (n in 1..=u64::MAX) -> U64FeeRatio<u64, u64> {
                U64FeeRatio { fee_num: n, fee_denom: n }
            }
    }

    prop_compose! {
        fn valid_zero_denom_fee_ratio()
            (fee_num in 0..=u64::MAX, fee_denom in Just(0)) -> U64FeeRatio<u64, u64> {
                U64FeeRatio { fee_num, fee_denom }
            }
    }

    prop_compose! {
        fn valid_zero_num_fee_ratio()
            (fee_num in Just(0), fee_denom in 0..=u64::MAX) -> U64FeeRatio<u64, u64> {
                U64FeeRatio { fee_num, fee_denom }
            }
    }

    pub fn valid_zero_fee_ratio() -> impl Strategy<Value = U64FeeRatio<u64, u64>> {
        valid_zero_num_fee_ratio()
            .boxed()
            .prop_union(valid_zero_denom_fee_ratio().boxed())
    }
}
