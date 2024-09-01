//! Conversion utilities between ark types and big-endian representations for use onchain

use ark_bn254::{Fr, G1Affine, G2Affine};
use ark_ff::{BigInteger256, PrimeField};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize, SerializationError};

/// Size of scalar field in bytes
pub const FR: usize = 32;

/// Size of packed uncompressed affine G1 point (x, y) in bytes
pub const G1: usize = 64;

/// Size of packed uncompressed affine G2 point (x, y) in bytes
pub const G2: usize = 128;

/// Convert a [`ark_bn254::Fr`] to a big endian U256,
/// the form the solana syscalls expect.
///
/// Inverse of [`be_to_fr`]
///
/// ## Notes
///
/// - Like all [`PrimeField`]s in `ark`, [`ark_bn254::Fr`] is in Montgomery form, but serialized
///   as a BigInt little endian by calling [`PrimeField::into_bigint()`] first.
///   E.g. the internal bytes of `2` is 4 huge u64s, but is serialized as `[2u8, 0u8, ..., 0u8]`
///  
/// - This fn was created bec the `solana_program` implementation `convert_endianness_64` has an unnecessary `Vec` allocation
#[inline]
pub fn fr_to_be(fr: &Fr) -> [u8; FR] {
    let mut res = [0u8; FR];
    let bi = fr.into_bigint(); // this would be a const fn if into_bigint() was const
    for i in 0..4 {
        let s = i * 8;
        res[s..s + 8].copy_from_slice(&bi.0[3 - i].to_be_bytes());
    }
    res
}

/// Convert a big endian U256 into a [`ark_bn254::Fr`].
///
/// Inverse of [`fr_to_be`]
///
/// TODO: verify that this mods prime modulus if U256 is out of range
#[inline]
pub fn be_to_fr(be: &[u8; FR]) -> Fr {
    let x: &[[u8; 8]; 4] = unsafe { core::mem::transmute(be) };
    let mut limbs = x.map(u64::from_be_bytes);
    limbs.reverse();
    BigInteger256::new(limbs).into()
}

#[inline]
pub fn g1_to_be(g1: &G1Affine) -> [u8; G1] {
    curve_to_be(g1)
}

#[inline]
pub fn be_to_g1(be: [u8; G1]) -> Result<G1Affine, SerializationError> {
    be_to_curve(be)
}

#[inline]
pub fn g2_to_be(g2: &G2Affine) -> [u8; G2] {
    curve_to_be(g2)
}

#[inline]
pub fn be_to_g2(be: [u8; G2]) -> Result<G2Affine, SerializationError> {
    be_to_curve(be)
}

#[inline]
fn curve_to_be<C: CanonicalSerialize, const N: usize>(c: &C) -> [u8; N] {
    let mut res = [0u8; N];
    // unwrap-safety: make sure N is of the correct dimension
    c.serialize_uncompressed(res.as_mut()).unwrap();
    res[..N / 2].reverse();
    res[N / 2..].reverse();
    res
}

/// Errors if provided bytes represent an invalid curve point
#[inline]
fn be_to_curve<C: CanonicalDeserialize, const N: usize>(
    mut be: [u8; N],
) -> Result<C, SerializationError> {
    be[..N / 2].reverse();
    be[N / 2..].reverse();
    C::deserialize_uncompressed(be.as_slice())
}

#[cfg(test)]
mod tests {
    use ark_serialize::CanonicalSerialize;
    use proptest::prelude::*;

    use crate::{fr_from_hash, G1_GEN, G2_GEN};

    use super::*;

    /// Copied from
    /// https://docs.rs/solana-program/latest/src/solana_program/alt_bn128/mod.rs.html#200-236
    /// for checking impls against solana's
    pub fn convert_endianness_64(bytes: &[u8]) -> Vec<u8> {
        bytes
            .chunks(32)
            .flat_map(|b| b.iter().copied().rev().collect::<Vec<u8>>())
            .collect::<Vec<u8>>()
    }

    proptest! {
        #[test]
        fn fr_to_be_matches_solana_impl(rand_bytes: [u8; FR]) {
            let fr = fr_from_hash(rand_bytes);

            let mut expected_bytes = vec![];
            fr.serialize_uncompressed(&mut expected_bytes).unwrap();
            let expected_bytes = convert_endianness_64(&expected_bytes);

            let out = fr_to_be(&fr);
            prop_assert_eq!(out.as_slice(), expected_bytes.as_slice());
        }
    }

    proptest! {
        #[test]
        fn fr_to_be_roundtrip(rand_bytes: [u8; FR]) {
            let fr = fr_from_hash(rand_bytes);
            assert_eq!(fr, be_to_fr(&fr_to_be(&fr)));
        }
    }

    proptest! {
        #[test]
        fn g1_to_be_roundtrip(rand_multiplier_bytes: [u8; FR]) {
            let g1 = G1_GEN * fr_from_hash(rand_multiplier_bytes);
            assert_eq!(g1, be_to_g1(g1_to_be(&g1.into())).unwrap());
        }
    }

    proptest! {
        #[test]
        fn g2_to_be_roundtrip(rand_multiplier_bytes: [u8; FR]) {
            let g2 = G2_GEN * fr_from_hash(rand_multiplier_bytes);
            assert_eq!(g2, be_to_g2(g2_to_be(&g2.into())).unwrap());
        }
    }
}
