use crate::{AmtsAfterFee, CeilDiv, MathError, ReversibleFee, U64BpsFee, U64ValueRange};

impl ReversibleFee for CeilDiv<U64BpsFee> {
    fn apply(&self, amt_before_fee: u64) -> Result<AmtsAfterFee, MathError> {
        CeilDiv(self.0.try_to_u64_fee_ratio()?).apply(amt_before_fee)
    }

    fn reverse_from_amt_after_fee(&self, amt_after_fee: u64) -> Result<U64ValueRange, MathError> {
        CeilDiv(self.0.try_to_u64_fee_ratio()?).reverse_from_amt_after_fee(amt_after_fee)
    }

    fn reverse_from_fee_charged(&self, fee_charged: u64) -> Result<U64ValueRange, MathError> {
        CeilDiv(self.0.try_to_u64_fee_ratio()?).reverse_from_fee_charged(fee_charged)
    }
}

#[cfg(all(test, feature = "std"))]
mod tests {
    use proptest::prelude::*;

    use crate::{bps_fee_test_utils::*, AmtsAfterFeeBuilder};

    use super::*;

    prop_compose! {
        fn valid_fees()
            (fee in valid_bps_fee()) -> CeilDiv<U64BpsFee> {
                CeilDiv(fee)
            }
    }

    prop_compose! {
        fn valid_nonmax_fees()
            (fee in valid_nonmax_bps_fee()) -> CeilDiv<U64BpsFee> {
                CeilDiv(fee)
            }
    }

    prop_compose! {
        fn valid_nonzero_fees()
            (fee in valid_nonzero_bps_fee()) -> CeilDiv<U64BpsFee> {
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
        fn zero_fee_apply_no_op(amt: u64) {
            let a = CeilDiv(U64BpsFee::ZERO).apply(amt).unwrap();
            prop_assert_eq!(a.amt_after_fee(), amt);
            prop_assert_eq!(a.fee_charged(), 0);
        }
    }

    proptest! {
        #[test]
        fn max_fee_apply_zero(amt: u64) {
            prop_assert_eq!(
                CeilDiv(U64BpsFee::MAX).apply(amt).unwrap(),
                AmtsAfterFeeBuilder::new_amt_bef_fee(amt).with_amt_aft_fee(0).unwrap()
            );
        }
    }

    // reverse_from_amt_after_fee()

    proptest! {
        #[test]
        fn amt_after_fee_round_trip(amt: u64, fee in valid_nonmax_fees()) {
            let a = fee.apply(amt).unwrap();
            let amt_after_fee = a.amt_after_fee();

            let r = fee.reverse_from_amt_after_fee(a.amt_after_fee()).unwrap();
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
        fn zero_fee_amt_after_fee_reverse_no_op(amt_after_fee: u64) {
            prop_assert_eq!(
                CeilDiv(U64BpsFee::ZERO).reverse_from_amt_after_fee(amt_after_fee).unwrap(),
                U64ValueRange::single(amt_after_fee)
            );
        }
    }

    proptest! {
        #[test]
        fn max_fee_nonzero_amt_after_fee_reverse_err(non_zero_amt_after_fee in 1..=u64::MAX) {
            prop_assert_eq!(CeilDiv(U64BpsFee::MAX).reverse_from_amt_after_fee(non_zero_amt_after_fee).unwrap_err(), MathError);
        }
    }

    #[test]
    fn max_fee_zero_amt_after_fee_reverse_range_full() {
        assert_eq!(
            CeilDiv(U64BpsFee::MAX)
                .reverse_from_amt_after_fee(0)
                .unwrap(),
            U64ValueRange::FULL
        );
    }

    // reverse_from_fee_charged()

    proptest! {
        #[test]
        fn fee_charged_round_trip(amt: u64, fee in valid_nonzero_fees()) {
            let a = fee.apply(amt).unwrap();
            let fee_charged = a.fee_charged();
            let amt_after_fee = a.amt_after_fee();

            let r = fee.reverse_from_fee_charged(a.fee_charged()).unwrap();
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
        fn zero_fee_nonzero_fee_charged_reverse_err(nonzero_fee_charged in 1..=u64::MAX) {
            prop_assert_eq!(CeilDiv(U64BpsFee::ZERO).reverse_from_fee_charged(nonzero_fee_charged).unwrap_err(), MathError);
        }
    }

    #[test]
    fn zero_fee_zero_fee_charged_reverse_range_full() {
        assert_eq!(
            CeilDiv(U64BpsFee::ZERO)
                .reverse_from_fee_charged(0)
                .unwrap(),
            U64ValueRange::FULL
        );
    }

    proptest! {
        #[test]
        fn max_fee_fee_charged_reverse_no_op(fee_charged: u64) {
            prop_assert_eq!(
                CeilDiv(U64BpsFee::MAX).reverse_from_fee_charged(fee_charged).unwrap(),
                U64ValueRange::single(fee_charged)
            );
        }
    }

    // invalid

    proptest! {
        #[test]
        fn invalid_fee_apply_err(amt: u64, fee in invalid_bps_fee()) {
            prop_assert_eq!(CeilDiv(fee).apply(amt).unwrap_err(), MathError);
        }
    }

    proptest! {
        #[test]
        fn invalid_fee_amt_after_fee_reverse_err(amt_after_fee: u64, fee in invalid_bps_fee()) {
            prop_assert_eq!(CeilDiv(fee).reverse_from_amt_after_fee(amt_after_fee).unwrap_err(), MathError);
        }
    }

    proptest! {
        #[test]
        fn invalid_fee_fee_charged_reverse_err(fee_charged: u64, fee in invalid_bps_fee()) {
            prop_assert_eq!(CeilDiv(fee).reverse_from_fee_charged(fee_charged).unwrap_err(), MathError);
        }
    }
}
