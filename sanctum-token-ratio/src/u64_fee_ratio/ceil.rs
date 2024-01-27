use crate::{
    AmtsAfterFee, AmtsAfterFeeBuilder, CeilDiv, FeeRatio, FeeRatioBounds, FeeRatioInv, FloorDiv,
    MathError, ReversibleFee, ReversibleRatio, U64FeeRatio, U64ValueRange,
};

impl<N: Copy + Into<u128>, D: Copy + Into<u128>> ReversibleFee for CeilDiv<U64FeeRatio<N, D>> {
    /// Returns the results of applying this fee to a token amount
    ///
    /// Returns:
    /// - no fees charged if fee_num == 0 || fee_denom == 0
    ///
    /// Errors if:
    /// - fee_num > fee_denom (fee > 100%)
    fn apply(&self, amt_before_fee: u64) -> Result<AmtsAfterFee, MathError> {
        let fee_charged = CeilDiv(self.0.to_u64_ratio()).apply(amt_before_fee)?;
        AmtsAfterFeeBuilder::new_amt_bef_fee(amt_before_fee).with_fee_charged(fee_charged)
    }

    /// Returns a possible amount that was fed into self.apply()
    ///
    /// # Returns:
    /// - `U64ValueRange::single(amt_after_fee)` if fee_num == 0 || fee_denom == 0 (zero fees)
    /// - `U64ValueRange::full()` if fee_num == fee_denom (fee = 100%) and amt_after_fee == 0
    /// - `U64ValueRange::zero()` if fee != 100% and fee != 0 and amt_after_fee = 0
    ///
    /// # Errors
    /// - if fee_num > fee_denom (fee > 100%)
    /// - if fee_num == fee_denom but amt_after_fee != 0
    ///
    /// # Derivation
    ///
    /// ```md
    /// let y = amt_after_fee, x = amt_before_fee, n = fee_numerator, d = fee_denominator
    ///
    /// y = x - ceil(nx/d)
    /// ceil(nx/d) = x - y
    /// x - y - 1 < nx/d <= x - y
    ///
    /// LHS: x(1 - n/d) < y + 1
    /// RHS: y <= x(1 - n/d)
    /// y <= x(1 - n/d) < y + 1
    /// y <= x[(d - n)/d] < y + 1
    ///
    /// This is the same as reversing FloorDiv<U64Ratio> with n = d - n instead.
    /// ```
    fn reverse_from_amt_after_fee(&self, amt_after_fee: u64) -> Result<U64ValueRange, MathError> {
        let Self(fee) = self;
        if fee.is_zero() {
            Ok(U64ValueRange::single(amt_after_fee))
        } else {
            FloorDiv(fee.one_minus_fee_ratio()?).reverse(amt_after_fee)
        }
    }

    /// Returns a possible amount that was fed into self.apply().
    ///
    /// # Returns:
    /// - `U64ValueRange::full()` if zero fee and fee_charged == 0
    /// - `fee_charged` if fee_num == fee_denom (fee == 100%)
    ///
    /// # Errors:
    /// - if fee_num > fee_denom (fee > 100%)
    /// - if zero fee but fee_charged != 0
    ///
    /// # Derivation:
    ///
    /// ```md
    /// let y = fee_charged, x = amt_before_fee, n = fee_numerator, d = fee_denominator
    ///
    /// y = ceil(nx/d)
    ///
    /// This is the same as reversing CeilDiv<U64Ratio>
    /// ```
    fn reverse_from_fee_charged(&self, fee_charged: u64) -> Result<U64ValueRange, MathError> {
        let Self(fee) = self;
        if fee.is_max() {
            Ok(U64ValueRange::single(fee_charged))
        } else {
            CeilDiv(fee.to_u64_ratio()).reverse(fee_charged)
        }
    }
}

#[cfg(all(test, feature = "std"))]
mod tests {
    use proptest::prelude::*;

    use crate::fee_ratio_test_utils::*;

    use super::*;

    prop_compose! {
        fn valid_fees()
            (fee in valid_fee_ratio()) -> CeilDiv<U64FeeRatio<u64, u64>> {
                CeilDiv(fee)
            }
    }

    prop_compose! {
        fn valid_nonzero_fees()
            (fee in valid_nonzero_fee_ratio()) -> CeilDiv<U64FeeRatio<u64, u64>> {
                CeilDiv(fee)
            }
    }

    prop_compose! {
        fn valid_nonmax_fees()
            (fee in valid_nonzero_fee_ratio()) -> CeilDiv<U64FeeRatio<u64, u64>> {
                CeilDiv(fee)
            }
    }

    prop_compose! {
        fn valid_max_fees()
            (fee in valid_max_fee_ratio()) -> CeilDiv<U64FeeRatio<u64, u64>> {
                CeilDiv(fee)
            }
    }

    prop_compose! {
        fn valid_zero_fees()
            (fee in valid_zero_fee_ratio()) -> CeilDiv<U64FeeRatio<u64, u64>> {
                CeilDiv(fee)
            }
    }

    // basic

    proptest! {
        #[test]
        fn fee_invariants(amt: u64, fee in valid_fees()) {
            let a = fee.apply(amt).unwrap();
            prop_assert!(a.amt_after_fee() <= amt);
            prop_assert_eq!(amt, a.amt_after_fee() + a.fee_charged());
        }
    }

    proptest! {
        #[test]
        fn zero_fee_apply_no_op(amt: u64, zero_fee in valid_zero_fees()) {
            let expected = AmtsAfterFeeBuilder::new_amt_bef_fee(amt).with_fee_charged(0).unwrap();
            prop_assert_eq!(zero_fee.apply(amt).unwrap(), expected);
        }
    }

    proptest! {
        #[test]
        fn max_fee_apply_zero(amt: u64, fee in valid_max_fees()) {
            let expected = AmtsAfterFeeBuilder::new_amt_bef_fee(amt).with_amt_aft_fee(0).unwrap();
            prop_assert_eq!(fee.apply(amt).unwrap(), expected);
        }
    }

    // reverse_from_amt_after_fee()

    proptest! {
        #[test]
        fn amt_after_fee_round_trip(amt: u64, fee in valid_nonmax_fees()) {
            let amt_after_fee = fee.apply(amt).unwrap().amt_after_fee();

            let r = fee.reverse_from_amt_after_fee(amt_after_fee).unwrap();
            let min = r.get_min();
            let max = r.get_max();

            // cannot guarantee reversed == amt or fee_charged == apply_on_reversed.fee_charged
            let min_amt_after_fee = fee.apply(min).unwrap().amt_after_fee();
            prop_assert!(
                amt_after_fee == min_amt_after_fee || amt_after_fee - 1 == min_amt_after_fee,
                "{amt_after_fee}, {min_amt_after_fee}"
            );
            let max_amt_after_fee = fee.apply(max).unwrap().amt_after_fee();
            prop_assert!(
                amt_after_fee == max_amt_after_fee || amt_after_fee + 1 == max_amt_after_fee,
                "{amt_after_fee}, {max_amt_after_fee}"
            );
        }
    }

    proptest! {
        #[test]
        fn zero_fee_amt_after_fee_reverse_no_op(amt_after_fee: u64, zero_fee in valid_zero_fees()) {
            prop_assert_eq!(zero_fee.reverse_from_amt_after_fee(amt_after_fee).unwrap(), U64ValueRange::single(amt_after_fee));
        }
    }

    proptest! {
        #[test]
        fn max_fee_nonzero_amt_after_fee_reverse_err(non_zero_amt_after_fee in 1..=u64::MAX, fee in valid_max_fees()) {
            prop_assert_eq!(fee.reverse_from_amt_after_fee(non_zero_amt_after_fee).unwrap_err(), MathError);
        }
    }

    proptest! {
        #[test]
        fn max_fee_zero_amt_after_fee_reverse_range_full(fee in valid_max_fees()) {
            prop_assert_eq!(fee.reverse_from_amt_after_fee(0).unwrap(), U64ValueRange::FULL);
        }
    }

    // reverse_from_fee_charged()

    proptest! {
        #[test]
        fn fee_charged_round_trip(amt: u64, fee in valid_nonzero_fees()) {
            let a = fee.apply(amt).unwrap();
            let fee_charged = a.fee_charged();
            let amt_after_fee = a.amt_after_fee();

            let r = fee.reverse_from_fee_charged(fee_charged).unwrap();
            let min = r.get_min();
            let max = r.get_max();

            // cannot guarantee reversed == amt or amt_after_fee == apply_on_reversed.amt_after_fee
            // but check that the fee charged differs from the original value by at most 1 in the correct direction

            let apply_min = fee.apply(min).unwrap();
            let min_amt_after_fee = apply_min.amt_after_fee();
            let min_fee_charged = apply_min.fee_charged();
            prop_assert!(amt_after_fee >= min_amt_after_fee);
            prop_assert!(
                fee_charged == min_fee_charged || fee_charged == min_fee_charged + 1,
                "{fee_charged}, {min_fee_charged}"
            );
            let apply_max = fee.apply(max).unwrap();
            let max_amt_after_fee = apply_max.amt_after_fee();
            let max_fee_charged = apply_max.fee_charged();
            prop_assert!(amt_after_fee <= max_amt_after_fee);
            prop_assert!(
                fee_charged == max_fee_charged || fee_charged == max_fee_charged - 1,
                "{fee_charged}, {max_fee_charged}"
            );
        }
    }

    proptest! {
        #[test]
        fn zero_fee_nonzero_fee_charged_reverse_err(nonzero_fee_charged in 1..=u64::MAX, zero_fee in valid_zero_fees()) {
            prop_assert_eq!(zero_fee.reverse_from_fee_charged(nonzero_fee_charged).unwrap_err(), MathError);
        }
    }

    proptest! {
        #[test]
        fn zero_fee_zero_fee_charged_reverse_range_full(zero_fee in valid_zero_fees()) {
            prop_assert_eq!(zero_fee.reverse_from_fee_charged(0).unwrap(), U64ValueRange::FULL);
        }
    }

    proptest! {
        #[test]
        fn max_fee_fee_charged_reverse_no_op(fee_charged: u64, fee in valid_max_fees()) {
            prop_assert_eq!(fee.reverse_from_fee_charged(fee_charged).unwrap(), U64ValueRange::single(fee_charged));
        }
    }
}
