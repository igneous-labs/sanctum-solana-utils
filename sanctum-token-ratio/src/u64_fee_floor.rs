use core::cmp::Ordering;

use crate::{AmtsAfterFee, MathError, U64RatioFloor, U64ValueRange};

/// A fee ratio applied to a token amount. Should be <= 1.0.
///
/// `fee_charged = amt * fee_num // fee_denom``
///
/// `amt_after_fee = amt - fee_charged`
#[derive(Debug, Copy, Clone, Default, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
pub struct U64FeeFloor<N, D> {
    pub fee_num: N,
    pub fee_denom: D,
}

impl<N: Copy + Into<u128>, D: Copy + Into<u128>> U64FeeFloor<N, D> {
    const fn fee_ratio_floor(&self) -> U64RatioFloor<N, D> {
        U64RatioFloor {
            num: self.fee_num,
            denom: self.fee_denom,
        }
    }

    /// Returns the results of applying this fee to a token amount
    ///
    /// Returns:
    /// - no fees charged if fee_num == 0 || fee_denom == 0
    ///
    /// Errors if:
    /// - fee_num > fee_denom (fee > 100%)
    pub fn apply(&self, amt: u64) -> Result<AmtsAfterFee, MathError> {
        let fee_charged = self.fee_ratio_floor().apply(amt)?;
        let amt_after_fee = amt.checked_sub(fee_charged).ok_or(MathError)?;
        Ok(AmtsAfterFee {
            amt_after_fee,
            fee_charged,
        })
    }

    /// Returns a possible amount that was fed into self.apply()
    ///
    /// Returns:
    /// - `U64ValueRange::single(amt_after_fee)` if fee_num == 0 || fee_denom == 0 (zero fees)
    /// - `U64ValueRange::full()` if fee_num == fee_denom (fee = 100%) and amt_after_fee == 0
    /// - `U64ValueRange::zero()` if fee != 100% and fee != 0 and amt_after_fee = 0
    ///
    /// Errors if:
    /// - fee_num > fee_denom (fee > 100%)
    /// - fee_num == fee_denom but amt_after_fee != 0
    pub fn reverse_from_amt_after_fee(
        &self,
        amt_after_fee: u64,
    ) -> Result<U64ValueRange, MathError> {
        let n = self.fee_num.into();
        let d = self.fee_denom.into();
        if n == 0 || d == 0 {
            return Ok(U64ValueRange::single(amt_after_fee));
        }
        if n > d {
            return Err(MathError);
        }
        if n == d {
            if amt_after_fee != 0 {
                return Err(MathError);
            } else {
                return Ok(U64ValueRange::full());
            }
        }
        if amt_after_fee == 0 {
            return Ok(U64ValueRange::zero());
        }

        let y: u128 = amt_after_fee.into();
        let d_minus_n = d - n; // n < d checked above

        let dy = d.checked_mul(y).ok_or(MathError)?;
        let max = (dy / d_minus_n).try_into().map_err(|_e| MathError)?; // d_minus_n != 0 since d != n

        let y_minus_1 = y - 1; // y != 0 checked above
        let dy_minus_1 = d.checked_mul(y_minus_1).ok_or(MathError)?;
        let min = (dy_minus_1 / d_minus_n) // d_minus_n != 0 since d != n
            .saturating_add(1)
            .try_into()
            .map_err(|_e| MathError)?;

        Ok(U64ValueRange { min, max })
    }

    /// Returns a possible amount that was fed into self.apply().
    ///
    /// Returns:
    /// - `U64ValueRange::full()` if zero fee and fee_charged == 0
    /// - `fee_charged` if fee_num == fee_denom (fee == 100%)
    ///
    /// Errors if:
    /// - fee_num > fee_denom (fee > 100%)
    /// - zero fee but fee_charged != 0
    pub fn reverse_from_fee_charged(&self, fee_charged: u64) -> Result<U64ValueRange, MathError> {
        let n = self.fee_num.into();
        let d = self.fee_denom.into();
        if n == 0 || d == 0 {
            if fee_charged == 0 {
                return Ok(U64ValueRange::full());
            } else {
                return Err(MathError);
            }
        }
        if n > d {
            return Err(MathError);
        }
        if n == d {
            return Ok(U64ValueRange::single(fee_charged));
        }
        self.fee_ratio_floor().reverse(fee_charged)
    }

    pub fn is_valid(&self) -> bool {
        self.fee_num.into() <= self.fee_denom.into()
    }

    pub fn is_zero(&self) -> bool {
        self.fee_denom.into() == 0 || self.fee_num.into() == 0
    }
}

impl<
        LN: Copy + Into<u128>,
        LD: Copy + Into<u128>,
        RN: Copy + Into<u128>,
        RD: Copy + Into<u128>,
    > PartialEq<U64FeeFloor<RN, RD>> for U64FeeFloor<LN, LD>
{
    fn eq(&self, other: &U64FeeFloor<RN, RD>) -> bool {
        self.fee_ratio_floor().eq(&other.fee_ratio_floor())
    }
}

impl<N: Copy + Into<u128>, D: Copy + Into<u128>> Eq for U64FeeFloor<N, D> {}

impl<
        LN: Copy + Into<u128>,
        LD: Copy + Into<u128>,
        RN: Copy + Into<u128>,
        RD: Copy + Into<u128>,
    > PartialOrd<U64FeeFloor<RN, RD>> for U64FeeFloor<LN, LD>
{
    fn partial_cmp(&self, other: &U64FeeFloor<RN, RD>) -> Option<Ordering> {
        self.fee_ratio_floor().partial_cmp(&other.fee_ratio_floor())
    }
}

impl<N: Copy + Into<u128>, D: Copy + Into<u128>> Ord for U64FeeFloor<N, D> {
    fn cmp(&self, rhs: &Self) -> Ordering {
        self.partial_cmp(rhs).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::*;

    prop_compose! {
        fn invalid_fees()
            (fee_num in any::<u64>())
            (fee_denom in 1..fee_num, fee_num in Just(fee_num)) -> U64FeeFloor<u64, u64> {
                U64FeeFloor { fee_num, fee_denom }
            }
    }

    prop_compose! {
        fn valid_fees()
            (fee_denom in any::<u64>())
            (fee_num in 0..=fee_denom, fee_denom in Just(fee_denom)) -> U64FeeFloor<u64, u64> {
                U64FeeFloor { fee_num, fee_denom }
            }
    }

    prop_compose! {
        fn valid_nonzero_fees()
            (fee_denom in any::<u64>())
            (fee_num in 1..=fee_denom, fee_denom in Just(fee_denom)) -> U64FeeFloor<u64, u64> {
                U64FeeFloor { fee_num, fee_denom }
            }
    }

    prop_compose! {
        fn valid_nonmax_fees()
            (fee_denom in any::<u64>())
            (fee_num in 0..fee_denom, fee_denom in Just(fee_denom)) -> U64FeeFloor<u64, u64> {
                U64FeeFloor { fee_num, fee_denom }
            }
    }

    prop_compose! {
        fn valid_max_fees()
            (n in 1..=u64::MAX) -> U64FeeFloor<u64, u64> {
                U64FeeFloor { fee_num: n, fee_denom: n }
            }
    }

    prop_compose! {
        fn valid_zero_denom_fees()
            (fee_num in 0..=u64::MAX, fee_denom in Just(0)) -> U64FeeFloor<u64, u64> {
                U64FeeFloor { fee_num, fee_denom }
            }
    }

    prop_compose! {
        fn valid_zero_num_fees()
            (fee_num in Just(0), fee_denom in 0..=u64::MAX) -> U64FeeFloor<u64, u64> {
                U64FeeFloor { fee_num, fee_denom }
            }
    }

    prop_compose! {
        fn valid_zero_fees()
            (zero_num in valid_zero_num_fees(), zero_denom in valid_zero_denom_fees()) -> [U64FeeFloor<u64, u64>; 2] {
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

    // reverse_from_amt_after_fee()

    proptest! {
        #[test]
        fn amt_after_fee_round_trip(amt: u64, fee in valid_nonmax_fees()) {
            let AmtsAfterFee { amt_after_fee, .. } = fee.apply(amt).unwrap();

            let U64ValueRange { min, max } = fee.reverse_from_amt_after_fee(amt_after_fee).unwrap();

            // cannot guarantee reversed == amt or fee_charged == apply_on_reversed.fee_charged
            prop_assert_eq!(amt_after_fee, fee.apply(min).unwrap().amt_after_fee);
            prop_assert_eq!(amt_after_fee, fee.apply(max).unwrap().amt_after_fee);
        }
    }

    proptest! {
        #[test]
        fn zero_fee_amt_after_fee_reverse_no_op(amt_after_fee: u64, zero_fees in valid_zero_fees()) {
            for zero_fee in zero_fees {
                prop_assert_eq!(zero_fee.reverse_from_amt_after_fee(amt_after_fee).unwrap(), U64ValueRange::single(amt_after_fee));
            }
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
            prop_assert_eq!(fee.reverse_from_amt_after_fee(0).unwrap(), U64ValueRange::full());
        }
    }

    proptest! {
        #[test]
        fn invalid_fee_amt_after_fee_reverse_err(amt_after_fee: u64, fee in invalid_fees()) {
            prop_assert_eq!(fee.reverse_from_amt_after_fee(amt_after_fee).unwrap_err(), MathError);
        }
    }

    // reverse_from_fee_charged()

    proptest! {
        #[test]
        fn fee_charged_round_trip(amt: u64, fee in valid_nonzero_fees()) {
            let AmtsAfterFee { fee_charged, .. } = fee.apply(amt).unwrap();

            let U64ValueRange { min, max } = fee.reverse_from_fee_charged(fee_charged).unwrap();

            // cannot guarantee reversed == amt or amt_after_fee == apply_on_reversed.amt_after_fee
            prop_assert_eq!(fee_charged, fee.apply(min).unwrap().fee_charged);
            prop_assert_eq!(fee_charged, fee.apply(max).unwrap().fee_charged);
        }
    }

    proptest! {
        #[test]
        fn zero_fee_nonzero_fee_charged_reverse_err(nonzero_fee_charged in 1..=u64::MAX, zero_fees in valid_zero_fees()) {
            for zero_fee in zero_fees {
                prop_assert_eq!(zero_fee.reverse_from_fee_charged(nonzero_fee_charged).unwrap_err(), MathError);
            }
        }
    }

    proptest! {
        #[test]
        fn zero_fee_zero_fee_charged_reverse_range_full(zero_fees in valid_zero_fees()) {
            for zero_fee in zero_fees {
                prop_assert_eq!(zero_fee.reverse_from_fee_charged(0).unwrap(), U64ValueRange::full());
            }
        }
    }

    proptest! {
        #[test]
        fn max_fee_fee_charged_reverse_no_op(fee_charged: u64, fee in valid_max_fees()) {
            prop_assert_eq!(fee.reverse_from_fee_charged(fee_charged).unwrap(), U64ValueRange::single(fee_charged));
        }
    }

    proptest! {
        #[test]
        fn invalid_fee_fee_charged_reverse_err(fee_charged: u64, fee in invalid_fees()) {
            prop_assert_eq!(fee.reverse_from_fee_charged(fee_charged).unwrap_err(), MathError);
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

    proptest! {
        #[test]
        fn ord(common: u64, a: u64, b: u64) {
            if a == b {
                prop_assert_eq!(
                    U64FeeFloor { fee_num: a, fee_denom: common },
                    U64FeeFloor { fee_num: b, fee_denom: common }
                );
                prop_assert_eq!(
                    U64FeeFloor { fee_num: common, fee_denom: a },
                    U64FeeFloor { fee_num: common, fee_denom: b }
                );
            }
            let (smaller, larger) = if a < b {
                (a, b)
            } else {
                (b, a)
            };
            let s = U64FeeFloor { fee_num: smaller, fee_denom: common };
            let l = U64FeeFloor { fee_num: larger, fee_denom: common };
            prop_assert!(s < l);

            let s = U64FeeFloor { fee_num: common, fee_denom: larger };
            let l = U64FeeFloor { fee_num: common, fee_denom: smaller };
            prop_assert!(s < l);
        }
    }
}
