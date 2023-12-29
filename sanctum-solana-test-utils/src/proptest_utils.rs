use proptest::prelude::*;
use solana_program::{clock::Clock, pubkey::Pubkey};

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
