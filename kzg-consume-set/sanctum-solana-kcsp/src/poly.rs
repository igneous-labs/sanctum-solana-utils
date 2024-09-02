use std::borrow::Borrow;

use ark_bn254::Fr;
use ark_ff::Field;

/// Given all the roots of a polynomial, output the coefficient array in ascending powers.
///
/// Returns an empty vec if roots are empty.
///
/// Difference with onchain vers is this returns a vec instead of a const generic array
#[inline]
pub fn poly_from_roots<S: Borrow<Fr>>(roots: &[S]) -> Vec<Fr> {
    // this just does the naive n^2 thing of multiplying everything out
    let d = roots.len();
    if d == 0 {
        return Vec::new();
    }

    let mut res = vec![Fr::ZERO; d + 1];

    // highest power coeff is always 1
    let mut n = d;
    res[n] = Fr::ONE;

    for root in roots.iter() {
        n -= 1;
        for j in n..d {
            let sub = res[j + 1] * root.borrow();
            res[j] -= sub;
        }
    }

    res
}

#[cfg(test)]
mod poly_from_roots_tests {
    use ark_ff::BigInt;
    use proptest::{collection::vec, prelude::*};
    use sanctum_solana_kcsc::{fr_from_hash, HASH_SIZE};

    use super::*;

    // this just does the naive n^2 thing of multiplying everything out
    // used to check fft implementation
    fn poly_from_roots_brute_force<S: Borrow<Fr>>(roots: &[S]) -> Vec<Fr> {
        let d = roots.len();
        if d == 0 {
            return Vec::new();
        }

        let mut res = vec![Fr::ZERO; d + 1];

        // highest power coeff is always 1
        let mut n = d;
        res[n] = Fr::ONE;

        for root in roots.iter() {
            n -= 1;
            for j in n..d {
                let sub = res[j + 1] * root.borrow();
                res[j] -= sub;
            }
        }

        res
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
            let coeffs = poly_from_roots(&roots);
            let expected_coeffs = poly_from_roots_brute_force(&roots);
            prop_assert_eq!(&coeffs, &expected_coeffs);
            for root in roots {
                prop_assert_eq!(Fr::ZERO, eval_poly(&coeffs, &root));
            }
        }
    }
}
