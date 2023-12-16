use std::cmp::Ordering;

use crate::MathError;

/// A ratio that is applied to a u64 token amount
/// with floor division
#[derive(Debug, Copy, Clone, Default, Hash)]
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

    /// Returns 0 if denominator == 0 || numerator == 0
    pub fn pseudo_reverse(&self, amt_after_apply: u64) -> Result<u64, MathError> {
        let d: u128 = self.denom.into();
        let n: u128 = self.num.into();
        if d == 0 || n == 0 {
            return Ok(0);
        }
        let y: u128 = amt_after_apply.into();
        let dy = y.checked_mul(d).ok_or(MathError)?;
        // n != 0, d != 0
        let q_floor: u64 = (dy / n).try_into().map_err(|_e| MathError)?;
        let r = dy % n;

        if r == 0 {
            return Ok(q_floor);
        }

        let d_plus_r = d.checked_add(r).ok_or(MathError)?;
        let q_ceil = q_floor.checked_add(1).ok_or(MathError)?;

        if d_plus_r >= n || d >= n {
            return Ok(q_ceil);
        }

        let r_plus_r = r.checked_add(r).ok_or(MathError)?;

        // d < n
        let res = if r_plus_r <= (n - d) { q_floor } else { q_ceil };
        Ok(res)
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

#[cfg(test)]
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

    proptest! {
        #[test]
        fn ratio_gte_one_round_trip((amt, ratio) in ratio_gte_one_amt_no_overflow()) {
            let applied = ratio.apply(amt).unwrap();
            let reversed = ratio.pseudo_reverse(applied).unwrap();
            prop_assert_eq!(reversed, amt);
        }
    }

    proptest! {
        #[test]
        fn ratio_lte_one_round_trip(amt: u64, ratio in ratio_lte_one()) {
            let applied = ratio.apply(amt).unwrap();
            let reversed = ratio.pseudo_reverse(applied).unwrap();
            // will not always be eq due to floor
            prop_assert!(reversed <= amt);
            // but make sure they applying the ratio again yields the same result
            let apply_on_reversed = ratio.apply(reversed).unwrap();
            prop_assert_eq!(applied, apply_on_reversed);
        }
    }

    proptest! {
        #[test]
        fn zero_denom(num: u64, denom in Just(0u64), amt: u64) {
            let ratio = U64RatioFloor { num, denom };
            prop_assert_eq!(ratio.apply(amt).unwrap(), 0);
            prop_assert_eq!(ratio.pseudo_reverse(amt).unwrap(), 0);
        }
    }

    proptest! {
        #[test]
        fn zero_num(num in Just(0u64), denom: u64, amt: u64) {
            let ratio = U64RatioFloor { num, denom };
            prop_assert_eq!(ratio.apply(amt).unwrap(), 0);
            prop_assert_eq!(ratio.pseudo_reverse(amt).unwrap(), 0);
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
