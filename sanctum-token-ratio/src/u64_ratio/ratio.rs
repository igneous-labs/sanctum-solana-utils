use core::cmp::Ordering;

// Cannot derive Hash because to ensure
// `k1 == k2 -> hash(k1) == hash(k2)`
// invariant is not violated, we need to hash the fraction's lowest form
// https://doc.rust-lang.org/std/hash/trait.Hash.html#hash-and-eq

/// A ratio that is applied to a u64 token amount.
/// A zero `denom` is treated as 0.
///
/// Must use with [`crate::CeilDiv`] or [`crate::FloorDiv`]
#[derive(Debug, Copy, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
pub struct U64Ratio<N, D> {
    pub num: N,
    pub denom: D,
}

impl<N: Copy + Into<u128>, D: Copy + Into<u128>> U64Ratio<N, D> {
    pub fn is_zero(&self) -> bool {
        self.num.into() == 0 || self.denom.into() == 0
    }

    /// Returns true if this ratio represents 1.0 i.e. num == denom
    pub fn is_one(&self) -> bool {
        !self.is_zero() && self.num.into() == self.denom.into()
    }
}

fn cmp_inner<
    LN: Copy + Into<u128>,
    LD: Copy + Into<u128>,
    RN: Copy + Into<u128>,
    RD: Copy + Into<u128>,
>(
    lhs: &U64Ratio<LN, LD>,
    rhs: &U64Ratio<RN, RD>,
) -> Ordering {
    // zero-edge cases
    if lhs.is_zero() {
        return if rhs.is_zero() {
            Ordering::Equal
        } else {
            Ordering::Less
        };
    }
    // lhs != 0
    if rhs.is_zero() {
        return Ordering::Greater;
    }

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
    > PartialEq<U64Ratio<RN, RD>> for U64Ratio<LN, LD>
{
    fn eq(&self, rhs: &U64Ratio<RN, RD>) -> bool {
        cmp_inner(self, rhs).is_eq()
    }
}

impl<N: Copy + Into<u128>, D: Copy + Into<u128>> Eq for U64Ratio<N, D> {}

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

#[cfg(all(test, feature = "std"))]
pub(crate) mod ratio_test_utils {
    use proptest::prelude::*;

    use super::*;

    prop_compose! {
        pub fn u64_ratio_gte_one()
            (denom in any::<u64>())
            (num in denom..=u64::MAX, denom in Just(denom)) -> U64Ratio<u64, u64> {
                U64Ratio { num, denom }
            }
    }

    prop_compose! {
        pub fn u64_ratio_lte_one()
            (denom in any::<u64>())
            (num in 0..=denom, denom in Just(denom)) -> U64Ratio<u64, u64> {
                U64Ratio { num, denom }
            }
    }

    prop_compose! {
        pub fn nonzero_u64_ratio()
            (num in 1..=u64::MAX, denom in 1..=u64::MAX) -> U64Ratio<u64, u64> {
                U64Ratio { num, denom }
            }
    }

    prop_compose! {
        pub fn zero_num_u64_ratio()
            (denom in any::<u64>()) -> U64Ratio<u64, u64>
            {
                U64Ratio { num: 0, denom }
            }
    }

    prop_compose! {
        pub fn zero_denom_u64_ratio()
            (num in any::<u64>()) -> U64Ratio<u64, u64>
            {
                U64Ratio { num, denom: 0 }
            }
    }
}

#[cfg(all(test, feature = "std"))]
mod tests {
    use proptest::prelude::*;

    use super::*;

    proptest! {
        #[test]
        fn ord(common: u64, a: u64, b: u64) {
            if a == b {
                prop_assert_eq!(
                    U64Ratio { num: a, denom: common },
                    U64Ratio { num: b, denom: common }
                );
                prop_assert_eq!(
                    U64Ratio { num: common, denom: a },
                    U64Ratio { num: common, denom: b }
                );
            }
            let (smaller, larger) = if a < b {
                (a, b)
            } else {
                (b, a)
            };
            let s = U64Ratio { num: smaller, denom: common };
            let l = U64Ratio { num: larger, denom: common };
            prop_assert!(s < l);

            let s = U64Ratio { num: common, denom: larger };
            let l = U64Ratio { num: common, denom: smaller };
            prop_assert!(s < l);
        }
    }

    proptest! {
        #[test]
        fn zero_eq(a: u64, b: u64) {
            prop_assert_eq!(
                U64Ratio { num: a, denom: 0u64 },
                U64Ratio { num: b, denom: 0u64 }
            );
            prop_assert_eq!(
                U64Ratio { num: 0u64, denom: a},
                U64Ratio { num: b, denom: 0u64 }
            );
            prop_assert_eq!(
                U64Ratio { num: a, denom: 0u64 },
                U64Ratio { num: 0u64, denom: b }
            );
            prop_assert_eq!(
                U64Ratio { num: 0u64, denom: a },
                U64Ratio { num: 0u64, denom: b }
            );
        }
    }
}
