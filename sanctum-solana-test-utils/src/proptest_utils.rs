use proptest::prelude::*;
use solana_program::{clock::Clock, program_option::COption, pubkey::Pubkey};

prop_compose! {
    pub fn pubkey()
        (pk: [u8; 32]) -> Pubkey {
            Pubkey::from(pk)
        }
}

prop_compose! {
    pub fn clock()
        (slot: u64, epoch_start_timestamp: i64, epoch: u64, leader_schedule_epoch: u64, unix_timestamp: i64) -> Clock {
            Clock { slot, epoch_start_timestamp, epoch, leader_schedule_epoch, unix_timestamp }
        }
}

prop_compose! {
    pub fn coption_some_pubkey()
        (pk in pubkey()) -> COption<Pubkey> {
            COption::Some(pk)
        }
}

pub fn coption_pubkey() -> impl Strategy<Value = COption<Pubkey>> {
    coption_some_pubkey()
        .boxed()
        .prop_union(Just(COption::None).boxed())
}

prop_compose! {
    pub fn coption_some_u64()
        (n: u64) -> COption<u64> {
            COption::Some(n)
        }
}

prop_compose! {
    pub fn coption_some_u64_max_exclusive(max_exclusive: u64)
        (n in 0..max_exclusive) -> COption<u64> {
            COption::Some(n)
        }
}

prop_compose! {
    pub fn coption_some_u64_max_inclusive(max_inclusive: u64)
        (n in 0..=max_inclusive) -> COption<u64> {
            COption::Some(n)
        }
}

pub fn coption_u64() -> impl Strategy<Value = COption<u64>> {
    coption_some_u64()
        .boxed()
        .prop_union(Just(COption::None).boxed())
}

pub fn coption_u64_max_exclusive(max_exclusive: u64) -> impl Strategy<Value = COption<u64>> {
    coption_some_u64_max_exclusive(max_exclusive)
        .boxed()
        .prop_union(Just(COption::None).boxed())
}

pub fn coption_u64_max_inclusive(max_inclusive: u64) -> impl Strategy<Value = COption<u64>> {
    coption_some_u64_max_inclusive(max_inclusive)
        .boxed()
        .prop_union(Just(COption::None).boxed())
}
