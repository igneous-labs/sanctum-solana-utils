use sanctum_solana_kcsc::{ToHash, HASH_SIZE};
use solana_program::hash::hashv;

/// A type that's represented by a byte buffer
/// can be easily hashed by just hashing the underlying byte buffer.
pub struct ByteBuf<T>(pub T);

impl<T> ToHash for ByteBuf<T>
where
    T: AsRef<[u8]>,
{
    #[inline]
    fn to_hash(&self) -> [u8; HASH_SIZE] {
        hashv(&[self.0.as_ref()]).to_bytes()
    }
}
