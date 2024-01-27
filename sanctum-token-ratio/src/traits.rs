use core::ops::Deref;

use crate::{AmtsAfterFee, MathError, U64ValueRange};

pub trait ReversibleFee {
    fn apply(&self, amt_before_fee: u64) -> Result<AmtsAfterFee, MathError>;

    fn reverse_from_amt_after_fee(&self, amt_after_fee: u64) -> Result<U64ValueRange, MathError>;

    fn reverse_from_fee_charged(&self, fee_charged: u64) -> Result<U64ValueRange, MathError>;
}

impl<Ref: Deref<Target = T>, T: ReversibleFee + ?Sized> ReversibleFee for Ref {
    fn apply(&self, amt_before_fee: u64) -> Result<AmtsAfterFee, MathError> {
        self.deref().apply(amt_before_fee)
    }

    fn reverse_from_amt_after_fee(&self, amt_after_fee: u64) -> Result<U64ValueRange, MathError> {
        self.deref().reverse_from_amt_after_fee(amt_after_fee)
    }

    fn reverse_from_fee_charged(&self, fee_charged: u64) -> Result<U64ValueRange, MathError> {
        self.deref().reverse_from_fee_charged(fee_charged)
    }
}

pub trait ReversibleRatio {
    fn apply(&self, amount: u64) -> Result<u64, MathError>;

    fn reverse(&self, amt_after_apply: u64) -> Result<U64ValueRange, MathError>;
}

impl<Ref: Deref<Target = T>, T: ReversibleRatio + ?Sized> ReversibleRatio for Ref {
    fn apply(&self, amount: u64) -> Result<u64, MathError> {
        self.deref().apply(amount)
    }

    fn reverse(&self, amt_after_apply: u64) -> Result<U64ValueRange, MathError> {
        self.deref().reverse(amt_after_apply)
    }
}
