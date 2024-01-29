use crate::{utils::CheckedCeilDiv, CeilDiv, MathError, ReversibleRatio, U64Ratio, U64ValueRange};

impl<N: Copy + Into<u128>, D: Copy + Into<u128>> ReversibleRatio for CeilDiv<U64Ratio<N, D>> {
    /// Returns ceil(amt * num / denom)
    ///
    /// Returns 0 if denominator == 0
    fn apply(&self, amount: u64) -> Result<u64, MathError> {
        if self.0.is_zero() {
            return Ok(0);
        }
        let U64Ratio { num, denom } = self.0;
        let d: u128 = denom.into();
        let n: u128 = num.into();
        let x: u128 = amount.into();
        x.checked_mul(n)
            .and_then(|nx| nx.checked_ceil_div(d)) // d != 0
            .and_then(|res| res.try_into().ok())
            .ok_or(MathError)
    }

    /// Returns the range of possible values that were fed into `self.apply()`
    /// to get `amt_after_apply`
    ///
    /// # Returns
    /// - [`U64ValueRange::FULL`] if denom == 0 || num == 0 and amt_after_apply == 0
    /// - min exclusive, rounds down if dy is not divisible by n. Else min inclusive.
    /// - max is always exclusive. Rounds up if dy is not divisible by n
    /// - [`U64ValueRange::ZERO`] if amt_after_apply == 0
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
    /// y = ceil(nx / d)
    /// y-1 < nx / d <= y
    ///
    /// LHS (min):
    /// dy-d <= nx
    /// d(y-1) / n <= x
    /// (dy - d) / n <= x
    ///
    /// RHS (max):
    /// nx <= dy
    /// x < dy / n
    /// ```
    fn reverse(&self, amt_after_apply: u64) -> Result<U64ValueRange, MathError> {
        if self.0.is_zero() {
            return if amt_after_apply == 0 {
                Ok(U64ValueRange::FULL)
            } else {
                Err(MathError)
            };
        }
        // only way to get 0 after ceil div by a non-zero ratio is if input was 0.
        // early return ensures dy - d below does not overflow
        if amt_after_apply == 0 {
            return Ok(U64ValueRange::ZERO);
        }

        let U64Ratio { num, denom } = self.0;
        let d: u128 = denom.into();
        let n: u128 = num.into();
        let y: u128 = amt_after_apply.into();

        let dy = d.checked_mul(y).ok_or(MathError)?;

        let dy_minus_d = dy.checked_sub(d).ok_or(MathError)?;
        let min: u64 = dy_minus_d
            .checked_div(n)
            .and_then(|min| min.try_into().ok())
            .ok_or(MathError)?;

        let max: u64 = dy
            .checked_ceil_div(n)
            .and_then(|max| max.try_into().ok())
            .ok_or(MathError)?;

        // min should always <= max since
        // y_minus_1 < y
        U64ValueRange::try_from_min_max(min, max)
    }
}

#[cfg(all(test, feature = "std"))]
mod tests {
    use proptest::prelude::*;

    use crate::ratio_test_utils::*;

    use super::*;

    prop_compose! {
        fn ratio_gte_one()
            (ratio in u64_ratio_gte_one()) -> CeilDiv<U64Ratio<u64, u64>> {
                CeilDiv(ratio)
            }
    }

    prop_compose! {
        fn ratio_lte_one()
            (ratio in u64_ratio_lte_one()) -> CeilDiv<U64Ratio<u64, u64>> {
                CeilDiv(ratio)
            }
    }

    prop_compose! {
        /// max_limit is the max number that ratio can be applied to without overflowing u64
        fn ratio_gte_one_and_overflow_max_limit()
            (u64ratio in ratio_gte_one()) -> (u64, CeilDiv<U64Ratio<u64, u64>>) {
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
            (amt in 0..=maxlimit, u64ratio in Just(u64ratio)) -> (u64, CeilDiv<U64Ratio<u64, u64>>) {
                (amt, u64ratio)
            }
    }

    prop_compose! {
        fn ratio_lte_one_reverse_no_overflow()
            ((amt_after_apply, CeilDiv(U64Ratio { num, denom })) in ratio_gte_one_amt_no_overflow()) -> (u64, CeilDiv<U64Ratio<u64, u64>>)
            {
                (amt_after_apply, CeilDiv(U64Ratio { num: denom, denom: num }))
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
            (ratio in zero_num_u64_ratio()) -> CeilDiv<U64Ratio<u64, u64>>
            {
                CeilDiv(ratio)
            }
    }

    prop_compose! {
        fn zero_denom_ratio()
            (ratio in zero_denom_u64_ratio()) -> CeilDiv<U64Ratio<u64, u64>>
            {
                CeilDiv(ratio)
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

    proptest! {
        #[test]
        fn reverse_zero_is_zero(ratio in nonzero_u64_ratio()) {
            prop_assert_eq!(CeilDiv(ratio).reverse(0).unwrap(), U64ValueRange::ZERO);
        }
    }
}
