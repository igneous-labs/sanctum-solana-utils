use solana_program::{
    program_error::ProgramError,
    pubkey::{Pubkey, PUBKEY_BYTES},
    stake::state::{Authorized, Delegation, Lockup, Meta, Stake},
};
use solana_readonly_account::ReadonlyAccountData;

use crate::STAKE_ACCOUNT_LEN;

pub const STAKE_STATE_UNINITIALIZED_DISCM: u8 = 0;
pub const STAKE_STATE_INITIALIZED_DISCM: u8 = 1;
pub const STAKE_STATE_STAKE_DISCM: u8 = 2;
pub const STAKE_STATE_REWARDS_POOL_DISCM: u8 = 3;

pub const STAKE_DISCM_OFFSET: usize = 0;
// meta
pub const STAKE_META_OFFSET: usize = STAKE_DISCM_OFFSET + 1;
pub const STAKE_META_RENT_EXEMPT_RESERVE_OFFSET: usize = STAKE_META_OFFSET;
// meta.authorized
pub const STAKE_META_AUTHORIZED_OFFSET: usize = STAKE_META_OFFSET + 8;
pub const STAKE_META_AUTHORIZED_STAKER_OFFSET: usize = STAKE_META_AUTHORIZED_OFFSET;
pub const STAKE_META_AUTHORIZED_WITHDRAWER_OFFSET: usize =
    STAKE_META_AUTHORIZED_STAKER_OFFSET + PUBKEY_BYTES;
// meta.lockup
pub const STAKE_META_LOCKUP_OFFSET: usize = STAKE_META_AUTHORIZED_WITHDRAWER_OFFSET + PUBKEY_BYTES;
pub const STAKE_META_LOCKUP_UNIX_TIMESTAMP_OFFSET: usize = STAKE_META_LOCKUP_OFFSET;
pub const STAKE_META_LOCKUP_EPOCH_OFFSET: usize = STAKE_META_LOCKUP_UNIX_TIMESTAMP_OFFSET + 8;
pub const STAKE_META_LOCKUP_CUSTODIAN_OFFSET: usize = STAKE_META_LOCKUP_EPOCH_OFFSET + 8;
// stake
pub const STAKE_STAKE_OFFSET: usize = STAKE_META_LOCKUP_CUSTODIAN_OFFSET + PUBKEY_BYTES;
// stake.delegation
pub const STAKE_STAKE_DELEGATION_OFFSET: usize = STAKE_STAKE_OFFSET;
pub const STAKE_STAKE_DELEGATION_VOTER_PUBKEY_OFFSET: usize = STAKE_STAKE_DELEGATION_OFFSET;
pub const STAKE_STAKE_DELEGATION_STAKE_OFFSET: usize =
    STAKE_STAKE_DELEGATION_VOTER_PUBKEY_OFFSET + PUBKEY_BYTES;
pub const STAKE_STAKE_DELEGATION_ACTIVATION_EPOCH_OFFSET: usize =
    STAKE_STAKE_DELEGATION_STAKE_OFFSET + 8;
pub const STAKE_STAKE_DELEGATION_DEACTIVATION_EPOCH_OFFSET: usize =
    STAKE_STAKE_DELEGATION_ACTIVATION_EPOCH_OFFSET + 8;
pub const STAKE_STAKE_DELEGATION_WARMUP_COOLDOWN_RATE_DEPRECATED_OFFSET: usize =
    STAKE_STAKE_DELEGATION_DEACTIVATION_EPOCH_OFFSET + 8;
pub const STAKE_STAKE_CREDITS_OBSERVED_OFFSET: usize =
    STAKE_STAKE_DELEGATION_WARMUP_COOLDOWN_RATE_DEPRECATED_OFFSET + 8;
// stakeflags
pub const STAKE_STAKE_FLAGS_OFFSET: usize = STAKE_STAKE_CREDITS_OBSERVED_OFFSET + 8;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StakeStateMarker {
    Uninitialized,
    Initialized,
    Stake,
    RewardsPool,
}

impl TryFrom<u8> for StakeStateMarker {
    type Error = ProgramError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            STAKE_STATE_UNINITIALIZED_DISCM => Ok(Self::Uninitialized),
            STAKE_STATE_INITIALIZED_DISCM => Ok(Self::Initialized),
            STAKE_STATE_STAKE_DISCM => Ok(Self::Stake),
            STAKE_STATE_REWARDS_POOL_DISCM => Ok(Self::RewardsPool),
            _ => Err(ProgramError::InvalidAccountData),
        }
    }
}

fn initialized_or_stake_checked_method<R: ReadonlyStakeAccount + ?Sized, T>(
    r: &R,
    f: fn(&R) -> T,
) -> Result<T, ProgramError> {
    match r.stake_state_marker() {
        StakeStateMarker::Initialized | StakeStateMarker::Stake => Ok(f(r)),
        StakeStateMarker::Uninitialized | StakeStateMarker::RewardsPool => {
            Err(ProgramError::InvalidAccountData)
        }
    }
}

fn stake_checked_method<R: ReadonlyStakeAccount + ?Sized, T>(
    r: &R,
    f: fn(&R) -> T,
) -> Result<T, ProgramError> {
    match r.stake_state_marker() {
        StakeStateMarker::Stake => Ok(f(r)),
        StakeStateMarker::Uninitialized
        | StakeStateMarker::Initialized
        | StakeStateMarker::RewardsPool => Err(ProgramError::InvalidAccountData),
    }
}

/// Getter methods that only deserialize the required account
/// data subslice instead of the entire account data vec.
///
/// All getter methods are unchecked and will panic if data is malfored,
/// be sure to call
/// [`ReadonlyStakeAccount::stake_data_is_valid`]
/// before calling the other methods
///
/// The `*_unchecked()` methods do not check that the stake is of the correct StakeState enum
/// before reading the bytes
pub trait ReadonlyStakeAccount {
    fn stake_data_is_valid(&self) -> bool;

    fn stake_state_marker(&self) -> StakeStateMarker;

    fn stake_meta_unchecked(&self) -> Meta;

    fn stake_meta(&self) -> Result<Meta, ProgramError> {
        initialized_or_stake_checked_method(self, Self::stake_meta_unchecked)
    }

    fn stake_meta_rent_exempt_reserve_unchecked(&self) -> u64;

    fn stake_meta_rent_exempt_reserve(&self) -> Result<u64, ProgramError> {
        initialized_or_stake_checked_method(self, Self::stake_meta_rent_exempt_reserve_unchecked)
    }

    fn stake_meta_authorized_unchecked(&self) -> Authorized;

    fn stake_meta_authorized(&self) -> Result<Authorized, ProgramError> {
        initialized_or_stake_checked_method(self, Self::stake_meta_authorized_unchecked)
    }

    fn stake_meta_authorized_staker_unchecked(&self) -> Pubkey;

    fn stake_meta_authorized_staker(&self) -> Result<Pubkey, ProgramError> {
        initialized_or_stake_checked_method(self, Self::stake_meta_authorized_staker_unchecked)
    }

    fn stake_meta_authorized_withdrawer_unchecked(&self) -> Pubkey;

    fn stake_meta_authorized_withdrawer(&self) -> Result<Pubkey, ProgramError> {
        initialized_or_stake_checked_method(self, Self::stake_meta_authorized_withdrawer_unchecked)
    }

    fn stake_meta_lockup_unchecked(&self) -> Lockup;

    fn stake_meta_lockup(&self) -> Result<Lockup, ProgramError> {
        initialized_or_stake_checked_method(self, Self::stake_meta_lockup_unchecked)
    }

    fn stake_meta_lockup_unix_timestamp_unchecked(&self) -> i64;

    fn stake_meta_lockup_unix_timestamp(&self) -> Result<i64, ProgramError> {
        initialized_or_stake_checked_method(self, Self::stake_meta_lockup_unix_timestamp_unchecked)
    }

    fn stake_meta_lockup_epoch_unchecked(&self) -> u64;

    fn stake_meta_lockup_epoch(&self) -> Result<u64, ProgramError> {
        initialized_or_stake_checked_method(self, Self::stake_meta_lockup_epoch_unchecked)
    }

    fn stake_meta_lockup_custodian_unchecked(&self) -> Pubkey;

    fn stake_meta_lockup_custodian(&self) -> Result<Pubkey, ProgramError> {
        initialized_or_stake_checked_method(self, Self::stake_meta_lockup_custodian_unchecked)
    }

    fn stake_stake_unchecked(&self) -> Stake;

    fn stake_stake(&self) -> Result<Stake, ProgramError> {
        stake_checked_method(self, Self::stake_stake_unchecked)
    }

    fn stake_stake_delegation_unchecked(&self) -> Delegation;

    fn stake_stake_delegation(&self) -> Result<Delegation, ProgramError> {
        stake_checked_method(self, Self::stake_stake_delegation_unchecked)
    }

    fn stake_stake_delegation_voter_pubkey_unchecked(&self) -> Pubkey;

    fn stake_stake_delegation_voter_pubkey(&self) -> Result<Pubkey, ProgramError> {
        stake_checked_method(self, Self::stake_stake_delegation_voter_pubkey_unchecked)
    }

    fn stake_stake_delegation_stake_unchecked(&self) -> u64;

    fn stake_stake_delegation_stake(&self) -> Result<u64, ProgramError> {
        stake_checked_method(self, Self::stake_stake_delegation_stake_unchecked)
    }

    fn stake_stake_delegation_activation_epoch_unchecked(&self) -> u64;

    fn stake_stake_delegation_activation_epoch(&self) -> Result<u64, ProgramError> {
        stake_checked_method(
            self,
            Self::stake_stake_delegation_activation_epoch_unchecked,
        )
    }

    fn stake_stake_delegation_deactivation_epoch_unchecked(&self) -> u64;

    fn stake_stake_delegation_deactivation_epoch(&self) -> Result<u64, ProgramError> {
        stake_checked_method(
            self,
            Self::stake_stake_delegation_deactivation_epoch_unchecked,
        )
    }

    fn stake_stake_delegation_warmup_cooldown_rate_deprecated_unchecked(&self) -> f64;

    fn stake_stake_delegation_warmup_cooldown_rate_deprecated(&self) -> Result<f64, ProgramError> {
        stake_checked_method(
            self,
            Self::stake_stake_delegation_warmup_cooldown_rate_deprecated_unchecked,
        )
    }

    fn stake_stake_credits_observed_unchecked(&self) -> u64;

    fn stake_stake_credits_observed(&self) -> Result<u64, ProgramError> {
        stake_checked_method(self, Self::stake_stake_credits_observed_unchecked)
    }

    fn stake_stake_flags_unchecked(&self) -> u8;

    fn stake_stake_flags(&self) -> Result<u8, ProgramError> {
        stake_checked_method(self, Self::stake_stake_flags_unchecked)
    }
}

impl<R: ReadonlyAccountData> ReadonlyStakeAccount for R {
    fn stake_data_is_valid(&self) -> bool {
        let d = self.data();
        d.len() == STAKE_ACCOUNT_LEN && StakeStateMarker::try_from(d[STAKE_DISCM_OFFSET]).is_ok()
    }

    fn stake_state_marker(&self) -> StakeStateMarker {
        StakeStateMarker::try_from(self.data()[STAKE_DISCM_OFFSET]).unwrap()
    }

    fn stake_meta_unchecked(&self) -> Meta {
        Meta {
            rent_exempt_reserve: self.stake_meta_rent_exempt_reserve_unchecked(),
            authorized: self.stake_meta_authorized_unchecked(),
            lockup: self.stake_meta_lockup_unchecked(),
        }
    }

    fn stake_meta_rent_exempt_reserve_unchecked(&self) -> u64 {
        let d = self.data();
        let b: &[u8; 8] = d
            [STAKE_META_RENT_EXEMPT_RESERVE_OFFSET..STAKE_META_RENT_EXEMPT_RESERVE_OFFSET + 8]
            .try_into()
            .unwrap();
        u64::from_le_bytes(*b)
    }

    fn stake_meta_authorized_unchecked(&self) -> Authorized {
        Authorized {
            staker: self.stake_meta_authorized_staker_unchecked(),
            withdrawer: self.stake_meta_authorized_withdrawer_unchecked(),
        }
    }

    fn stake_meta_authorized_staker_unchecked(&self) -> Pubkey {
        let d = self.data();
        let b: &[u8; 32] = d[STAKE_META_AUTHORIZED_STAKER_OFFSET
            ..STAKE_META_AUTHORIZED_STAKER_OFFSET + PUBKEY_BYTES]
            .try_into()
            .unwrap();
        Pubkey::from(*b)
    }

    fn stake_meta_authorized_withdrawer_unchecked(&self) -> Pubkey {
        let d = self.data();
        let b: &[u8; 32] = d[STAKE_META_AUTHORIZED_WITHDRAWER_OFFSET
            ..STAKE_META_AUTHORIZED_WITHDRAWER_OFFSET + PUBKEY_BYTES]
            .try_into()
            .unwrap();
        Pubkey::from(*b)
    }

    fn stake_meta_lockup_unchecked(&self) -> Lockup {
        Lockup {
            unix_timestamp: self.stake_meta_lockup_unix_timestamp_unchecked(),
            epoch: self.stake_meta_lockup_epoch_unchecked(),
            custodian: self.stake_meta_lockup_custodian_unchecked(),
        }
    }

    fn stake_meta_lockup_unix_timestamp_unchecked(&self) -> i64 {
        let d = self.data();
        let b: &[u8; 8] = d
            [STAKE_META_LOCKUP_UNIX_TIMESTAMP_OFFSET..STAKE_META_LOCKUP_UNIX_TIMESTAMP_OFFSET + 8]
            .try_into()
            .unwrap();
        i64::from_le_bytes(*b)
    }

    fn stake_meta_lockup_epoch_unchecked(&self) -> u64 {
        let d = self.data();
        let b: &[u8; 8] = d[STAKE_META_LOCKUP_EPOCH_OFFSET..STAKE_META_LOCKUP_EPOCH_OFFSET + 8]
            .try_into()
            .unwrap();
        u64::from_le_bytes(*b)
    }

    fn stake_meta_lockup_custodian_unchecked(&self) -> Pubkey {
        let d = self.data();
        let b: &[u8; 32] = d
            [STAKE_META_LOCKUP_CUSTODIAN_OFFSET..STAKE_META_LOCKUP_CUSTODIAN_OFFSET + PUBKEY_BYTES]
            .try_into()
            .unwrap();
        Pubkey::from(*b)
    }

    fn stake_stake_unchecked(&self) -> Stake {
        Stake {
            delegation: self.stake_stake_delegation_unchecked(),
            credits_observed: self.stake_stake_credits_observed_unchecked(),
        }
    }

    #[allow(deprecated)]
    fn stake_stake_delegation_unchecked(&self) -> Delegation {
        Delegation {
            voter_pubkey: self.stake_stake_delegation_voter_pubkey_unchecked(),
            stake: self.stake_stake_delegation_stake_unchecked(),
            activation_epoch: self.stake_stake_delegation_activation_epoch_unchecked(),
            deactivation_epoch: self.stake_stake_delegation_deactivation_epoch_unchecked(),
            warmup_cooldown_rate: self
                .stake_stake_delegation_warmup_cooldown_rate_deprecated_unchecked(),
        }
    }

    fn stake_stake_delegation_voter_pubkey_unchecked(&self) -> Pubkey {
        let d = self.data();
        let b: &[u8; 32] = d[STAKE_STAKE_DELEGATION_VOTER_PUBKEY_OFFSET
            ..STAKE_STAKE_DELEGATION_VOTER_PUBKEY_OFFSET + PUBKEY_BYTES]
            .try_into()
            .unwrap();
        Pubkey::from(*b)
    }

    fn stake_stake_delegation_stake_unchecked(&self) -> u64 {
        let d = self.data();
        let b: &[u8; 8] = d
            [STAKE_STAKE_DELEGATION_STAKE_OFFSET..STAKE_STAKE_DELEGATION_STAKE_OFFSET + 8]
            .try_into()
            .unwrap();
        u64::from_le_bytes(*b)
    }

    fn stake_stake_delegation_activation_epoch_unchecked(&self) -> u64 {
        let d = self.data();
        let b: &[u8; 8] = d[STAKE_STAKE_DELEGATION_ACTIVATION_EPOCH_OFFSET
            ..STAKE_STAKE_DELEGATION_ACTIVATION_EPOCH_OFFSET + 8]
            .try_into()
            .unwrap();
        u64::from_le_bytes(*b)
    }

    fn stake_stake_delegation_deactivation_epoch_unchecked(&self) -> u64 {
        let d = self.data();
        let b: &[u8; 8] = d[STAKE_STAKE_DELEGATION_DEACTIVATION_EPOCH_OFFSET
            ..STAKE_STAKE_DELEGATION_DEACTIVATION_EPOCH_OFFSET + 8]
            .try_into()
            .unwrap();
        u64::from_le_bytes(*b)
    }

    fn stake_stake_delegation_warmup_cooldown_rate_deprecated_unchecked(&self) -> f64 {
        let d = self.data();
        let b: &[u8; 8] = d[STAKE_STAKE_DELEGATION_WARMUP_COOLDOWN_RATE_DEPRECATED_OFFSET
            ..STAKE_STAKE_DELEGATION_WARMUP_COOLDOWN_RATE_DEPRECATED_OFFSET + 8]
            .try_into()
            .unwrap();
        f64::from_le_bytes(*b)
    }

    fn stake_stake_credits_observed_unchecked(&self) -> u64 {
        let d = self.data();
        let b: &[u8; 8] = d
            [STAKE_STAKE_CREDITS_OBSERVED_OFFSET..STAKE_STAKE_CREDITS_OBSERVED_OFFSET + 8]
            .try_into()
            .unwrap();
        u64::from_le_bytes(*b)
    }

    fn stake_stake_flags_unchecked(&self) -> u8 {
        self.data()[STAKE_STAKE_FLAGS_OFFSET]
    }
}
