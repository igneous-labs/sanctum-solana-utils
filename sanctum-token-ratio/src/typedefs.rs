//! TOOD: add proptests to check documented invariants

use crate::MathError;

// inline all simple functions so that they can be inlined by consumers.
// dont need to do the same for generic fns and methods on generic structs since those
// are available to be inlined by consumers. TODO: confirm this

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct AmtsAfterFeeBuilder(u64);

impl AmtsAfterFeeBuilder {
    /// Constructs a new `AmtsAfterFeeBuilder` from an amt_before_fee
    #[inline]
    pub const fn new_amt_bef_fee(abf: u64) -> Self {
        Self(abf)
    }

    /// Returns the amt_before_fee encapsulated by this builder
    #[inline]
    pub const fn amt_bef_fee(&self) -> u64 {
        self.0
    }

    /// Errors if fee_charged > amt_before_fee
    #[inline]
    pub const fn with_fee_charged(self, fee_charged: u64) -> Result<AmtsAfterFee, MathError> {
        // match instead of ok_or(MathError)? to enable use with const
        let amt_after_fee = match self.amt_bef_fee().checked_sub(fee_charged) {
            Some(a) => a,
            None => return Err(MathError),
        };
        Ok(AmtsAfterFee {
            amt_after_fee,
            fee_charged,
        })
    }

    /// Panics if fee_charged > amt_before_fee
    #[inline]
    pub const fn with_fee_charged_unchecked(self, fee_charged: u64) -> AmtsAfterFee {
        // cannot unwrap() in const fn, but can match with panic! static msg
        match self.with_fee_charged(fee_charged) {
            Ok(r) => r,
            Err(_e) => panic!("fee_charged > amt_before_fee"),
        }
    }

    /// Errors if amt_after_fee > amt_before_fee
    #[inline]
    pub const fn with_amt_aft_fee(self, amt_after_fee: u64) -> Result<AmtsAfterFee, MathError> {
        // match instead of ok_or(MathError)? to enable use with const
        let fee_charged = match self.amt_bef_fee().checked_sub(amt_after_fee) {
            Some(f) => f,
            None => return Err(MathError),
        };
        Ok(AmtsAfterFee {
            amt_after_fee,
            fee_charged,
        })
    }

    /// Panics if amt_after_fee > amt_before_fee
    #[inline]
    pub const fn with_amt_aft_fee_unchecked(self, amt_after_fee: u64) -> AmtsAfterFee {
        // cannot unwrap() in const fn, but can match with panic! static msg
        match self.with_amt_aft_fee(amt_after_fee) {
            Ok(r) => r,
            Err(_e) => panic!("amt_after_fee > amt_before_fee"),
        }
    }
}

/// invariant: amt_after_fees + fee_charged = amt_before_fees.
///
/// Fields are private to ensure invariant is never violated.
/// Use [`AmtsAfterFeeBuilder`] to build this struct
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
pub struct AmtsAfterFee {
    amt_after_fee: u64,
    fee_charged: u64,
}

impl AmtsAfterFee {
    #[inline]
    pub const fn amt_after_fee(&self) -> u64 {
        self.amt_after_fee
    }

    #[inline]
    pub const fn fee_charged(&self) -> u64 {
        self.fee_charged
    }

    #[inline]
    pub const fn amt_before_fee(&self) -> u64 {
        // cannot unwrap() in const fn, but can match with unreachable!()
        match self.amt_after_fee().checked_add(self.fee_charged()) {
            Some(r) => r,
            // since we can only create this from AmtsAfterFeeBuilder,
            // no overflow is guaranteed
            None => unreachable!(),
        }
    }
}

#[cfg(feature = "borsh")]
pub const AMTS_AFTER_FEE_BORSH_SER_LEN: usize = 16;

/// A range of u64 values. Values are usually exclusive.
///
/// Fields are private to ensure invariant of `min <= max` is never violated
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
pub struct U64ValueRange {
    min: u64,
    max: u64,
}

#[cfg(feature = "borsh")]
pub const U64_VALUE_RANGE_BORSH_SER_LEN: usize = 16;

impl U64ValueRange {
    /// `[0, 0]`
    pub const ZERO: Self = Self::single(0);

    /// `[u64::MAX, u64::MAX]`
    pub const MAX: Self = Self::single(u64::MAX);

    /// `[0, u64::MAX]`
    pub const FULL: Self = Self {
        min: 0,
        max: u64::MAX,
    };

    /// Create a new U64ValueRange from a min limit and max limit
    /// that can be passed in either order.
    #[inline]
    pub const fn from_range_auto(a: u64, b: u64) -> Self {
        if a > b {
            Self { min: b, max: a }
        } else {
            Self { min: a, max: b }
        }
    }

    /// Errors if min > max
    #[inline]
    pub const fn from_min_max(min: u64, max: u64) -> Result<Self, MathError> {
        if min > max {
            return Err(MathError);
        }
        Ok(Self { min, max })
    }

    /// Panics if min > max
    #[inline]
    pub const fn from_min_max_unchecked(min: u64, max: u64) -> Self {
        // cannot unwrap() in const fn, but can match with panic! static msg
        match Self::from_min_max(min, max) {
            Ok(r) => r,
            Err(_e) => panic!("min > max"),
        }
    }

    /// `[value, value]`
    #[inline]
    pub const fn single(value: u64) -> Self {
        Self {
            min: value,
            max: value,
        }
    }

    // Getters prefixed with `get_` to avoid collision with std::cmp methods

    #[inline]
    pub const fn get_min(&self) -> u64 {
        self.min
    }

    #[inline]
    pub const fn get_max(&self) -> u64 {
        self.max
    }
}

/// Indicates that any division operations in its main application should ceiling divide instead of floor
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct CeilDiv<T>(pub T);

impl<T> AsRef<T> for CeilDiv<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T> From<T> for CeilDiv<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}

/// Indicates that any division operations in its main application should floor divide instead of ceiling
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct FloorDiv<T>(pub T);

impl<T> AsRef<T> for FloorDiv<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T> From<T> for FloorDiv<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}
