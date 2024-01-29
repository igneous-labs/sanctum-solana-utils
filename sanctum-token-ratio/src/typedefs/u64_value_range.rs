use crate::MathError;

/// A range of u64 values.
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
            Err(MathError)
        } else {
            Ok(Self { min, max })
        }
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

    /// Verifies if this range is valid (min <= max)
    /// Useful for dealing with untrusted data e.g. deserialized over network
    #[inline]
    pub const fn validate(self) -> Result<Self, MathError> {
        match self.is_valid() {
            true => Ok(self),
            false => Err(MathError),
        }
    }

    /// Returns true if this range is valid (min <= max)
    /// Useful for dealing with untrusted data e.g. deserialized over network
    #[inline]
    pub const fn is_valid(&self) -> bool {
        self.min <= self.max
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
