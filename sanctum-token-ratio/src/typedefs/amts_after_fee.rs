use crate::MathError;

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
        match self.amt_bef_fee().checked_sub(fee_charged) {
            Some(amt_after_fee) => Ok(AmtsAfterFee {
                amt_after_fee,
                fee_charged,
            }),
            None => Err(MathError),
        }
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
        match self.amt_bef_fee().checked_sub(amt_after_fee) {
            Some(fee_charged) => Ok(AmtsAfterFee {
                amt_after_fee,
                fee_charged,
            }),
            None => Err(MathError),
        }
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

#[cfg(feature = "borsh")]
pub const AMTS_AFTER_FEE_BORSH_SER_LEN: usize = 16;

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
    pub const fn amt_before_fee(&self) -> Result<u64, MathError> {
        match self.amt_after_fee().checked_add(self.fee_charged()) {
            Some(r) => Ok(r),
            // overflow may occur if from untrusted sources e.g. deserialized over network
            None => Err(MathError),
        }
    }
}
