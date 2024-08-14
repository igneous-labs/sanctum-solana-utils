use ark_bn254::Fr;
use ark_ff::PrimeField;

use crate::FR;

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
pub fn fr_to_be(fr: &Fr) -> [u8; FR] {
    let mut res = [0u8; FR];
    let bi = fr.into_bigint();
    for i in 0..4 {
        let s = i * 8;
        res[s..s + 8].copy_from_slice(&bi.0[3 - i].to_be_bytes());
    }
    res
}

/// Panics on overflow due to unchecked arithmetic
#[inline]
pub(crate) const fn u256_be_sub(x: &[u8; 32], y: &[u8; 32]) -> [u8; 32] {
    let [x_hi, x_lo]: &[[u8; 16]; 2] = unsafe { core::mem::transmute(x) };
    let x_hi = u128::from_be_bytes(*x_hi);
    let x_lo = u128::from_be_bytes(*x_lo);

    let [y_hi, y_lo]: &[[u8; 16]; 2] = unsafe { core::mem::transmute(y) };
    let y_hi = u128::from_be_bytes(*y_hi);
    let y_lo = u128::from_be_bytes(*y_lo);

    let y_hi = if y_lo > x_lo { y_hi + 1 } else { y_hi };

    let lo = x_lo.wrapping_sub(y_lo);
    let hi = x_hi - y_hi;

    let res: [[u8; 16]; 2] = [hi.to_be_bytes(), lo.to_be_bytes()];

    unsafe { core::mem::transmute(res) }
}

#[cfg(test)]
mod tests {
    use ark_ff::{BigInteger, BigInteger256};
    use ark_serialize::CanonicalSerialize;
    use proptest::{prop_assert_eq, proptest};
    use sanctum_solana_kcsc::fr_from_hash;

    use crate::test_utils::convert_endianness_64;

    use super::*;

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

    fn be_to_bi(x: [u8; 32]) -> BigInteger256 {
        let x: [[u8; 8]; 4] = unsafe { core::mem::transmute(x) };
        let mut limbs = x.map(u64::from_be_bytes);
        limbs.reverse();
        BigInteger256::new(limbs)
    }

    proptest! {
        #[test]
        fn u256_be_sub_impl(x: [u8; 32], y: [u8; 32]) {
            let [x_bi, y_bi] = [x, y].map(be_to_bi);
            let [(x, x_bi), (y, y_bi)] = if x_bi < y_bi {
                [(y, y_bi), (x, x_bi)]
            } else {
                [(x, x_bi), (y, y_bi)]
            };

            let mut expected = x_bi;
            expected.sub_with_borrow(&y_bi);

            let actual = u256_be_sub(&x, &y);
            prop_assert_eq!(expected, be_to_bi(actual));
        }
    }
}
