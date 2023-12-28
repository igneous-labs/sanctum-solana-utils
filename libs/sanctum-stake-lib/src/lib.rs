mod utils;

pub use utils::*;

// This const is available in StakeState::size_of() and StakeStateV2::size_of(),
// but 1.17 will make using StakeState::size_of() a deprecation warning,
// so just define it here to make it independent
pub const STAKE_ACCOUNT_LEN: usize = 200;
