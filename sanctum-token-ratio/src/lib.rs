#![cfg_attr(not(feature = "std"), no_std)]

mod err;
mod traits;
mod typedefs;
mod u64_bps_fee;
mod u64_fee_ratio;
mod u64_ratio;
mod utils;

pub use err::*;
pub use traits::*;
pub use typedefs::*;
pub use u64_bps_fee::*;
pub use u64_fee_ratio::*;
pub use u64_ratio::*;

pub const BPS_DENOMINATOR: u16 = 10_000;
