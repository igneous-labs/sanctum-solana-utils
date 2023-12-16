use crate::{AmtsAfterFee, MathError, U64FeeFloor, BPS_DENOMINATOR};

/// A bps fee applied to a token amount
///
/// `fee_charged = amt * bps // 10_000``
///
/// `amt_after_fee = amt - fee_charged`
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub struct U64BpsFeeFloor(pub u16);

impl U64BpsFeeFloor {
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
    /// Returns `amt_after_fee` if bps == 0
    ///
    /// Errors if:
    /// - bps > 10_000 (fee > 100%)
    /// - bps == 10_000: infinite possibilities if fee = 100%
    pub fn pseudo_reverse_from_amt_after_fee(&self, amt_after_fee: u64) -> Result<u64, MathError> {
        self.to_u64_fee_floor()
            .pseudo_reverse_from_amt_after_fee(amt_after_fee)
    }

    /// Returns a possible amount that was fed into self.apply().
    ///
    /// Returns `fee_charged` if fee_num == fee_denom (fee == 100%)
    ///
    /// Errors if:
    /// - bps > 10_000 (fee > 100%)
    /// - bps == 0: can't compute amt_before_fee if no fees charged
    pub fn pseudo_reverse_from_fee_charged(&self, fee_charged: u64) -> Result<u64, MathError> {
        self.to_u64_fee_floor()
            .pseudo_reverse_from_fee_charged(fee_charged)
    }

    pub fn is_valid(&self) -> bool {
        self.0 <= BPS_DENOMINATOR
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::*;

    prop_compose! {
        fn valid_u64_fees()
            (fee_bps in 0..=BPS_DENOMINATOR) -> U64BpsFeeFloor {
                U64BpsFeeFloor(fee_bps)
            }
    }

    // basic

    proptest! {
        #[test]
        fn u64_fee_invariants(amt: u64, fee in valid_u64_fees()) {
            let AmtsAfterFee { amt_after_fee, fee_charged } = fee.apply(amt).unwrap();
            prop_assert!(amt_after_fee <= amt);
            prop_assert_eq!(amt, amt_after_fee + fee_charged);
        }
    }

    proptest! {
        #[test]
        fn u64_zero_fee(amt: u64) {
            let fee = U64BpsFeeFloor(0u16);
            let amts_after_fee = fee.apply(amt).unwrap();

            prop_assert_eq!(amts_after_fee.amt_after_fee, amt);
            prop_assert_eq!(amts_after_fee.fee_charged, 0);
        }
    }

    // pseudo_reverse_from_amt_after_fee()

    proptest! {
        #[test]
        fn u64_fee_round_trip_amt_after_fee(amt: u64, fee in valid_u64_fees()) {
            let AmtsAfterFee { amt_after_fee, .. } = fee.apply(amt).unwrap();

            let reversed = fee.pseudo_reverse_from_amt_after_fee(amt_after_fee).unwrap();
            let apply_on_reversed = fee.apply(reversed).unwrap();

            // cannot guarantee reversed == amt or fee_charged == apply_on_reversed.fee_charged
            prop_assert_eq!(amt_after_fee, apply_on_reversed.amt_after_fee);
        }
    }

    proptest! {
        #[test]
        fn u64_fee_zero_amt_after_fee_reverse_no_op(amt_after_fee: u64) {
            let zero_fee = U64BpsFeeFloor(0u16);
            let amt = zero_fee.pseudo_reverse_from_amt_after_fee(amt_after_fee).unwrap();
            prop_assert_eq!(amt_after_fee, amt);
        }
    }

    // pseudo_reverse_from_fee_charged()

    prop_compose! {
        fn valid_nonzero_u64_fees()
            (fee_bps in 1..=BPS_DENOMINATOR) -> U64BpsFeeFloor {
                U64BpsFeeFloor(fee_bps)
            }
    }

    proptest! {
        #[test]
        fn u64_fee_round_trip_fee_charged(amt: u64, fee in valid_nonzero_u64_fees()) {
            let AmtsAfterFee { fee_charged, .. } = fee.apply(amt).unwrap();

            let reversed = fee.pseudo_reverse_from_fee_charged(fee_charged).unwrap();
            let apply_on_reversed = fee.apply(reversed).unwrap();

            // cannot guarantee reversed == amt or amt_after_fee == apply_on_reversed.amt_after_fee
            prop_assert_eq!(fee_charged, apply_on_reversed.fee_charged);
        }
    }

    proptest! {
        #[test]
        fn u64_fee_zero_fee_charged_reverse_err(fee_charged: u64) {
            let zero_fee = U64BpsFeeFloor(0u16);
            prop_assert_eq!(zero_fee.pseudo_reverse_from_fee_charged(fee_charged).unwrap_err(), MathError);
        }
    }

    // is_valid()

    prop_compose! {
        fn u64_smaller_larger()
            (boundary in any::<u64>())
            (smaller in 0..=boundary, larger in boundary..=u64::MAX) -> (u64, u64) {
                (smaller, larger)
            }
    }

    proptest! {
        #[test]
        fn valid_invalid(bps: u16) {
            let fee = U64BpsFeeFloor(bps);
            if bps > BPS_DENOMINATOR {
                prop_assert!(!fee.is_valid())
            } else {
                prop_assert!(fee.is_valid())
            }
        }
    }
}
