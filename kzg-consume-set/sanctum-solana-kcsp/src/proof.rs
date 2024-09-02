use std::{borrow::Borrow, collections::HashSet, error::Error, fmt::Display};

use ark_bn254::{Fr, G2Projective};
use sanctum_solana_kcsc::{fr_from_hash, ToHash};

use crate::{eval_poly_pwrs_of_tau_g2, poly_from_roots};

#[derive(Debug, PartialEq, Eq)]
pub enum ProofGenErr {
    EmptyProof,
    RootNotFound,
}

impl Display for ProofGenErr {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::EmptyProof => f.write_str("EmptyProof"),
            Self::RootNotFound => f.write_str("RootNotFound"),
        }
    }
}

impl Error for ProofGenErr {}

/// For a committed polynomial `p(x)`, and divisor polynomial `z(x)` created from the roots
/// we wish to prove membership of,
/// compute the proof $\frac{p(\tau)}{z(\tau)}G2$
///
/// Args:
/// `quotient_poly_coeffs`: coefficients of the quotient polynomial $\frac{p(\x)}{z(\x)}$
#[inline]
pub fn gen_proof_for_quotient_poly_coeffs(
    quotient_poly_coeffs: impl IntoIterator<Item = impl Borrow<Fr>>,
    powers_of_tau_g2: impl IntoIterator<Item = impl Borrow<G2Projective>>,
) -> Result<G2Projective, ProofGenErr> {
    eval_poly_pwrs_of_tau_g2(quotient_poly_coeffs.into_iter().zip(powers_of_tau_g2))
        .ok_or(ProofGenErr::EmptyProof)
}

#[inline]
pub fn gen_proof_for_quotient_poly_roots(
    quotient_poly_roots: &[impl Borrow<Fr>],
    powers_of_tau_g2: impl IntoIterator<Item = impl Borrow<G2Projective>>,
) -> Result<G2Projective, ProofGenErr> {
    //let p = std::time::Instant::now();
    // TODO: poly_from_roots() takes up most of the time for large polys, need to make it faster
    let quotient_poly_coeffs = poly_from_roots(quotient_poly_roots);
    //eprintln!("poly_from_roots took: {}ms", p.elapsed().as_millis());
    gen_proof_for_quotient_poly_coeffs(quotient_poly_coeffs, powers_of_tau_g2)
}

/// Does not check if all indices in `roots_to_prove_indices` are in range,
/// ignores any that are out of range
#[inline]
pub fn quotient_poly_roots_from_indices<'a, S>(
    all_roots: impl IntoIterator<Item = S> + 'a,
    roots_to_prove_indices: &'a HashSet<usize>,
) -> impl Iterator<Item = S> + '_ {
    all_roots.into_iter().enumerate().filter_map(|(i, r)| {
        if roots_to_prove_indices.contains(&i) {
            None
        } else {
            Some(r)
        }
    })
}

#[inline]
pub fn roots_to_prove_indices(
    all_roots: &[impl Borrow<Fr>],
    roots_to_prove: impl IntoIterator<Item = impl Borrow<Fr>>,
) -> Result<HashSet<usize>, ProofGenErr> {
    let mut res = HashSet::new();
    roots_to_prove.into_iter().try_for_each(|root| {
        let i = all_roots
            .iter()
            .position(|x| x.borrow() == root.borrow())
            .ok_or(ProofGenErr::RootNotFound)?;
        res.insert(i);
        Ok(())
    })?;
    Ok(res)
}

#[inline]
pub fn gen_proof_with_roots(
    all_roots: &[impl Borrow<Fr>],
    roots_to_prove: impl IntoIterator<Item = impl Borrow<Fr>>,
    powers_of_tau_g2: impl IntoIterator<Item = impl Borrow<G2Projective>>,
) -> Result<G2Projective, ProofGenErr> {
    let indices = roots_to_prove_indices(all_roots, roots_to_prove)?;
    let quotient_poly_roots: Vec<_> = quotient_poly_roots_from_indices(all_roots, &indices)
        .map(|x| x.borrow())
        .collect();
    gen_proof_for_quotient_poly_roots(&quotient_poly_roots, powers_of_tau_g2)
}

#[inline]
pub fn gen_proof_with_all_roots_and_items_to_prove(
    all_roots: &[impl Borrow<Fr>],
    items_to_prove: impl IntoIterator<Item = impl ToHash>,
    powers_of_tau_g2: impl IntoIterator<Item = impl Borrow<G2Projective>>,
) -> Result<G2Projective, ProofGenErr> {
    gen_proof_with_roots(
        all_roots,
        items_to_prove
            .into_iter()
            .map(|x| fr_from_hash(x.to_hash())),
        powers_of_tau_g2,
    )
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use ark_ff::{BigInteger256, Field};
    use sanctum_solana_kcsc::G2_GEN;

    use super::*;

    const N: usize = 65_536;

    fn powers_of_tau_g2() -> Vec<G2Projective> {
        let tau = Fr::ONE.double();
        let mut res = Vec::with_capacity(N);
        res.push(G2Projective::from(G2_GEN));
        (1..N).fold(res, |mut res, _p| {
            res.push(*res.last().unwrap() * tau);
            res
        })
    }

    #[test]
    fn homo() {
        let p = Instant::now();
        let powers_of_tau_g2 = powers_of_tau_g2();
        eprintln!("powers_of_tau_g2 took: {}ms", p.elapsed().as_millis());
        let all_roots: Vec<_> = (0..N - 1)
            .map(|x| Fr::from(BigInteger256::from(x as u32)))
            .collect();
        let roots_to_prove = (69..169).map(|x| Fr::from(BigInteger256::from(x as u32)));
        let p = Instant::now();
        let _ = gen_proof_with_roots(&all_roots, roots_to_prove, powers_of_tau_g2).unwrap();
        eprintln!("gen_proof_with_roots took: {}ms", p.elapsed().as_millis());
    }
}
