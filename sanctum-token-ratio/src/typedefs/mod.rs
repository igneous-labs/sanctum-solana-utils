//! TOOD: add proptests to check documented invariants

// inline all simple functions so that they can be inlined by consumers.
// dont need to do the same for generic fns and methods on generic structs since those
// are available to be inlined by consumers. TODO: confirm this

mod amts_after_fee;
mod div;
mod u64_value_range;

pub use amts_after_fee::*;
pub use div::*;
pub use u64_value_range::*;
