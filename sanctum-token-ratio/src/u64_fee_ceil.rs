use core::cmp::Ordering;

use crate::{AmtsAfterFee, MathError, U64RatioFloor, U64ValueRange};

/// A fee ratio applied to a token amount. Should be <= 1.0.
///
/// `amt_after_fee = amt * (fee_denom - fee_num) // fee_denom`
///
/// `fee_charged = amt - amt_after_fee`
///
/// Effectively maximizes fees charged
#[derive(Debug, Copy, Clone, Default, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
pub struct U64FeeCeil<N, D> {
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
    /// - `U64ValueRange::single(amt_after_fee)` if fee_num == 0 || fee_denom == 0 (zero fees)
    /// - `U64ValueRange::full()` if fee_num == fee_denom (fee = 100%) and amt_after_fee == 0
    ///
    /// Errors if:
    /// - fee_num > fee_denom (fee > 100%)
    /// - fee_num == fee_denom but amt_after_fee != 0
    pub fn reverse_from_amt_after_fee(
        &self,
        amt_after_fee: u64,
    ) -> Result<U64ValueRange, MathError> {
        let n: u128 = self.fee_num.into();
        let d: u128 = self.fee_denom.into();
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
        let ratio_floor = self.amt_after_fee_ratio_floor()?;
        ratio_floor.reverse(amt_after_fee)
    }

    /// Returns a possible amount that was fed into self.apply().
    ///
    /// Returns:
    /// - `U64ValueRange::single(fee_charged)` if fee_num == fee_denom (fee == 100%)
    /// - `U64ValueRange::full()` if fee_num == 0 || fee_denom == 0 (zero fees) and fee_charged == 0
    /// - `U64ValueRange::zero()` if nonzero nonmax fee and fee_charged == 0
    ///
    /// Errors if:
    /// - fee_num > fee_denom (fee > 100%)
    /// - fee_num == 0 || fee_denom == 0 but fee_charged != 0
    pub fn reverse_from_fee_charged(&self, fee_charged: u64) -> Result<U64ValueRange, MathError> {
        let n: u128 = self.fee_num.into();
        let d: u128 = self.fee_denom.into();
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
        if fee_charged == 0 {
            return Ok(U64ValueRange::zero());
        }

        let f: u128 = fee_charged.into();

        let f_minus_1 = f - 1; // fee_charged != 0 checked above
        let df_minus_1 = d.checked_mul(f_minus_1).ok_or(MathError)?;
        let min = (df_minus_1 / n) // n != 0 checked above
            .saturating_add(1)
            .try_into()
            .map_err(|_e| MathError)?;

        let df = d.checked_mul(f).ok_or(MathError)?;
        let max = (df / n).try_into().map_err(|_e| MathError)?;

        Ok(U64ValueRange { min, max })
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
    > PartialEq<U64FeeCeil<RN, RD>> for U64FeeCeil<LN, LD>
{
    fn eq(&self, rhs: &U64FeeCeil<RN, RD>) -> bool {
        U64RatioFloor {
            num: self.fee_num,
            denom: self.fee_denom,
        }
        .eq(&U64RatioFloor {
            num: rhs.fee_num,
            denom: rhs.fee_denom,
        })
    }
}

impl<N: Copy + Into<u128>, D: Copy + Into<u128>> Eq for U64FeeCeil<N, D> {}

impl<
        LN: Copy + Into<u128>,
        LD: Copy + Into<u128>,
        RN: Copy + Into<u128>,
        RD: Copy + Into<u128>,
    > PartialOrd<U64FeeCeil<RN, RD>> for U64FeeCeil<LN, LD>
{
    fn partial_cmp(&self, rhs: &U64FeeCeil<RN, RD>) -> Option<Ordering> {
        U64RatioFloor {
            num: self.fee_num,
            denom: self.fee_denom,
        }
        .partial_cmp(&U64RatioFloor {
            num: rhs.fee_num,
            denom: rhs.fee_denom,
        })
    }
}

impl<N: Copy + Into<u128>, D: Copy + Into<u128>> Ord for U64FeeCeil<N, D> {
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
                    U64FeeCeil { fee_num: a, fee_denom: common },
                    U64FeeCeil { fee_num: b, fee_denom: common }
                );
                prop_assert_eq!(
                    U64FeeCeil { fee_num: common, fee_denom: a },
                    U64FeeCeil { fee_num: common, fee_denom: b }
                );
            }
            let (smaller, larger) = if a < b {
                (a, b)
            } else {
                (b, a)
            };
            let s = U64FeeCeil { fee_num: smaller, fee_denom: common };
            let l = U64FeeCeil { fee_num: larger, fee_denom: common };
            prop_assert!(s < l);

            let s = U64FeeCeil { fee_num: common, fee_denom: larger };
            let l = U64FeeCeil { fee_num: common, fee_denom: smaller };
            prop_assert!(s < l);
        }
    }
}
