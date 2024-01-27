use crate::{utils::CheckedCeilDiv, FloorDiv, MathError, ReversibleRatio, U64Ratio, U64ValueRange};

impl<N: Copy + Into<u128>, D: Copy + Into<u128>> ReversibleRatio for FloorDiv<U64Ratio<N, D>> {
    /// Returns amt * num // denom.
    ///
    /// Returns 0 if denominator == 0
    fn apply(&self, amount: u64) -> Result<u64, MathError> {
        let U64Ratio { num, denom } = self.0;
        let d: u128 = denom.into();
        if d == 0 {
            return Ok(0);
        }
        let n: u128 = num.into();
        let x: u128 = amount.into();
        x.checked_mul(n)
            .and_then(|nx| nx.checked_div(d))
            .and_then(|res| res.try_into().ok())
            .ok_or(MathError)
    }

    /// Returns the range of possible values that were fed into `self.apply()`
    /// to get `amt_after_apply`
    ///
    /// # Returns
    /// - `U64ValueRange::full()` if denom == 0 || num == 0 and amt_after_apply == 0
    /// - min exclusive, rounds down if dy is not divisible by n. Else min inclusive.
    /// - max is always exclusive. Rounds up if d(y + 1) is not divisible by n
    ///
    /// Range outputs are capped to u64 range (saturating_add/sub)
    ///
    /// # Errors
    /// - if denom == 0 || num == 0 but amt_after_apply != 0
    ///
    /// # Derivation
    ///
    /// ```md
    /// let x = input amount we are trying to find, y = amt_after_apply, n = numerator, d = denominator
    /// y = floor(nx / d)
    /// y <= nx / d < y + 1
    ///
    /// LHS (min):
    /// dy <= nx
    /// dy / n <= x
    ///
    /// RHS (max):
    /// nx < d(y+1)
    /// x < d(y+1) / n
    /// ```
    fn reverse(&self, amt_after_apply: u64) -> Result<U64ValueRange, MathError> {
        let U64Ratio { num, denom } = self.0;
        let d: u128 = denom.into();
        let n: u128 = num.into();
        if d == 0 || n == 0 {
            if amt_after_apply == 0 {
                return Ok(U64ValueRange::FULL);
            } else {
                return Err(MathError);
            };
        }

        let y: u128 = amt_after_apply.into();

        let dy = y.checked_mul(d).ok_or(MathError)?;
        let min: u64 = dy
            .checked_div(n)
            .and_then(|min| min.try_into().ok())
            .ok_or(MathError)?;

        let d_y_plus_1 = y
            .checked_add(1)
            .and_then(|y_plus_1| y_plus_1.checked_mul(d))
            .ok_or(MathError)?;
        let max: u64 = d_y_plus_1
            .checked_ceil_div(n)
            .and_then(|max| max.try_into().ok())
            .ok_or(MathError)?;

        // min should always <= max since
        // y_plus_1 > y
        U64ValueRange::from_min_max(min, max)
    }
}

#[cfg(all(test, feature = "std"))]
mod tests {
    use proptest::prelude::*;

    use crate::test_utils::*;

    use super::*;

    /// RHS doesn't hold for these values
    #[test]
    fn problem_case_1() {
        const NUM: u64 = 3571888240897306429;
        const DENOM: u64 = 1886854692549596243;
        const RATIO: FloorDiv<U64Ratio<u64, u64>> = FloorDiv(U64Ratio {
            num: NUM,
            denom: DENOM,
        });
        // assert!(NUM > DENOM);
        const AMT_AFTER_APPLY: u64 = 239994113087062952;
        RATIO.reverse(AMT_AFTER_APPLY).unwrap();
    }

    prop_compose! {
        fn ratio_gte_one()
            (ratio in u64_ratio_gte_one()) -> FloorDiv<U64Ratio<u64, u64>> {
                FloorDiv(ratio)
            }
    }

    prop_compose! {
        fn ratio_lte_one()
            (ratio in u64_ratio_lte_one()) -> FloorDiv<U64Ratio<u64, u64>> {
                FloorDiv(ratio)
            }
    }

    prop_compose! {
        /// max_limit is the max number that ratio can be applied to without overflowing u64
        fn ratio_gte_one_and_overflow_max_limit()
            (u64ratio in ratio_gte_one()) -> (u64, FloorDiv<U64Ratio<u64, u64>>) {
                if u64ratio.0.num == 0 {
                    return (u64::MAX, u64ratio);
                }
                let max_limit = u64ratio.0.denom as u128 * u64::MAX as u128 / u64ratio.0.num as u128;
                if max_limit >= u64::MAX as u128 {
                    return (u64::MAX, u64ratio);
                }
                (max_limit.try_into().unwrap(), u64ratio)
            }
    }

    prop_compose! {
        fn ratio_gte_one_amt_no_overflow()
            ((maxlimit, u64ratio) in ratio_gte_one_and_overflow_max_limit())
            (amt in 0..=maxlimit, u64ratio in Just(u64ratio)) -> (u64, FloorDiv<U64Ratio<u64, u64>>) {
                (amt, u64ratio)
            }
    }

    prop_compose! {
        fn ratio_lte_one_reverse_no_overflow()
            ((amt_after_apply, FloorDiv(U64Ratio { num, denom })) in ratio_gte_one_amt_no_overflow()) -> (u64, FloorDiv<U64Ratio<u64, u64>>)
            {
                (amt_after_apply, FloorDiv(U64Ratio { num: denom, denom: num }))
            }
    }

    proptest! {
        #[test]
        fn ratio_gte_one_min_max_invariant((amt, ratio) in ratio_gte_one_amt_no_overflow()) {
            let applied = ratio.apply(amt).unwrap();
            let r = ratio.reverse(applied).unwrap();
            prop_assert!(r.get_min() <= r.get_max());
        }
    }

    proptest! {
        #[test]
        fn ratio_lte_one_min_max_invariant((amt_after_apply, ratio) in ratio_lte_one_reverse_no_overflow()) {
            let r = ratio.reverse(amt_after_apply).unwrap();
            prop_assert!(r.get_min() <= r.get_max());
        }
    }

    proptest! {
        #[test]
        fn ratio_gte_one_round_trip((amt, ratio) in ratio_gte_one_amt_no_overflow()) {
            let applied = ratio.apply(amt).unwrap();
            let r = ratio.reverse(applied).unwrap();
            let min = r.get_min();
            let max = r.get_max();
            prop_assert!(min <= max);
            prop_assert!(min == amt || min == amt - 1);
            prop_assert!(max == amt || max == amt + 1);
        }
    }

    proptest! {
        #[test]
        fn ratio_lte_one_round_trip(amt: u64, ratio in ratio_lte_one()) {
            let applied = ratio.apply(amt).unwrap();
            let r = ratio.reverse(applied).unwrap();
            let min = r.get_min();
            let max = r.get_max();
            // will not always be eq due to floor
            prop_assert!(min <= amt);
            prop_assert!(amt <= max);
            // but make sure they applying the ratio again yields result that differ at most by 1 in the correct direction
            let apply_min = ratio.apply(min).unwrap();
            prop_assert!(applied == apply_min || applied == apply_min + 1);
            let apply_max = ratio.apply(max).unwrap();
            prop_assert!(applied == apply_max || applied == apply_max - 1);
        }
    }

    proptest! {
        #[test]
        fn ratio_lte_one_reverse_round_trip((amt_after_apply, ratio) in ratio_lte_one_reverse_no_overflow()) {
            let r = ratio.reverse(amt_after_apply).unwrap();
            let min = r.get_min();
            let max = r.get_max();
            prop_assert!(min <= max);
            let apply_min = ratio.apply(min).unwrap();
            prop_assert!(amt_after_apply == apply_min || amt_after_apply == apply_min + 1);
            let apply_max = ratio.apply(max).unwrap();
            prop_assert!(amt_after_apply == apply_max || amt_after_apply == apply_max - 1);
        }
    }

    prop_compose! {
        fn zero_num_ratio()
            (ratio in zero_num_u64_ratio()) -> FloorDiv<U64Ratio<u64, u64>>
            {
                FloorDiv(ratio)
            }
    }

    prop_compose! {
        fn zero_denom_ratio()
            (ratio in zero_denom_u64_ratio()) -> FloorDiv<U64Ratio<u64, u64>>
            {
                FloorDiv(ratio)
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
            prop_assert_eq!(ratio.reverse(0).unwrap(), U64ValueRange::FULL);
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
            prop_assert_eq!(ratio.reverse(0).unwrap(), U64ValueRange::FULL);
        }
    }
}
