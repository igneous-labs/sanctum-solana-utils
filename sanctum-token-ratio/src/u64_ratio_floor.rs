use core::cmp::Ordering;

use crate::{MathError, U64ValueRange};

/// A ratio that is applied to a u64 token amount
/// with floor division
#[derive(Debug, Copy, Clone, Default, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
pub struct U64RatioFloor<N, D> {
    pub num: N,
    pub denom: D,
}

impl<N: Copy + Into<u128>, D: Copy + Into<u128>> U64RatioFloor<N, D> {
    /// Returns amt * num // denom
    /// Returns 0 if denominator == 0
    pub fn apply(&self, amt: u64) -> Result<u64, MathError> {
        let d: u128 = self.denom.into();
        if d == 0 {
            return Ok(0);
        }
        let n: u128 = self.num.into();
        let x: u128 = amt.into();
        x.checked_mul(n)
            .map(|nx| nx / d) // d != 0
            .and_then(|res| res.try_into().ok())
            .ok_or(MathError)
    }

    /// Returns the range of possible values that were fed into `self.apply()`
    /// to get `amt_after_apply`
    ///
    /// Returns:
    /// - `U64ValueRange::full()` if denom == 0 || num == 0 and amt_after_apply == 0
    ///
    /// Range outputs are capped to u64 range (saturating_add/sub)
    ///
    /// Errors:
    /// - if denom == 0 || num == 0 but amt_after_apply != 0
    pub fn reverse(&self, amt_after_apply: u64) -> Result<U64ValueRange, MathError> {
        let d: u128 = self.denom.into();
        let n: u128 = self.num.into();
        let is_zero = d == 0 || n == 0;
        if is_zero {
            if amt_after_apply == 0 {
                return Ok(U64ValueRange::full());
            } else {
                return Err(MathError);
            };
        }

        let y: u128 = amt_after_apply.into();
        let y_plus_1 = y.checked_add(1).ok_or(MathError)?;
        // n != 0 here, safe to do unchecked division

        let dy = y.checked_mul(d).ok_or(MathError)?;
        let mut min: u64 = (dy / n).try_into().map_err(|_e| MathError)?;
        let min_rem = dy % n;
        if min_rem != 0 {
            min = min.saturating_add(1);
        }

        let dy_plus_1 = y_plus_1.checked_mul(d).ok_or(MathError)?;
        let mut max: u64 = (dy_plus_1 / n).try_into().map_err(|_e| MathError)?;
        let max_rem = dy_plus_1 % n;
        if max_rem == 0 {
            max = max.saturating_sub(1);
        }

        // (dy%n + d) < n, RHS doesn't hold
        if min > max {
            return Err(MathError);
        }

        Ok(U64ValueRange { min, max })
    }
}

impl<
        LN: Copy + Into<u128>,
        LD: Copy + Into<u128>,
        RN: Copy + Into<u128>,
        RD: Copy + Into<u128>,
    > PartialEq<U64RatioFloor<RN, RD>> for U64RatioFloor<LN, LD>
{
    fn eq(&self, rhs: &U64RatioFloor<RN, RD>) -> bool {
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

impl<N: Copy + Into<u128>, D: Copy + Into<u128>> Eq for U64RatioFloor<N, D> {}

impl<
        LN: Copy + Into<u128>,
        LD: Copy + Into<u128>,
        RN: Copy + Into<u128>,
        RD: Copy + Into<u128>,
    > PartialOrd<U64RatioFloor<RN, RD>> for U64RatioFloor<LN, LD>
{
    fn partial_cmp(&self, rhs: &U64RatioFloor<RN, RD>) -> Option<Ordering> {
        let ln: u128 = self.num.into();
        let ld: u128 = self.denom.into();
        let rn: u128 = rhs.num.into();
        let rd: u128 = rhs.denom.into();

        // panic on overflow, even if overflow checks off
        let lhs = ln.checked_mul(rd).unwrap();
        let rhs = rn.checked_mul(ld).unwrap();

        Some(lhs.cmp(&rhs))
    }
}

impl<N: Copy + Into<u128>, D: Copy + Into<u128>> Ord for U64RatioFloor<N, D> {
    fn cmp(&self, rhs: &Self) -> Ordering {
        self.partial_cmp(rhs).unwrap()
    }
}

#[cfg(all(test, feature = "std"))]
mod tests {
    use proptest::prelude::*;

    use super::*;

    prop_compose! {
        fn ratio_gte_one()
            (denom in any::<u64>())
            (num in denom..=u64::MAX, denom in Just(denom)) -> U64RatioFloor<u64, u64> {
                U64RatioFloor { num, denom }
            }
    }

    prop_compose! {
        fn ratio_lte_one()
            (denom in any::<u64>())
            (num in 0..=denom, denom in Just(denom)) -> U64RatioFloor<u64, u64> {
                U64RatioFloor { num, denom }
            }
    }

    prop_compose! {
        /// max_limit is the max number that ratio can be applied to without overflowing u64
        fn ratio_gte_one_and_overflow_max_limit()
            (u64ratio in ratio_gte_one()) -> (u64, U64RatioFloor<u64, u64>) {
                if u64ratio.num == 0 {
                    return (u64::MAX, u64ratio);
                }
                let max_limit = u64ratio.denom as u128 * u64::MAX as u128 / u64ratio.num as u128;
                if max_limit >= u64::MAX as u128 {
                    return (u64::MAX, u64ratio);
                }
                (max_limit.try_into().unwrap(), u64ratio)
            }
    }

    prop_compose! {
        fn ratio_gte_one_amt_no_overflow()
            ((maxlimit, u64ratio) in ratio_gte_one_and_overflow_max_limit())
            (amt in 0..=maxlimit, u64ratio in Just(u64ratio)) -> (u64, U64RatioFloor<u64, u64>) {
                (amt, u64ratio)
            }
    }

    prop_compose! {
        fn ratio_lte_one_reverse_no_overflow()
            ((amt_after_apply, U64RatioFloor { num, denom }) in ratio_gte_one_amt_no_overflow()) -> (u64, U64RatioFloor<u64, u64>)
            {
                (amt_after_apply, U64RatioFloor { num: denom, denom: num })
            }
    }

    prop_compose! {
        fn zero_num_ratio()
            (denom in any::<u64>()) -> U64RatioFloor<u64, u64>
            {
                U64RatioFloor { num: 0, denom }
            }
    }

    prop_compose! {
        fn zero_denom_ratio()
            (num in any::<u64>()) -> U64RatioFloor<u64, u64>
            {
                U64RatioFloor { num, denom: 0 }
            }
    }

    proptest! {
        #[test]
        fn ratio_gte_one_round_trip((amt, ratio) in ratio_gte_one_amt_no_overflow()) {
            let applied = ratio.apply(amt).unwrap();
            let U64ValueRange { min, max } = ratio.reverse(applied).unwrap();
            prop_assert_eq!(min, max);
            prop_assert_eq!(min, amt);
        }
    }

    proptest! {
        #[test]
        fn ratio_lte_one_round_trip(amt: u64, ratio in ratio_lte_one()) {
            let applied = ratio.apply(amt).unwrap();
            let U64ValueRange { min, max } = ratio.reverse(applied).unwrap();
            // will not always be eq due to floor
            prop_assert!(min <= amt);
            prop_assert!(amt <= max);
            // but make sure they applying the ratio again yields the same result
            prop_assert_eq!(applied, ratio.apply(min).unwrap());
            prop_assert_eq!(applied, ratio.apply(max).unwrap());
        }
    }

    proptest! {
        #[test]
        fn ratio_lte_one_reverse_round_trip((amt_after_apply, ratio) in ratio_lte_one_reverse_no_overflow()) {
            let U64ValueRange { min, max } = ratio.reverse(amt_after_apply).unwrap();
            prop_assert!(min <= max);
            prop_assert_eq!(amt_after_apply, ratio.apply(min).unwrap());
            prop_assert_eq!(amt_after_apply, ratio.apply(max).unwrap());
        }
    }

    proptest! {
        #[test]
        fn zero_denom_result_zero(amt: u64, ratio in zero_denom_ratio()) {
            prop_assert_eq!(ratio.apply(amt).unwrap(), 0);
        }
    }

    proptest! {
        #[test]
        fn zero_denom_non_zero_amt_after_apply_reverse_err(amt_after_apply in 1..=u64::MAX, ratio in zero_denom_ratio()) {
            prop_assert_eq!(ratio.reverse(amt_after_apply).unwrap_err(), MathError);
        }
    }

    proptest! {
        #[test]
        fn zero_denom_zero_amt_after_apply_reverse_full_range(ratio in zero_denom_ratio()) {
            prop_assert_eq!(ratio.reverse(0).unwrap(), U64ValueRange::full());
        }
    }

    proptest! {
        #[test]
        fn zero_num_result_zero(amt: u64, ratio in zero_num_ratio()) {
            prop_assert_eq!(ratio.apply(amt).unwrap(), 0);
        }
    }

    proptest! {
        #[test]
        fn zero_num_non_zero_amt_after_apply_reverse_err(amt_after_apply in 1..=u64::MAX, ratio in zero_num_ratio()) {
            prop_assert_eq!(ratio.reverse(amt_after_apply).unwrap_err(), MathError);
        }
    }

    proptest! {
        #[test]
        fn zero_num_zero_amt_after_apply_reverse_full_range(ratio in zero_num_ratio()) {
            prop_assert_eq!(ratio.reverse(0).unwrap(), U64ValueRange::full());
        }
    }

    proptest! {
        #[test]
        fn ord(common: u64, a: u64, b: u64) {
            if a == b {
                prop_assert_eq!(
                    U64RatioFloor { num: a, denom: common },
                    U64RatioFloor { num: b, denom: common }
                );
                prop_assert_eq!(
                    U64RatioFloor { num: common, denom: a },
                    U64RatioFloor { num: common, denom: b }
                );
            }
            let (smaller, larger) = if a < b {
                (a, b)
            } else {
                (b, a)
            };
            let s = U64RatioFloor { num: smaller, denom: common };
            let l = U64RatioFloor { num: larger, denom: common };
            prop_assert!(s < l);

            let s = U64RatioFloor { num: common, denom: larger };
            let l = U64RatioFloor { num: common, denom: smaller };
            prop_assert!(s < l);
        }
    }
}
