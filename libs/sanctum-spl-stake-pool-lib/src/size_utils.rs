//! Mostly copied from upstream

use solana_program::{rent::Rent, stake::state::StakeStateV2};

use crate::{MIN_ACTIVE_STAKE, MIN_RESERVE_BALANCE_EXCLUDE_RENT, STAKE_PROG_MIN_DELEGATION};

/// Get the minimum delegation required by a stake account in a stake pool
pub const fn min_delegation() -> u64 {
    if STAKE_PROG_MIN_DELEGATION > MIN_ACTIVE_STAKE {
        STAKE_PROG_MIN_DELEGATION
    } else {
        MIN_ACTIVE_STAKE
    }
}

pub fn min_reserve_lamports(rent: &Rent) -> u64 {
    rent.minimum_balance(std::mem::size_of::<StakeStateV2>())
        .saturating_add(MIN_RESERVE_BALANCE_EXCLUDE_RENT)
}

/// Returns lamports required to be transferred from the pool's
/// reserve to create a new validator stake account to add a validator to the pool
pub fn lamports_for_new_vsa(rent: &Rent) -> u64 {
    rent.minimum_balance(std::mem::size_of::<StakeStateV2>())
        .saturating_add(min_delegation())
}
