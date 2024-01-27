use crate::{utils::CheckedCeilDiv, CeilDiv, MathError, ReversibleRatio, U64Ratio, U64ValueRange};

impl<N: Copy + Into<u128>, D: Copy + Into<u128>> ReversibleRatio for CeilDiv<U64Ratio<N, D>> {
    /// Returns ceil(amt * num / denom)
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
            .and_then(|nx| nx.checked_ceil_div(d)) // d != 0
            .and_then(|res| res.try_into().ok())
            .ok_or(MathError)
    }

    /// Returns the range of possible values that were fed into `self.apply()`
    /// to get `amt_after_apply`
    ///
    /// # Returns
    /// - `U64ValueRange::full()` if denom == 0 || num == 0 and amt_after_apply == 0
    /// - min exclusive, rounds down if dy is not divisible by n. Else min inclusive.
    /// - max is always exclusive. Rounds up if dy is not divisible by n
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
    ///
    /// RHS (max):
    /// nx <= dy
    /// x < dy / n
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
        // if ratio > 0, then
        // only way ceil div results in 0 is if amt_before_apply == 0
        if amt_after_apply == 0 {
            return Ok(U64ValueRange::ZERO);
        }

        let y: u128 = amt_after_apply.into();

        let d_y_minus_1 = y
            .checked_sub(1)
            .and_then(|y_minus_1| y_minus_1.checked_mul(d))
            .ok_or(MathError)?;
        let min: u64 = d_y_minus_1
            .checked_div(n)
            .and_then(|min| min.try_into().ok())
            .ok_or(MathError)?;

        let max: u64 = y
            .checked_mul(d)
            .and_then(|dy| dy.checked_ceil_div(n))
            .and_then(|max| max.try_into().ok())
            .ok_or(MathError)?;

        // min should always <= max since
        // y_minus_1 < y
        U64ValueRange::from_min_max(min, max)
    }
}

#[cfg(all(test, feature = "std"))]
mod tests {
    use proptest::prelude::*;

    use crate::test_utils::*;

    use super::*;

    // TODO: Add the other tests adapted from floor

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
}
