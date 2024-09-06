use ark_bn254::Fr;
use ark_ff::BigInt;

pub const HASH_SIZE: usize = 32;

pub trait ToHash {
    fn to_hash(&self) -> [u8; HASH_SIZE];
}

impl<T> ToHash for &T
where
    T: ToHash + ?Sized,
{
    #[inline]
    fn to_hash(&self) -> [u8; HASH_SIZE] {
        (*self).to_hash()
    }
}

/// Converts a 256-bit cryptographic hash into a field element of bn254's scalar prime field
/// by zero-ing out the 3 high bits and interpreting it as a little-endian 253-bit number
///
/// Details:
/// - TODO: need to confirm that zeroing out 3 high bits does not weaken
///   wtv 256-bit crypto hashing algo's collision resistance or bias fatally
/// - BigInt is backed by 4 u64s in little-endian order i.e. `0x00..01`'s repr is `[1u64, 0, 0, 0]`
/// - Fr is Montgomery form of inner BigInt
/// - this fn takes ~2500 CUs onchain
#[inline]
pub const fn fr_from_hash(hash: [u8; HASH_SIZE]) -> Fr {
    let [u0, u1, u2, mut u3]: [[u8; 8]; 4] = unsafe { core::mem::transmute(hash) };
    let u0 = u64::from_le_bytes(u0);
    let u1 = u64::from_le_bytes(u1);
    let u2 = u64::from_le_bytes(u2);
    // zero-out high 3 bits
    u3[7] &= 0b0001_1111;
    let u3 = u64::from_le_bytes(u3);
    Fr::new(BigInt::new([u0, u1, u2, u3]))
}