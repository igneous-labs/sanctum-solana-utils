/// Panics on overflow due to unchecked arithmetic
///
/// reimplementation as const fn to keep [`crate::AltBn128G1G2PairingEqCheck::with_g1a`] `const`
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
    use proptest::{prop_assert_eq, proptest};

    use super::*;

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
