use core::ops::Deref;

use crate::{MathError, U64Ratio};

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
pub trait FeeRatioValid {
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
impl<T: FeeRatio> FeeRatioValid for T {
    fn is_valid(&self) -> bool {
        if self.is_zero() {
            true
        } else {
            self.fee_num().into() <= self.fee_denom().into()
        }
    }
}
