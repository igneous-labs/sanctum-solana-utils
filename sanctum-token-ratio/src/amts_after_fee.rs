/// amt_after_fees + fee_charged = amt_before_fees
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq, Hash)]
pub struct AmtsAfterFee {
    pub amt_after_fee: u64,
    pub fee_charged: u64,
}
