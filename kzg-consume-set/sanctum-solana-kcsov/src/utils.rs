use ark_bn254::Fr;
use ark_ff::PrimeField;
use solana_program::alt_bn128::prelude::ALT_BN128_FIELD_SIZE;

/// Convert a [`ark_bn254::Fr`] to a big endian U256,
/// the form the solana syscalls expect.
///
/// ## Notes
///
/// - Like all [`PrimeField`]s in `ark`, [`ark_bn254::Fr`] is in Montgomery form, but serialized
///   as a BigInt little endian by calling [`PrimeField::into_bigint()`] first.
///    E.g. the internal bytes of `2` is 4 huge u64s, but is serialized as `[2u8, 0u8, ..., 0u8]`
///  
/// - This fn was created bec the `solana_program` implementation `convert_endianness_64` has an unnecessary `Vec` allocation
#[inline]
pub fn fr_to_be(fr: &Fr) -> [u8; ALT_BN128_FIELD_SIZE] {
    let mut res = [0u8; ALT_BN128_FIELD_SIZE];
    let bi = fr.into_bigint();
    for i in 0..4 {
        let s = i * 8;
        res[s..s + 8].copy_from_slice(&bi.0[3 - i].to_be_bytes());
    }
    res
}

#[cfg(test)]
mod tests {
    use ark_serialize::CanonicalSerialize;
    use proptest::{prop_assert_eq, proptest};
    use sanctum_solana_kcsc::fr_from_hash;

    use crate::test_utils::convert_endianness_64;

    use super::*;

    proptest! {
        #[test]
        fn fr_to_be_matches_solana_impl(rand_bytes: [u8; ALT_BN128_FIELD_SIZE]) {
            let fr = fr_from_hash(rand_bytes);

            let mut expected_bytes = vec![];
            fr.serialize_uncompressed(&mut expected_bytes).unwrap();
            let expected_bytes = convert_endianness_64(&expected_bytes);

            let out = fr_to_be(&fr);
            prop_assert_eq!(out.as_slice(), expected_bytes.as_slice());
        }
    }
}
