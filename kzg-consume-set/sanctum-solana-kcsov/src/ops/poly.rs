use core::{borrow::Borrow, fmt::Display};

use ark_bn254::Fr;
use ark_ff::Field;
use solana_program::alt_bn128::{compression::prelude::G1, AltBn128Error};

use crate::{AltBn128G1ScalarMul, FR};

use super::AltBn128G1Add;

#[derive(Debug, PartialEq, Eq)]
pub enum PolyFromRootsErr {
    TooManyRoots,
    InvalidDegree,
}

impl Display for PolyFromRootsErr {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::TooManyRoots => f.write_str("TooManyRoots"),
            Self::InvalidDegree => f.write_str("InvalidDegree"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for PolyFromRootsErr {}

/// Given all the roots of a polynomial, output the coefficient array in ascending powers
///
/// i.e. with max degree 2 (DP1 = 3) $x^2 + 2x + 3$ -> [3, 2, 1], $x + 2$ -> [2, 1, 0].
///
/// This just does the naive n^2 thing of multiplying everything out,
/// meant to be used for small calculations onchain such as for calculating $z(\tau)G1$
/// for a small number of roots to verify
///
/// Generics:
/// - `DP1` - degree plus one. Maximum degree of the output polynomial + 1
///
/// Errors if:
/// - DP1 < 1 (max output degree < 0)
/// - roots.len() < 1 (degree < 1)
#[inline]
pub fn poly_from_roots<const DP1: usize>(roots: &[Fr]) -> Result<[Fr; DP1], PolyFromRootsErr> {
    // this just does the naive n^2 thing of multiplying everything out
    let d = roots.len();
    if DP1 < 1 || d < 1 {
        return Err(PolyFromRootsErr::InvalidDegree);
    }
    if d > DP1 - 1 {
        return Err(PolyFromRootsErr::TooManyRoots);
    }
    let mut res = [Fr::ZERO; DP1];

    // highest power coeff is always 1
    let mut n = d;
    res[n] = Fr::ONE;

    for root in roots.iter() {
        n -= 1;
        for j in n..d {
            res[j] -= res[j + 1] * root;
        }
    }

    Ok(res)
}

/// `itr` yields `(polynomial coefficients, G1 generator * tau^{p})` in increasing powers.
///
/// Note that first element is power 0 (constant term), so first point yielded should be just the G1 generator
///
/// If `itr` is created by zipping two iterators, it'll evaluate up to the min of either iterator's length
#[inline]
pub fn eval_poly_pwrs_of_tau_g1<S: Borrow<[u8; FR]>, P: Borrow<[u8; G1]>>(
    itr: impl IntoIterator<Item = (S, P)>,
) -> Result<[u8; G1], AltBn128Error> {
    let mut itr = itr.into_iter();
    // TODO: switch to try_reduce once stable
    let constant = itr.next().map_or_else(
        || Err(AltBn128Error::InvalidInputData),
        |(s, p)| {
            AltBn128G1ScalarMul::new_zeros()
                .with_scalar(s.borrow())
                .with_g1_pt(p.borrow())
                .exec()
        },
    )?;
    itr.try_fold(constant, |accum, (s, p)| {
        let term = AltBn128G1ScalarMul::new_zeros()
            .with_scalar(s.borrow())
            .with_g1_pt(p.borrow())
            .exec()?;
        AltBn128G1Add::new_zeros()
            .with_lhs(&accum)
            .with_rhs(&term)
            .exec()
    })
}

#[cfg(test)]
mod tests {
    use ark_ff::BigInt;
    use proptest::{collection::vec, prelude::*};
    use sanctum_solana_kcsc::{fr_from_hash, HASH_SIZE};

    use super::*;

    proptest! {
        #[test]
        fn linear_from_roots(r: [u8; HASH_SIZE]) {
            let r = fr_from_hash(r);
            let [c0, c1] = poly_from_roots(&[r]).unwrap();
            prop_assert_eq!(c1, Fr::ONE);
            prop_assert_eq!(c0, -r);
        }
    }

    proptest! {
        #[test]
        fn quadratic_from_roots(r1: [u8; HASH_SIZE], r2: [u8; HASH_SIZE]) {
            let [r1, r2] = [r1, r2].map(fr_from_hash);
            let [c0, c1, c2] = poly_from_roots(&[r1, r2]).unwrap();
            prop_assert_eq!(c2, Fr::ONE);
            prop_assert_eq!(c1, -r1 - r2);
            prop_assert_eq!(c0, r1 * r2);
        }
    }

    // TODO: move this to test_utils if needed
    fn eval_poly(coeffs: &[Fr], x: &Fr) -> Fr {
        if coeffs.is_empty() {
            panic!("Empty coeffs");
        }
        let mut coeffs = coeffs.iter().enumerate();
        // unwrap-safety: not empty() checked above
        let (_zero, constant) = coeffs.next().unwrap();
        coeffs.fold(*constant, |accum, (i, c)| {
            accum + *c * x.pow(BigInt::new([i as u64, 0, 0, 0]))
        })
    }

    proptest! {
        #[test]
        fn poly_from_roots_max_10_deg(roots in vec(any::<[u8; HASH_SIZE]>(), 1..10)) {
            let roots: Vec<_> = roots.into_iter().map(fr_from_hash).collect();
            let coeffs: [Fr; 11] = poly_from_roots(&roots).unwrap();
            for root in roots {
                prop_assert_eq!(Fr::ZERO, eval_poly(&coeffs, &root));
            }
        }
    }
}
