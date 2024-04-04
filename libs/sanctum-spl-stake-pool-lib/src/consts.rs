use spl_stake_pool_interface::Fee;

pub const ZERO_FEE: Fee = Fee {
    denominator: 1,
    numerator: 0,
};

// Copied from spl-stake-pool upstream.

/// TODO: stake pool program may change parameters
pub const MIN_RESERVE_BALANCE_EXCLUDE_RENT: u64 = 0;

/// TODO: stake pool program may change parameters
/// TODO: upgrade to get_packed_len(), only available on borsh >= 1.0
pub const STAKE_POOL_SIZE: usize = 611;

/// TODO: stake pool program may change parameters
/// Minimum amount of staked lamports required in a validator stake account to
/// allow for merges without a mismatch on credits observed
pub const MIN_ACTIVE_STAKE: u64 = 1_000_000;

/// TODO: stake program may change parameters
/// if min delegation feature is activated
pub const STAKE_PROG_MIN_DELEGATION: u64 = 1;
