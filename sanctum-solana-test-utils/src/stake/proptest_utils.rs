use proptest::{prelude::*, strategy::Union};
use solana_program::stake::state::{Authorized, Delegation, Lockup, Meta, Stake, StakeState};

use crate::proptest_utils::pubkey;

prop_compose! {
    pub fn authorized()
        (staker in pubkey(), withdrawer in pubkey()) -> Authorized {
            Authorized { staker, withdrawer }
        }
}

prop_compose! {
    pub fn lockup()
        (unix_timestamp: i64, epoch: u64, custodian in pubkey()) -> Lockup {
            Lockup { unix_timestamp, epoch, custodian }
        }
}

prop_compose! {
    pub fn meta()
        (rent_exempt_reserve: u64, authorized in authorized(), lockup in lockup()) -> Meta {
            Meta { rent_exempt_reserve, authorized, lockup }
        }
}

prop_compose! {
    #[allow(deprecated)]
    pub fn delegation()
        (voter_pubkey in pubkey(), stake: u64, activation_epoch: u64, deactivation_epoch: u64, warmup_cooldown_rate: f64) -> Delegation {
            Delegation { voter_pubkey, stake, activation_epoch, deactivation_epoch, warmup_cooldown_rate }
        }
}

prop_compose! {
    pub fn stake()
        (delegation in delegation(), credits_observed: u64) -> Stake {
            Stake { delegation, credits_observed }
        }
}

#[derive(Clone, Copy, Debug)]
enum StakeStateMarker {
    Uninitialized,
    Initialized,
    Stake,
    RewardsPool,
}

fn stake_state_marker() -> impl Strategy<Value = StakeStateMarker> {
    Union::new([
        Just(StakeStateMarker::Uninitialized),
        Just(StakeStateMarker::Initialized),
        Just(StakeStateMarker::Stake),
        Just(StakeStateMarker::RewardsPool),
    ])
}

// TODO: StakeStateV2 for 1.17
prop_compose! {
    #[allow(deprecated)]
    pub fn stake_state()
        (marker in stake_state_marker(), meta in meta(), stake in stake()) -> StakeState {
            match marker {
                StakeStateMarker::Uninitialized => StakeState::Uninitialized,
                StakeStateMarker::Initialized => StakeState::Initialized(meta),
                StakeStateMarker::Stake => StakeState::Stake(meta, stake),
                StakeStateMarker::RewardsPool => StakeState::RewardsPool,
            }
        }
}
