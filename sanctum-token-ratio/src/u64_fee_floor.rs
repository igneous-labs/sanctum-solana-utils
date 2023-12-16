use crate::{AmtsAfterFee, MathError, U64RatioFloor};

/// A fee ratio that should be <= 1.0.
/// fee_amt = floor(amt * fee_num / fee_denom)
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub struct U64FeeFloor<N: Copy + Into<u128>, D: Copy + Into<u128>> {
    pub fee_num: N,
    pub fee_denom: D,
}

impl<N: Copy + Into<u128>, D: Copy + Into<u128>> U64FeeFloor<N, D> {
    pub const fn fee_ratio_floor(&self) -> U64RatioFloor<N, D> {
        U64RatioFloor {
            num: self.fee_num,
            denom: self.fee_denom,
        }
    }

    /// Returns no fees charged if fee_num == 0 || fee_denom == 0
    ///
    /// Errors if:
    /// - fee_num > fee_denom (fee > 100%)
    pub fn apply(&self, amt: u64) -> Result<AmtsAfterFee, MathError> {
        let fees_charged = self.fee_ratio_floor().apply(amt)?;
        let amt_after_fee = amt.checked_sub(fees_charged).ok_or(MathError)?;
        Ok(AmtsAfterFee {
            amt_after_fee,
            fees_charged,
        })
    }

    /// Returns a possible amount that was fed into self.apply()
    ///
    /// Returns `amt_after_apply` if fee_num == 0 || fee_denom == 0
    ///
    /// Errors if:
    /// - fee_num >= fee_denom (fee >= 100%)
    pub fn pseudo_reverse_from_amt_after_fee(&self, amt_after_fee: u64) -> Result<u64, MathError> {
        let n = self.fee_num.into();
        let d = self.fee_denom.into();
        if n == 0 || d == 0 {
            return Ok(amt_after_fee);
        }
        if n >= d {
            return Err(MathError);
        }
        let y: u128 = amt_after_fee.into();
        let dy = y.checked_mul(d).ok_or(MathError)?;
        let d_minus_n = d - n; // n < d checked above
        let q_floor = dy / d_minus_n; // d_minus_n != 0 since d != n

        q_floor.try_into().map_err(|_e| MathError)
    }

    /// Returns a possible amount that was fed into self.apply()
    ///
    /// Errors if:
    /// - fee_num >= fee_denom (fee >= 100%)
    /// - fee_num == 0 || fee_denom == 0: can't compute amt_before_fee if no fees charged
    pub fn pseudo_reverse_from_fees_charged(&self, fees_charged: u64) -> Result<u64, MathError> {
        let n = self.fee_num.into();
        let d = self.fee_denom.into();
        if n >= d || n == 0 || d == 0 {
            return Err(MathError);
        }
        self.fee_ratio_floor().pseudo_reverse(fees_charged)
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
        fn valid_u64_fees()
            (fee_denom in any::<u64>())
            (fee_num in 0..=fee_denom, fee_denom in Just(fee_denom)) -> U64FeeFloor<u64, u64> {
                U64FeeFloor { fee_num, fee_denom }
            }
    }

    prop_compose! {
        fn zero_denom_u64_fees()
            (fee_num in 0..=u64::MAX, fee_denom in Just(0)) -> U64FeeFloor<u64, u64> {
                U64FeeFloor { fee_num, fee_denom }
            }
    }

    prop_compose! {
        fn zero_num_u64_fees()
            (fee_num in Just(0), fee_denom in 0..=u64::MAX) -> U64FeeFloor<u64, u64> {
                U64FeeFloor { fee_num, fee_denom }
            }
    }

    prop_compose! {
        fn zero_u64_fees()
            (zero_num in zero_num_u64_fees(), zero_denom in zero_denom_u64_fees()) -> [U64FeeFloor<u64, u64>; 2] {
                [zero_num, zero_denom]
            }
    }

    proptest! {
        #[test]
        fn u64_fee_invariants(amt: u64, fee in valid_u64_fees()) {
            let AmtsAfterFee { amt_after_fee, fees_charged } = fee.apply(amt).unwrap();
            prop_assert!(amt_after_fee <= amt);
            prop_assert_eq!(amt, amt_after_fee + fees_charged);
        }
    }

    proptest! {
        #[test]
        fn u64_fee_round_trip_amt_after_fee(amt: u64, fee in valid_u64_fees()) {
            let AmtsAfterFee { amt_after_fee, .. } = fee.apply(amt).unwrap();

            let reversed = fee.pseudo_reverse_from_amt_after_fee(amt_after_fee).unwrap();
            let apply_on_reversed = fee.apply(reversed).unwrap();

            // cannot guarantee reversed == amt or fees_charged == apply_on_reversed.fees_charged
            prop_assert_eq!(amt_after_fee, apply_on_reversed.amt_after_fee);
        }
    }

    proptest! {
        #[test]
        fn u64_fee_zero_no_op(amt: u64, zero_fees in zero_u64_fees()) {
            for fee in zero_fees {
                let AmtsAfterFee { amt_after_fee, fees_charged } = fee.apply(amt).unwrap();
                prop_assert_eq!(amt_after_fee, amt);
                prop_assert_eq!(fees_charged, 0);
            }
        }
    }

    proptest! {
        #[test]
        fn u64_fee_zero_amt_after_fee_reverse_no_op(amt_after_fee: u64, zero_fees in zero_u64_fees()) {
            for zero_fee in zero_fees {
                let amt = zero_fee.pseudo_reverse_from_amt_after_fee(amt_after_fee).unwrap();
                prop_assert_eq!(amt_after_fee, amt);
            }
        }
    }

    prop_compose! {
        fn valid_nonzero_u64_fees()
            (fee_denom in any::<u64>())
            (fee_num in 1..=fee_denom, fee_denom in Just(fee_denom)) -> U64FeeFloor<u64, u64> {
                U64FeeFloor { fee_num, fee_denom }
            }
    }

    proptest! {
        #[test]
        fn u64_fee_round_trip_fees_charged(amt: u64, fee in valid_nonzero_u64_fees()) {
            let AmtsAfterFee { fees_charged, .. } = fee.apply(amt).unwrap();

            let reversed = fee.pseudo_reverse_from_fees_charged(fees_charged).unwrap();
            let apply_on_reversed = fee.apply(reversed).unwrap();

            // cannot guarantee reversed == amt or amt_after_fee == apply_on_reversed.amt_after_fee
            prop_assert_eq!(fees_charged, apply_on_reversed.fees_charged);
        }
    }

    proptest! {
        #[test]
        fn u64_fee_zero_fees_charged_reverse_err(fees_charged: u64, zero_fees in zero_u64_fees()) {
            for zero_fee in zero_fees {
                prop_assert_eq!(zero_fee.pseudo_reverse_from_fees_charged(fees_charged).unwrap_err(), MathError);
            }
        }
    }

    prop_compose! {
        fn u64_smaller_larger()
            (boundary in any::<u64>())
            (smaller in 0..=boundary, larger in boundary..=u64::MAX) -> (u64, u64) {
                (smaller, larger)
            }
    }

    proptest! {
        #[test]
        fn correct_valid_invalid_conditions((smaller, larger) in u64_smaller_larger()) {
            let valid = U64FeeFloor { fee_num: smaller, fee_denom: larger };
            prop_assert!(valid.is_valid());
            if smaller != larger {
                let invalid = U64FeeFloor { fee_num: larger, fee_denom: smaller };
                prop_assert!(!invalid.is_valid());
            }
        }
    }
}
