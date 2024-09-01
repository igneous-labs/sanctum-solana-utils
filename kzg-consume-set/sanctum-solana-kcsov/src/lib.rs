// TODO: need to figure out how to get rust-analyzer to work with target_os = "solana"
// without messing cargo up for the rest of the crates

#![cfg_attr(all(not(feature = "std"), not(test)), no_std)]

mod consts;
mod kcsc;
mod ops;
mod utils;

use utils::*;

pub use consts::*;
pub use kcsc::*;
pub use ops::*;
