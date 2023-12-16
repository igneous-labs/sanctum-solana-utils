use crate::{AmtsAfterFee, MathError, U64RatioFloor};

/// A fee ratio applied to a token amount. Should be <= 1.0.
///
/// `amt_after_fee = amt * (fee_denom - fee_num) // fee_denom`
///
/// `fee_charged = amt - amt_after_fee`
///
/// Effectively maximizes fees charged
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub struct U64FeeCeil<N: Copy + Into<u128>, D: Copy + Into<u128>> {
    pub fee_num: N,
    pub fee_denom: D,
}

impl<N: Copy + Into<u128>, D: Copy + Into<u128>> U64FeeCeil<N, D> {
    /// The U64RatioFloor to apply to amount to get amount_after_fee
    fn amt_after_fee_ratio_floor(&self) -> Result<U64RatioFloor<u128, u128>, MathError> {
        let n: u128 = self.fee_num.into();
        let d: u128 = self.fee_denom.into();
        let num = d.checked_sub(n).ok_or(MathError)?;
        Ok(U64RatioFloor { num, denom: d })
    }

    /// Returns the results of applying this fee to a token amount
    ///
    /// Returns:
    /// - no fees charged if fee_num == 0 || fee_denom == 0
    ///
    /// Errors if:
    /// - fee_num > fee_denom (fee > 100%)
    pub fn apply(&self, amt: u64) -> Result<AmtsAfterFee, MathError> {
        let n: u128 = self.fee_num.into();
        let d: u128 = self.fee_denom.into();
        if n == 0 || d == 0 {
            return Ok(AmtsAfterFee {
                amt_after_fee: amt,
                fee_charged: 0,
            });
        }
        let ratio_floor = self.amt_after_fee_ratio_floor()?;
        let amt_after_fee = ratio_floor.apply(amt)?;
        let fee_charged = amt.checked_sub(amt_after_fee).ok_or(MathError)?;
        Ok(AmtsAfterFee {
            amt_after_fee,
            fee_charged,
        })
    }

    /// Returns a possible amount that was fed into self.apply()
    ///
    /// Returns:
    /// - `amt_after_fee` if fee_num == 0 || fee_denom == 0 (zero fees)
    ///
    /// Errors if:
    /// - fee_num > fee_denom (fee > 100%)
    /// - fee_num == fee_denom (fee = 100%): infinite possibilities if fee = 100%
    pub fn pseudo_reverse_from_amt_after_fee(&self, amt_after_fee: u64) -> Result<u64, MathError> {
        let n: u128 = self.fee_num.into();
        let d: u128 = self.fee_denom.into();
        if n == 0 || d == 0 {
            return Ok(amt_after_fee);
        }
        if n >= d {
            return Err(MathError);
        }
        let ratio_floor = self.amt_after_fee_ratio_floor()?;
        ratio_floor.pseudo_reverse(amt_after_fee)
    }

    /// Returns a possible amount that was fed into self.apply().
    ///
    /// Returns `fee_charged` if fee_num == fee_denom (fee == 100%)
    ///
    /// Errors if:
    /// - fee_num > fee_denom (fee > 100%)
    /// - fee_num == 0 || fee_denom == 0: can't compute amt_before_fee if no fees charged
    pub fn pseudo_reverse_from_fee_charged(&self, fee_charged: u64) -> Result<u64, MathError> {
        let n: u128 = self.fee_num.into();
        let d: u128 = self.fee_denom.into();
        if n > d || n == 0 || d == 0 {
            return Err(MathError);
        }
        if n == d {
            return Ok(fee_charged);
        }
        // x = floor(df/n)
        U64RatioFloor { num: d, denom: n }.apply(fee_charged)
    }

    pub fn is_valid(&self) -> bool {
        self.fee_num.into() <= self.fee_denom.into()
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::*;

    prop_compose! {
        fn invalid_fees()
            (fee_num in any::<u64>())
            (fee_denom in 1..fee_num, fee_num in Just(fee_num)) -> U64FeeCeil<u64, u64> {
                U64FeeCeil { fee_num, fee_denom }
            }
    }

    prop_compose! {
        fn valid_fees()
            (fee_denom in any::<u64>())
            (fee_num in 0..=fee_denom, fee_denom in Just(fee_denom)) -> U64FeeCeil<u64, u64> {
                U64FeeCeil { fee_num, fee_denom }
            }
    }

    prop_compose! {
        fn valid_nonzero_fees()
            (fee_denom in any::<u64>())
            (fee_num in 1..=fee_denom, fee_denom in Just(fee_denom)) -> U64FeeCeil<u64, u64> {
                U64FeeCeil { fee_num, fee_denom }
            }
    }

    prop_compose! {
        fn valid_nonmax_fees()
            (fee_denom in any::<u64>())
            (fee_num in 0..fee_denom, fee_denom in Just(fee_denom)) -> U64FeeCeil<u64, u64> {
                U64FeeCeil { fee_num, fee_denom }
            }
    }

    prop_compose! {
        fn valid_max_fees()
            (n in 1..=u64::MAX) -> U64FeeCeil<u64, u64> {
                U64FeeCeil { fee_num: n, fee_denom: n }
            }
    }

    prop_compose! {
        fn valid_zero_denom_fees()
            (fee_num in 0..=u64::MAX, fee_denom in Just(0)) -> U64FeeCeil<u64, u64> {
                U64FeeCeil { fee_num, fee_denom }
            }
    }

    prop_compose! {
        fn valid_zero_num_fees()
            (fee_num in Just(0), fee_denom in 0..=u64::MAX) -> U64FeeCeil<u64, u64> {
                U64FeeCeil { fee_num, fee_denom }
            }
    }

    prop_compose! {
        fn valid_zero_fees()
            (zero_num in valid_zero_num_fees(), zero_denom in valid_zero_denom_fees()) -> [U64FeeCeil<u64, u64>; 2] {
                [zero_num, zero_denom]
            }
    }

    // basic

    proptest! {
        #[test]
        fn fee_invariants(amt: u64, fee in valid_fees()) {
            let AmtsAfterFee { amt_after_fee, fee_charged } = fee.apply(amt).unwrap();
            prop_assert!(amt_after_fee <= amt);
            prop_assert_eq!(amt, amt_after_fee + fee_charged);
        }
    }

    proptest! {
        #[test]
        fn zero_fee_apply_no_op(amt: u64, zero_fees in valid_zero_fees()) {
            for fee in zero_fees {
                let AmtsAfterFee { amt_after_fee, fee_charged } = fee.apply(amt).unwrap();
                prop_assert_eq!(amt_after_fee, amt);
                prop_assert_eq!(fee_charged, 0);
            }
        }
    }

    proptest! {
        #[test]
        fn max_fee_apply_zero(amt: u64, fee in valid_max_fees()) {
            prop_assert_eq!(fee.apply(amt).unwrap(), AmtsAfterFee { amt_after_fee: 0, fee_charged: amt });
        }
    }

    proptest! {
        #[test]
        fn invalid_fee_apply_err(amt: u64, fee in invalid_fees()) {
            prop_assert_eq!(fee.apply(amt).unwrap_err(), MathError);
        }
    }

    // pseudo_reverse_from_amt_after_fee()

    proptest! {
        #[test]
        fn amt_after_fee_round_trip(amt: u64, fee in valid_nonmax_fees()) {
            let AmtsAfterFee { amt_after_fee, .. } = fee.apply(amt).unwrap();

            let reversed = fee.pseudo_reverse_from_amt_after_fee(amt_after_fee).unwrap();
            let apply_on_reversed = fee.apply(reversed).unwrap();

            // cannot guarantee reversed == amt or fee_charged == apply_on_reversed.fee_charged
            prop_assert_eq!(amt_after_fee, apply_on_reversed.amt_after_fee);
        }
    }

    proptest! {
        #[test]
        fn zero_fee_amt_after_fee_reverse_no_op(amt_after_fee: u64, zero_fees in valid_zero_fees()) {
            for zero_fee in zero_fees {
                let amt = zero_fee.pseudo_reverse_from_amt_after_fee(amt_after_fee).unwrap();
                prop_assert_eq!(amt_after_fee, amt);
            }
        }
    }

    proptest! {
        #[test]
        fn max_fee_amt_after_fee_reverse_err(amt_after_fee: u64, fee in valid_max_fees()) {
            prop_assert_eq!(fee.pseudo_reverse_from_amt_after_fee(amt_after_fee).unwrap_err(), MathError);
        }
    }

    proptest! {
        #[test]
        fn invalid_fee_amt_after_fee_reverse_err(amt_after_fee: u64, fee in invalid_fees()) {
            prop_assert_eq!(fee.pseudo_reverse_from_amt_after_fee(amt_after_fee).unwrap_err(), MathError);
        }
    }

    // pseudo_reverse_from_fee_charged()

    proptest! {
        #[test]
        fn fee_charged_round_trip(amt: u64, fee in valid_nonzero_fees()) {
            let AmtsAfterFee { fee_charged, .. } = fee.apply(amt).unwrap();

            let reversed = fee.pseudo_reverse_from_fee_charged(fee_charged).unwrap();
            let apply_on_reversed = fee.apply(reversed).unwrap();

            // cannot guarantee reversed == amt or amt_after_fee == apply_on_reversed.amt_after_fee
            prop_assert_eq!(fee_charged, apply_on_reversed.fee_charged);
        }
    }

    proptest! {
        #[test]
        fn zero_fee_fee_charged_reverse_err(fee_charged: u64, zero_fees in valid_zero_fees()) {
            for zero_fee in zero_fees {
                prop_assert_eq!(zero_fee.pseudo_reverse_from_fee_charged(fee_charged).unwrap_err(), MathError);
            }
        }
    }

    proptest! {
        #[test]
        fn max_fee_fee_charged_reverse_no_op(fee_charged: u64, fee in valid_max_fees()) {
            let amt = fee.pseudo_reverse_from_fee_charged(fee_charged).unwrap();
            prop_assert_eq!(fee_charged, amt);
        }
    }

    proptest! {
        #[test]
        fn invalid_fee_fee_charged_reverse_err(fee_charged: u64, fee in invalid_fees()) {
            prop_assert_eq!(fee.pseudo_reverse_from_fee_charged(fee_charged).unwrap_err(), MathError);
        }
    }

    // is_valid()

    proptest! {
        #[test]
        fn correct_valid_conditions(valid in valid_fees()) {
            prop_assert!(valid.is_valid());
        }
    }

    proptest! {
        #[test]
        fn correct_invalid_conditions(invalid in invalid_fees()) {
            prop_assert!(!invalid.is_valid());
        }
    }
}
