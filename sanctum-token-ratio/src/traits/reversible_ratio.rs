use core::ops::Deref;

use crate::{MathError, U64ValueRange};

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
