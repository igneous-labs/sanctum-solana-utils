use core::ops::Deref;

use crate::{AmtsAfterFee, MathError, U64Ratio, U64ValueRange};

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

/// A fee rate/percentage that is expressed as a ratio
pub trait FeeRatio {
    type N: Copy + Into<u128>;
    type D: Copy + Into<u128>;

    fn to_u64_ratio(&self) -> U64Ratio<Self::N, Self::D>;

    fn fee_num(&self) -> Self::N;

    fn fee_denom(&self) -> Self::D;
}

impl<Ref: Deref<Target = T>, T: FeeRatio + ?Sized> FeeRatio for Ref {
    type N = T::N;

    type D = T::D;

    fn to_u64_ratio(&self) -> U64Ratio<Self::N, Self::D> {
        self.deref().to_u64_ratio()
    }

    fn fee_num(&self) -> Self::N {
        self.deref().fee_num()
    }

    fn fee_denom(&self) -> Self::D {
        self.deref().fee_denom()
    }
}

pub trait FeeRatioBounds {
    /// Returns true if this fee charges nothing (0%)
    fn is_zero(&self) -> bool;

    /// Returns true if this fee charges 100%
    fn is_max(&self) -> bool;
}

// blanket to make it unoverridable
impl<T: FeeRatio> FeeRatioBounds for T {
    fn is_zero(&self) -> bool {
        self.to_u64_ratio().is_zero()
    }

    fn is_max(&self) -> bool {
        self.to_u64_ratio().is_one()
    }
}

/// Reads `1 - fee_ratio` aka `(d - n)/d`
pub trait FeeRatioInv {
    type N: Copy + Into<u128>;
    type D: Copy + Into<u128>;

    /// (d - n)/d
    fn one_minus_fee_ratio(&self) -> Result<U64Ratio<u128, u128>, MathError>;
}

// blanket to make it unoverridable
impl<T: FeeRatio> FeeRatioInv for T {
    type N = T::N;

    type D = T::D;

    fn one_minus_fee_ratio(&self) -> Result<U64Ratio<u128, u128>, MathError> {
        let n: u128 = self.fee_num().into();
        let d: u128 = self.fee_denom().into();
        match d.checked_sub(n) {
            Some(num) => Ok(U64Ratio { num, denom: d }),
            None => Err(MathError),
        }
    }
}

/// Useful when working with untrusted data
/// e.g. bytes deserialized over network
pub trait FeeRatioValidator {
    fn is_valid(&self) -> bool;

    fn validate(self) -> Result<Self, MathError>
    where
        Self: Sized,
    {
        match self.is_valid() {
            true => Ok(self),
            false => Err(MathError),
        }
    }
}

// blanket to make it unoverridable
impl<T: FeeRatio> FeeRatioValidator for T {
    fn is_valid(&self) -> bool {
        let n: u128 = self.fee_num().into();
        let d: u128 = self.fee_denom().into();
        n <= d
    }
}
