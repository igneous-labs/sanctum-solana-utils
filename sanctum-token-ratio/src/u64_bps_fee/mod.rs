//! See [`U64BpsFee`]

mod bps_fee;
mod ceil;
mod floor;

pub use bps_fee::*;
pub use ceil::*;
pub use floor::*;

pub const BPS_DENOMINATOR: u16 = 10_000;
