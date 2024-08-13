//! Reimplementation of arithmetic ops in solana_program::alt_bn128
//! with some optimizations:
//! - avoid all unnecessary `Vec` allocations.
//! - allow syscalls to operate on buffers by ref

mod g1_add;
mod g1_scalar_mul;
mod kcsc_compress;
mod kcsc_decompress;
mod pairing;

pub use g1_add::*;
pub use g1_scalar_mul::*;
pub use kcsc_compress::*;
pub use kcsc_decompress::*;
pub use pairing::*;
