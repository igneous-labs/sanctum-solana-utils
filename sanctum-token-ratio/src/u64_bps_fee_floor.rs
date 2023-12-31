use crate::{AmtsAfterFee, MathError, U64FeeFloor, U64ValueRange, BPS_DENOMINATOR};

/// A bps fee applied to a token amount
///
/// `fee_charged = amt * bps // 10_000``
///
/// `amt_after_fee = amt - fee_charged`
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
pub struct U64BpsFeeFloor(pub u16);

#[cfg(feature = "borsh")]
pub const U64_BPS_FEE_FLOOR_BORSH_SER_LEN: usize = 2;

impl U64BpsFeeFloor {
    pub const ZERO: Self = Self(0);
    pub const MAX: Self = Self(BPS_DENOMINATOR);

    pub const fn to_u64_fee_floor(&self) -> U64FeeFloor<u16, u16> {
        U64FeeFloor {
            fee_num: self.0,
            fee_denom: BPS_DENOMINATOR,
        }
    }

    /// Returns the results of applying this fee to a token amount
    ///
    /// Errors if:
    /// - bps > 10_000 (fee > 100%)
    pub fn apply(&self, amt: u64) -> Result<AmtsAfterFee, MathError> {
        self.to_u64_fee_floor().apply(amt)
    }

    /// Returns a possible amount that was fed into self.apply()
    ///
    /// Returns:
    /// - `U64ValueRange::single(amt_after_fee)` if bps == 0
    /// - `U64ValueRange::full()` if bps == 10_000 and amt_after_fee == 0
    /// - `U64ValueRange::zero()` if bps != 0 and bps != 10_000 and amt_after_fee = 0
    ///
    /// Errors if:
    /// - bps > 10_000 (fee > 100%)
    /// - bps == 10_000 but amt_after_fee != 0
    pub fn reverse_from_amt_after_fee(
        &self,
        amt_after_fee: u64,
    ) -> Result<U64ValueRange, MathError> {
        self.to_u64_fee_floor()
            .reverse_from_amt_after_fee(amt_after_fee)
    }

    /// Returns a possible amount that was fed into self.apply().
    ///
    /// Returns:
    /// - `U64ValueRange::full()` if zero fee and fee_charged == 0
    /// - `fee_charged` if bps == 10_000 (fee == 100%)
    ///
    /// Errors if:
    /// - bps > 10_000 (fee > 100%)
    /// - zero fee but fee_charged != 0
    pub fn reverse_from_fee_charged(&self, fee_charged: u64) -> Result<U64ValueRange, MathError> {
        self.to_u64_fee_floor()
            .reverse_from_fee_charged(fee_charged)
    }

    pub fn is_valid(&self) -> bool {
        self.0 <= BPS_DENOMINATOR
    }

    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }
}

#[cfg(all(test, feature = "std"))]
mod tests {
    use proptest::prelude::*;

    use super::*;

    prop_compose! {
        fn invalid_fees()
            (bps in 10_001..=u16::MAX) -> U64BpsFeeFloor {
                U64BpsFeeFloor(bps)
            }
    }

    prop_compose! {
        fn valid_fees()
            (bps in 0..=BPS_DENOMINATOR) -> U64BpsFeeFloor {
                U64BpsFeeFloor(bps)
            }
    }

    prop_compose! {
        fn valid_nonzero_fees()
            (bps in 1..=BPS_DENOMINATOR) -> U64BpsFeeFloor {
                U64BpsFeeFloor(bps)
            }
    }

    prop_compose! {
        fn valid_nonmax_fees()
            (bps in 0..BPS_DENOMINATOR) -> U64BpsFeeFloor {
                U64BpsFeeFloor(bps)
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
        fn zero_fee_apply_no_op(amt: u64) {
            let AmtsAfterFee { amt_after_fee, fee_charged } = U64BpsFeeFloor::ZERO.apply(amt).unwrap();
            prop_assert_eq!(amt_after_fee, amt);
            prop_assert_eq!(fee_charged, 0);
        }
    }

    proptest! {
        #[test]
        fn max_fee_apply_zero(amt: u64) {
            prop_assert_eq!(U64BpsFeeFloor::MAX.apply(amt).unwrap(), AmtsAfterFee { amt_after_fee: 0, fee_charged: amt });
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
        fn zero_fee_amt_after_fee_reverse_no_op(amt_after_fee: u64) {
            prop_assert_eq!(U64BpsFeeFloor::ZERO.reverse_from_amt_after_fee(amt_after_fee).unwrap(), U64ValueRange::single(amt_after_fee));
        }
    }

    proptest! {
        #[test]
        fn max_fee_nonzero_amt_after_fee_reverse_err(non_zero_amt_after_fee in 1..=u64::MAX) {
            prop_assert_eq!(U64BpsFeeFloor::MAX.reverse_from_amt_after_fee(non_zero_amt_after_fee).unwrap_err(), MathError);
        }
    }

    #[test]
    fn max_fee_zero_amt_after_fee_reverse_range_full() {
        assert_eq!(
            U64BpsFeeFloor::MAX.reverse_from_amt_after_fee(0).unwrap(),
            U64ValueRange::full()
        );
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
        fn zero_fee_nonzero_fee_charged_reverse_err(nonzero_fee_charged in 1..=u64::MAX) {
            prop_assert_eq!(U64BpsFeeFloor::ZERO.reverse_from_fee_charged(nonzero_fee_charged).unwrap_err(), MathError);
        }
    }

    #[test]
    fn zero_fee_zero_fee_charged_reverse_range_full() {
        assert_eq!(
            U64BpsFeeFloor::ZERO.reverse_from_fee_charged(0).unwrap(),
            U64ValueRange::full()
        );
    }

    proptest! {
        #[test]
        fn max_fee_fee_charged_reverse_no_op(fee_charged: u64) {
            prop_assert_eq!(U64BpsFeeFloor::MAX.reverse_from_fee_charged(fee_charged).unwrap(), U64ValueRange::single(fee_charged));
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
}
