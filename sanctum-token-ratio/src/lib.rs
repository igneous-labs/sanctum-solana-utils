#![cfg_attr(not(feature = "std"), no_std)]

mod err;
mod traits;
mod typedefs;
mod u64_fee;
mod u64_ratio;
mod utils;
/*
mod u64_bps_fee_ceil;
mod u64_bps_fee_floor;
mod u64_fee_ceil;
mod u64_fee_floor;
mod u64_ratio_floor;
 */

pub use err::*;
pub use traits::*;
pub use typedefs::*;
pub use u64_fee::*;
pub use u64_ratio::*;
/*
pub use u64_bps_fee_ceil::*;
pub use u64_bps_fee_floor::*;
pub use u64_fee_ceil::*;
pub use u64_fee_floor::*;
pub use u64_ratio_floor::*;
 */

pub const BPS_DENOMINATOR: u16 = 10_000;
