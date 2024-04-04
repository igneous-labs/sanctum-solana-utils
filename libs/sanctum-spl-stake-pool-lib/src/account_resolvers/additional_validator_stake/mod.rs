use std::num::NonZeroU32;

use solana_program::pubkey::Pubkey;

mod decrease;
mod increase;

pub use decrease::*;
pub use increase::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AdditionalValidatorStakePdas {
    pub withdraw_authority: Pubkey,
    pub validator_stake_account: Pubkey,
    pub transient_stake_account: Pubkey,
    pub ephemeral_stake_account: Pubkey,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AdditionalValidatorStakeSeeds {
    pub validator: Option<NonZeroU32>,
    pub transient: u64,
    pub ephemeral: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProgramIdAndVote {
    pub program_id: Pubkey,
    pub vote_account: Pubkey,
}
