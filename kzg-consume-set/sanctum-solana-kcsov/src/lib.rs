// TODO: need to figure out how to get rust-analyzer to work with target_os = "solana"
// without messing cargo up for the rest of the crates

#![cfg_attr(all(not(feature = "std"), not(test)), no_std)]

mod consts;
mod kcsc;
mod ops;
mod utils;

pub use consts::*;
pub use kcsc::*;
pub use ops::*;
pub use utils::*;

#[cfg(test)]
pub mod test_utils {
    /// Copied from
    /// https://docs.rs/solana-program/latest/src/solana_program/alt_bn128/mod.rs.html#200-236
    /// for checking impls against solana's
    pub fn convert_endianness_64(bytes: &[u8]) -> Vec<u8> {
        bytes
            .chunks(32)
            .flat_map(|b| b.iter().copied().rev().collect::<Vec<u8>>())
            .collect::<Vec<u8>>()
    }
}
