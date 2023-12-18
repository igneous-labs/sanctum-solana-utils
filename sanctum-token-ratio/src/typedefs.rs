/// amt_after_fees + fee_charged = amt_before_fees
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
pub struct AmtsAfterFee {
    pub amt_after_fee: u64,
    pub fee_charged: u64,
}

#[cfg(feature = "borsh")]
pub const AMTS_AFTER_FEE_BORSH_SER_LEN: usize = 16;

/// A range of u64 values. Values inclusive.
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
pub struct U64ValueRange {
    pub min: u64,
    pub max: u64,
}

#[cfg(feature = "borsh")]
pub const U64_VALUE_RANGE_BORSH_SER_LEN: usize = 16;

impl U64ValueRange {
    /// `[0, 0]`
    pub const fn zero() -> Self {
        Self::single(0)
    }

    /// `[0, u64::MAX]`
    pub const fn full() -> Self {
        Self {
            min: 0,
            max: u64::MAX,
        }
    }

    /// `[u64::MAX, u64::MAX]`
    pub const fn max() -> Self {
        Self::single(u64::MAX)
    }

    /// `[value, value]`
    pub const fn single(value: u64) -> Self {
        Self {
            min: value,
            max: value,
        }
    }
}
