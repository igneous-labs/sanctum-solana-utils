#![cfg_attr(not(feature = "std"), no_std)]

mod err;
mod typedefs;
mod u64_bps_fee_ceil;
mod u64_bps_fee_floor;
mod u64_fee_ceil;
mod u64_fee_floor;
mod u64_ratio_floor;

pub use err::*;
pub use typedefs::*;
pub use u64_bps_fee_ceil::*;
pub use u64_bps_fee_floor::*;
pub use u64_fee_ceil::*;
pub use u64_fee_floor::*;
pub use u64_ratio_floor::*;

pub const BPS_DENOMINATOR: u16 = 10_000;
