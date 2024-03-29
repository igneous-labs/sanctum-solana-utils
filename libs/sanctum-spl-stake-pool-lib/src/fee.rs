use std::cmp::Ordering;

use spl_stake_pool_interface::{Fee, FeeType};

/// Newtype used to perform fraction comparisons between [`Fee`]s
#[derive(Debug, Clone, Copy)]
pub struct CmpFee<'a>(pub &'a Fee);

impl<'a> CmpFee<'a> {
    pub fn is_zero(&self) -> bool {
        self.0.numerator == 0 || self.0.denominator == 0
    }

    /// Returns true if this Fee represents 1.0 (100% fees)
    pub fn is_one(&self) -> bool {
        !self.is_zero() && self.0.numerator == self.0.denominator
    }
}

fn cmp_inner(lhs: &CmpFee, rhs: &CmpFee) -> Ordering {
    // zero-edge cases
    if lhs.is_zero() {
        return if rhs.is_zero() {
            Ordering::Equal
        } else {
            Ordering::Less
        };
    }
    // lhs != 0
    if rhs.is_zero() {
        return Ordering::Greater;
    }

    // cast into u128 to avoid overflow
    let ln: u128 = lhs.0.numerator.into();
    let ld: u128 = lhs.0.denominator.into();
    let rn: u128 = rhs.0.numerator.into();
    let rd: u128 = rhs.0.denominator.into();

    let lhs = ln * rd;
    let rhs = rn * ld;

    lhs.cmp(&rhs)
}

impl<'a> PartialEq for CmpFee<'a> {
    fn eq(&self, other: &Self) -> bool {
        cmp_inner(self, other).is_eq()
    }
}

impl<'a> Eq for CmpFee<'a> {}

impl<'a> PartialOrd for CmpFee<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for CmpFee<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        cmp_inner(self, other)
    }
}

/// Owned version of [`CmpFee`]
#[derive(Debug, Clone)]
pub struct CmpFeeOwned(pub Fee);

impl PartialEq for CmpFeeOwned {
    fn eq(&self, other: &Self) -> bool {
        CmpFee(&self.0).eq(&CmpFee(&other.0))
    }
}

impl Eq for CmpFeeOwned {}

impl PartialOrd for CmpFeeOwned {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CmpFeeOwned {
    fn cmp(&self, other: &Self) -> Ordering {
        CmpFee(&self.0).cmp(&CmpFee(&other.0))
    }
}

/// Newtype used to perform fraction comparisons between [`FeeType`]s
#[derive(Debug, Clone, Copy)]
pub struct EqFeeType<'a>(pub &'a FeeType);

impl<'a> PartialEq for EqFeeType<'a> {
    fn eq(&self, other: &Self) -> bool {
        match (self.0, other.0) {
            (FeeType::SolDeposit { fee: s }, FeeType::SolDeposit { fee: o })
            | (FeeType::StakeDeposit { fee: s }, FeeType::StakeDeposit { fee: o })
            | (FeeType::SolWithdrawal { fee: s }, FeeType::SolWithdrawal { fee: o })
            | (FeeType::StakeWithdrawal { fee: s }, FeeType::StakeWithdrawal { fee: o })
            | (FeeType::Epoch { fee: s }, FeeType::Epoch { fee: o }) => CmpFee(s).eq(&CmpFee(o)),
            (FeeType::SolReferral { fee: s }, FeeType::SolReferral { fee: o })
            | (FeeType::StakeReferral { fee: s }, FeeType::StakeReferral { fee: o }) => s.eq(o),
            _ => false,
        }
    }
}

impl<'a> Eq for EqFeeType<'a> {}

/// Owned version of [`EqFeeType`]
#[derive(Debug, Clone)]
pub struct EqFeeTypeOwned(pub FeeType);

impl PartialEq for EqFeeTypeOwned {
    fn eq(&self, other: &Self) -> bool {
        EqFeeType(&self.0).eq(&EqFeeType(&other.0))
    }
}

impl Eq for EqFeeTypeOwned {}
