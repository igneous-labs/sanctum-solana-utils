// The resolvers here use the old style solana-readonly-account generics as input
pub mod add_validator_to_pool;
pub mod additional_validator_stake;
pub mod cleanup_removed_validator_entries;
pub mod initialize;
pub mod remove_validator_from_pool;
pub mod set_fee;
pub mod set_funding_authority;
pub mod set_manager;
pub mod set_staker;
pub mod update_stake_pool_balance;
pub mod update_validator_list_balance;

pub use add_validator_to_pool::*;
pub use additional_validator_stake::*;
pub use cleanup_removed_validator_entries::*;
pub use initialize::*;
pub use remove_validator_from_pool::*;
pub use set_fee::*;
pub use set_funding_authority::*;
pub use set_manager::*;
pub use set_staker::*;
pub use update_stake_pool_balance::*;
pub use update_validator_list_balance::*;

// The resolvers here use the new experimental style of taking &DeserializedAccount as input
pub mod deposit_stake_with_slippage;
pub mod redelegate;
pub mod withdraw_stake_with_slippage;

pub use deposit_stake_with_slippage::*;
pub use redelegate::*;
pub use withdraw_stake_with_slippage::*;
