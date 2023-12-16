use crate::{AmtsAfterFee, MathError, U64FeeCeil, BPS_DENOMINATOR};

/// A bps fee to charge where value <= 10_000
/// amt_after_fees = floor(amt * (10_000 - fee_num) / 10_000),
/// effectively maximizing fees charged
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub struct U64BpsFeeCeil(pub u16);

impl U64BpsFeeCeil {
    pub const fn as_u64_fee_ceil(&self) -> U64FeeCeil<u16, u16> {
        U64FeeCeil {
            fee_num: self.0,
            fee_denom: BPS_DENOMINATOR,
        }
    }

    /// Errors if value > 10_000 (fee > 100%)
    pub fn apply(&self, amt: u64) -> Result<AmtsAfterFee, MathError> {
        self.as_u64_fee_ceil().apply(amt)
    }

    /// Returns a possible amount that was fed into self.apply()
    ///
    /// Returns `amt_after_apply` if fee_num == 0 || fee_denom == 0
    ///
    /// Errors if:
    /// - fee_num > fee_denom (fee > 100%)
    pub fn pseudo_reverse_from_amt_after_fee(&self, amt_after_fee: u64) -> Result<u64, MathError> {
        self.as_u64_fee_ceil()
            .pseudo_reverse_from_amt_after_fee(amt_after_fee)
    }

    /// Returns a possible amount that was fed into self.apply()
    ///
    /// Returns `fees_charged` if fee_num == fee_denom (fee == 100%)
    ///
    /// Errors if:
    /// - fee_num > fee_denom (fee > 100%)
    /// - fee_num == 0 || fee_denom == 0: can't compute amt_before_fee if no fees charged
    pub fn pseudo_reverse_from_fees_charged(&self, fees_charged: u64) -> Result<u64, MathError> {
        self.as_u64_fee_ceil()
            .pseudo_reverse_from_fees_charged(fees_charged)
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
            (fee_bps in 0..=BPS_DENOMINATOR) -> U64BpsFeeCeil {
                U64BpsFeeCeil(fee_bps)
            }
    }

    // basic

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
        fn u64_zero_fee(amt: u64) {
            let fee = U64BpsFeeCeil(0u16);
            let amts_after_fee = fee.apply(amt).unwrap();

            prop_assert_eq!(amts_after_fee.amt_after_fee, amt);
            prop_assert_eq!(amts_after_fee.fees_charged, 0);
        }
    }

    // pseudo_reverse_from_amt_after_fee()

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
        fn u64_fee_zero_amt_after_fee_reverse_no_op(amt_after_fee: u64) {
            let zero_fee = U64BpsFeeCeil(0u16);
            let amt = zero_fee.pseudo_reverse_from_amt_after_fee(amt_after_fee).unwrap();
            prop_assert_eq!(amt_after_fee, amt);
        }
    }

    // pseudo_reverse_from_fees_charged()

    prop_compose! {
        fn valid_nonzero_u64_fees()
            (fee_bps in 1..=BPS_DENOMINATOR) -> U64BpsFeeCeil {
                U64BpsFeeCeil(fee_bps)
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
        fn u64_fee_zero_fees_charged_reverse_err(fees_charged: u64) {
            let zero_fee = U64BpsFeeCeil(0u16);
            prop_assert_eq!(zero_fee.pseudo_reverse_from_fees_charged(fees_charged).unwrap_err(), MathError);
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
            let fee = U64BpsFeeCeil(bps);
            if bps > BPS_DENOMINATOR {
                prop_assert!(!fee.is_valid())
            } else {
                prop_assert!(fee.is_valid())
            }
        }
    }
}
