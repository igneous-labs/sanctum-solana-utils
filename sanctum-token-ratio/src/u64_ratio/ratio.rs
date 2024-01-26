use core::cmp::Ordering;

use crate::{MathError, U64ValueRange};

/// A ratio that is applied to a u64 token amount.
/// A zero `denom` is treated as 0.
///
/// Must use with [`crate::CeilDiv`] or [`crate::FloorDiv`]
#[derive(Debug, Copy, Clone, Default, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
pub struct U64Ratio<N, D> {
    pub num: N,
    pub denom: D,
}

impl<
        LN: Copy + Into<u128>,
        LD: Copy + Into<u128>,
        RN: Copy + Into<u128>,
        RD: Copy + Into<u128>,
    > PartialEq<U64Ratio<RN, RD>> for U64Ratio<LN, LD>
{
    fn eq(&self, rhs: &U64Ratio<RN, RD>) -> bool {
        let ln: u128 = self.num.into();
        let ld: u128 = self.denom.into();
        let rn: u128 = rhs.num.into();
        let rd: u128 = rhs.denom.into();

        // panic on overflow, even if overflow checks off
        let lhs = ln.checked_mul(rd).unwrap();
        let rhs = rn.checked_mul(ld).unwrap();

        lhs == rhs
    }
}

impl<N: Copy + Into<u128>, D: Copy + Into<u128>> Eq for U64Ratio<N, D> {}

fn cmp_inner<
    LN: Copy + Into<u128>,
    LD: Copy + Into<u128>,
    RN: Copy + Into<u128>,
    RD: Copy + Into<u128>,
>(
    lhs: &U64Ratio<LN, LD>,
    rhs: &U64Ratio<RN, RD>,
) -> Ordering {
    let ln: u128 = lhs.num.into();
    let ld: u128 = lhs.denom.into();
    let rn: u128 = rhs.num.into();
    let rd: u128 = rhs.denom.into();

    // panic on overflow, even if overflow checks off
    let lhs = ln.checked_mul(rd).unwrap();
    let rhs = rn.checked_mul(ld).unwrap();

    lhs.cmp(&rhs)
}

impl<
        LN: Copy + Into<u128>,
        LD: Copy + Into<u128>,
        RN: Copy + Into<u128>,
        RD: Copy + Into<u128>,
    > PartialOrd<U64Ratio<RN, RD>> for U64Ratio<LN, LD>
{
    fn partial_cmp(&self, rhs: &U64Ratio<RN, RD>) -> Option<Ordering> {
        Some(cmp_inner(self, rhs))
    }
}

impl<N: Copy + Into<u128>, D: Copy + Into<u128>> Ord for U64Ratio<N, D> {
    fn cmp(&self, rhs: &Self) -> Ordering {
        cmp_inner(self, rhs)
    }
}

pub trait ReversibleRatio {
    fn apply(&self, amount: u64) -> Result<u64, MathError>;

    fn reverse(&self, amt_after_apply: u64) -> Result<U64ValueRange, MathError>;
}

impl<T: ReversibleRatio> ReversibleRatio for &T {
    fn apply(&self, amount: u64) -> Result<u64, MathError> {
        (*self).apply(amount)
    }

    fn reverse(&self, amt_after_apply: u64) -> Result<U64ValueRange, MathError> {
        (*self).reverse(amt_after_apply)
    }
}
