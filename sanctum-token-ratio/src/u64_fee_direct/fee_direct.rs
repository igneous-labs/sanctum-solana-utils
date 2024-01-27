use core::cmp::Ordering;

use crate::{MathError, U64Ratio};

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
pub struct U64FeeDirect<N, D> {
    fee_num: N,
    fee_denom: D,
}

impl<N, D> U64FeeDirect<N, D> {
    /// For use in const contexts. Does not check if fee is valid, user must
    /// do so on their own, else invariant is violated.
    pub const fn from_fee_num_and_denom_unchecked(fee_num: N, fee_denom: D) -> Self {
        Self { fee_num, fee_denom }
    }

    pub fn to_u64_ratio(self) -> U64Ratio<N, D> {
        U64Ratio {
            num: self.fee_num,
            denom: self.fee_denom,
        }
    }

    pub const fn fee_num(&self) -> &N {
        &self.fee_num
    }

    pub const fn fee_denom(&self) -> &D {
        &self.fee_denom
    }
}

impl<N: Copy + Into<u128>, D: Copy + Into<u128>> U64FeeDirect<N, D> {
    pub fn from_fee_num_and_denom(fee_num: N, fee_denom: D) -> Result<Self, MathError> {
        if fee_num.into() > fee_denom.into() {
            return Err(MathError);
        }
        Ok(Self { fee_num, fee_denom })
    }

    pub fn is_zero(&self) -> bool {
        self.to_u64_ratio().is_zero()
    }

    /// Returns true if this fee charges 100%
    pub fn is_max(&self) -> bool {
        self.to_u64_ratio().is_one()
    }
}

impl<
        LN: Copy + Into<u128>,
        LD: Copy + Into<u128>,
        RN: Copy + Into<u128>,
        RD: Copy + Into<u128>,
    > PartialEq<U64FeeDirect<RN, RD>> for U64FeeDirect<LN, LD>
{
    fn eq(&self, rhs: &U64FeeDirect<RN, RD>) -> bool {
        self.to_u64_ratio().eq(&rhs.to_u64_ratio())
    }
}

impl<N: Copy + Into<u128>, D: Copy + Into<u128>> Eq for U64FeeDirect<N, D> {}

impl<
        LN: Copy + Into<u128>,
        LD: Copy + Into<u128>,
        RN: Copy + Into<u128>,
        RD: Copy + Into<u128>,
    > PartialOrd<U64FeeDirect<RN, RD>> for U64FeeDirect<LN, LD>
{
    fn partial_cmp(&self, rhs: &U64FeeDirect<RN, RD>) -> Option<Ordering> {
        self.to_u64_ratio().partial_cmp(&rhs.to_u64_ratio())
    }
}

impl<N: Copy + Into<u128>, D: Copy + Into<u128>> Ord for U64FeeDirect<N, D> {
    fn cmp(&self, rhs: &Self) -> Ordering {
        self.to_u64_ratio().cmp(&rhs.to_u64_ratio())
    }
}

#[cfg(all(test, feature = "std"))]
pub(crate) mod fee_direct_test_utils {
    use proptest::prelude::*;

    use super::*;

    prop_compose! {
        pub fn valid_u64_fees()
            (fee_denom in any::<u64>())
            (fee_num in 0..=fee_denom, fee_denom in Just(fee_denom)) -> U64FeeDirect<u64, u64> {
                U64FeeDirect { fee_num, fee_denom }
            }
    }

    prop_compose! {
        pub fn valid_nonzero_u64_fees()
            (fee_denom in any::<u64>())
            (fee_num in 1..=fee_denom, fee_denom in Just(fee_denom)) -> U64FeeDirect<u64, u64> {
                U64FeeDirect { fee_num, fee_denom }
            }
    }

    prop_compose! {
        pub fn valid_nonmax_u64_fees()
            (fee_denom in any::<u64>())
            (fee_num in 0..fee_denom, fee_denom in Just(fee_denom)) -> U64FeeDirect<u64, u64> {
                U64FeeDirect { fee_num, fee_denom }
            }
    }

    prop_compose! {
        pub fn valid_max_u64_fees()
            (n in 1..=u64::MAX) -> U64FeeDirect<u64, u64> {
                U64FeeDirect { fee_num: n, fee_denom: n }
            }
    }

    prop_compose! {
        fn valid_zero_denom_u64_fees()
            (fee_num in 0..=u64::MAX, fee_denom in Just(0)) -> U64FeeDirect<u64, u64> {
                U64FeeDirect { fee_num, fee_denom }
            }
    }

    prop_compose! {
        fn valid_zero_num_u64_fees()
            (fee_num in Just(0), fee_denom in 0..=u64::MAX) -> U64FeeDirect<u64, u64> {
                U64FeeDirect { fee_num, fee_denom }
            }
    }

    pub fn valid_zero_u64_fees() -> impl Strategy<Value = U64FeeDirect<u64, u64>> {
        valid_zero_num_u64_fees()
            .boxed()
            .prop_union(valid_zero_denom_u64_fees().boxed())
    }
}
