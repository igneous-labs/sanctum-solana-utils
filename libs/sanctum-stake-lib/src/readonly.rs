use solana_program::{
    clock::Clock,
    program_error::ProgramError,
    pubkey::{Pubkey, PUBKEY_BYTES},
    stake::{
        self,
        state::{Authorized, Delegation, Lockup, Meta},
    },
};
use solana_readonly_account::ReadonlyAccountData;

use crate::STAKE_ACCOUNT_LEN;

pub const STAKE_STATE_UNINITIALIZED_DISCM: [u8; 4] = 0u32.to_le_bytes();
pub const STAKE_STATE_INITIALIZED_DISCM: [u8; 4] = 1u32.to_le_bytes();
pub const STAKE_STATE_STAKE_DISCM: [u8; 4] = 2u32.to_le_bytes();
pub const STAKE_STATE_REWARDS_POOL_DISCM: [u8; 4] = 3u32.to_le_bytes();

pub const STAKE_DISCM_OFFSET: usize = 0;
// meta
pub const STAKE_META_OFFSET: usize = STAKE_DISCM_OFFSET + 4; // StakeState serializes the discriminant as a u32
pub const STAKE_META_RENT_EXEMPT_RESERVE_OFFSET: usize = STAKE_META_OFFSET;
// meta.authorized
pub const STAKE_META_AUTHORIZED_OFFSET: usize = STAKE_META_RENT_EXEMPT_RESERVE_OFFSET + 8;
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

/// A possible stake account
///
/// ## Example
///
/// ```rust
/// use sanctum_stake_lib::ReadonlyStakeAccount;
/// use solana_program::{
///     account_info::AccountInfo,
///     entrypoint::ProgramResult
/// };
///
/// pub fn process(account: &AccountInfo) -> ProgramResult {
///     let account = ReadonlyStakeAccount(account);
///     let account = account.try_into_valid()?;
///     let account = account.try_into_stake()?;
///     solana_program::msg!("{}", account.stake_stake_credits_observed());
///     Ok(())
/// }
/// ```
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ReadonlyStakeAccount<T>(pub T);

impl<T> ReadonlyStakeAccount<T> {
    pub fn as_inner(&self) -> &T {
        &self.0
    }

    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T: ReadonlyAccountData> ReadonlyStakeAccount<T> {
    pub fn stake_data_is_valid(&self) -> bool {
        let d = self.0.data();
        if d.len() != STAKE_ACCOUNT_LEN {
            return false;
        }
        let b: &[u8; 4] = d[STAKE_DISCM_OFFSET..STAKE_DISCM_OFFSET + 4]
            .try_into()
            .unwrap();
        StakeStateMarker::try_from(*b).is_ok()
    }

    pub fn try_into_valid(self) -> Result<ValidStakeAccount<T>, ProgramError> {
        if !self.stake_data_is_valid() {
            return Err(ProgramError::InvalidAccountData);
        }
        Ok(ValidStakeAccount(self))
    }
}

impl<T> AsRef<T> for ReadonlyStakeAccount<T> {
    fn as_ref(&self) -> &T {
        self.as_inner()
    }
}

// can't impl From<ReadonlyStakeAccount<T>> for T due to orphan rules

/// A stake account that has been checked to contain valid data.
///
/// The only safe way to create this struct is via [`TryFrom<ReadonlyStakeAccount>`]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ValidStakeAccount<T>(ReadonlyStakeAccount<T>);

impl<T> ValidStakeAccount<T> {
    pub fn as_readonly(&self) -> &ReadonlyStakeAccount<T> {
        &self.0
    }

    pub fn into_readonly(self) -> ReadonlyStakeAccount<T> {
        self.0
    }
}

impl<T: ReadonlyAccountData> ValidStakeAccount<T> {
    pub fn stake_state_marker(&self) -> StakeStateMarker {
        let d = self.0.as_inner().data();
        let b: &[u8; 4] = d[STAKE_DISCM_OFFSET..STAKE_DISCM_OFFSET + 4]
            .try_into()
            .unwrap();
        StakeStateMarker::try_from(*b).unwrap()
    }

    pub fn try_into_stake_or_initialized(
        self,
    ) -> Result<StakeOrInitializedStakeAccount<T>, ProgramError> {
        match self.stake_state_marker() {
            StakeStateMarker::Initialized | StakeStateMarker::Stake => {
                Ok(StakeOrInitializedStakeAccount(self))
            }
            StakeStateMarker::Uninitialized | StakeStateMarker::RewardsPool => {
                Err(ProgramError::InvalidAccountData)
            }
        }
    }

    pub fn try_into_stake(self) -> Result<StakeStakeAccount<T>, ProgramError> {
        if let StakeStateMarker::Stake = self.stake_state_marker() {
            return Ok(StakeStakeAccount(
                self.try_into_stake_or_initialized().unwrap(),
            ));
        }
        Err(ProgramError::InvalidAccountData)
    }
}

impl<T: ReadonlyAccountData> TryFrom<ReadonlyStakeAccount<T>> for ValidStakeAccount<T> {
    type Error = ProgramError;

    fn try_from(value: ReadonlyStakeAccount<T>) -> Result<Self, Self::Error> {
        value.try_into_valid()
    }
}

impl<T> AsRef<ReadonlyStakeAccount<T>> for ValidStakeAccount<T> {
    fn as_ref(&self) -> &ReadonlyStakeAccount<T> {
        self.as_readonly()
    }
}

impl<T> From<ValidStakeAccount<T>> for ReadonlyStakeAccount<T> {
    fn from(value: ValidStakeAccount<T>) -> Self {
        value.into_readonly()
    }
}

/// A stake account that has been checked to be in the `Initialized` or `Stake` state.
///
/// The only safe way to create this struct is via [`TryFrom<Valid<T>>`]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StakeOrInitializedStakeAccount<T>(ValidStakeAccount<T>);

impl<T> StakeOrInitializedStakeAccount<T> {
    pub fn as_valid(&self) -> &ValidStakeAccount<T> {
        &self.0
    }

    pub fn into_valid(self) -> ValidStakeAccount<T> {
        self.0
    }
}

impl<T: ReadonlyAccountData> StakeOrInitializedStakeAccount<T> {
    pub fn stake_meta(&self) -> Meta {
        Meta {
            rent_exempt_reserve: self.stake_meta_rent_exempt_reserve(),
            authorized: self.stake_meta_authorized(),
            lockup: self.stake_meta_lockup(),
        }
    }

    pub fn stake_meta_rent_exempt_reserve(&self) -> u64 {
        deser_u64_le_unchecked(
            self.0.as_readonly().as_inner(),
            STAKE_META_RENT_EXEMPT_RESERVE_OFFSET,
        )
    }

    pub fn stake_meta_authorized(&self) -> Authorized {
        Authorized {
            staker: self.stake_meta_authorized_staker(),
            withdrawer: self.stake_meta_authorized_withdrawer(),
        }
    }

    pub fn stake_meta_authorized_staker(&self) -> Pubkey {
        deser_pubkey_unchecked(
            self.0.as_readonly().as_inner(),
            STAKE_META_AUTHORIZED_STAKER_OFFSET,
        )
    }

    pub fn stake_meta_authorized_withdrawer(&self) -> Pubkey {
        deser_pubkey_unchecked(
            self.0.as_readonly().as_inner(),
            STAKE_META_AUTHORIZED_WITHDRAWER_OFFSET,
        )
    }

    pub fn stake_meta_lockup(&self) -> Lockup {
        Lockup {
            unix_timestamp: self.stake_meta_lockup_unix_timestamp(),
            epoch: self.stake_meta_lockup_epoch(),
            custodian: self.stake_meta_lockup_custodian(),
        }
    }

    pub fn stake_meta_lockup_unix_timestamp(&self) -> i64 {
        deser_i64_le_unchecked(
            self.0.as_readonly().as_inner(),
            STAKE_META_LOCKUP_UNIX_TIMESTAMP_OFFSET,
        )
    }

    pub fn stake_meta_lockup_epoch(&self) -> u64 {
        deser_u64_le_unchecked(
            self.0.as_readonly().as_inner(),
            STAKE_META_LOCKUP_EPOCH_OFFSET,
        )
    }

    pub fn stake_meta_lockup_custodian(&self) -> Pubkey {
        deser_pubkey_unchecked(
            self.0.as_readonly().as_inner(),
            STAKE_META_LOCKUP_CUSTODIAN_OFFSET,
        )
    }

    /// The original solana-program API takes an optional custodian pubkey arg and returns true
    /// if thats equal to lockup.custodian, which doesn't really make any sense
    pub fn stake_lockup_is_in_force(&self, clock: &Clock) -> bool {
        self.stake_meta_lockup_unix_timestamp() > clock.unix_timestamp
            || self.stake_meta_lockup_epoch() > clock.epoch
    }

    pub fn try_into_stake(self) -> Result<StakeStakeAccount<T>, ProgramError> {
        self.into_valid().try_into()
    }
}

impl<T: ReadonlyAccountData> TryFrom<ValidStakeAccount<T>> for StakeOrInitializedStakeAccount<T> {
    type Error = ProgramError;

    fn try_from(value: ValidStakeAccount<T>) -> Result<Self, Self::Error> {
        value.try_into_stake_or_initialized()
    }
}

impl<T> AsRef<ValidStakeAccount<T>> for StakeOrInitializedStakeAccount<T> {
    fn as_ref(&self) -> &ValidStakeAccount<T> {
        self.as_valid()
    }
}

impl<T> From<StakeOrInitializedStakeAccount<T>> for ValidStakeAccount<T> {
    fn from(value: StakeOrInitializedStakeAccount<T>) -> Self {
        value.into_valid()
    }
}

/// A stake account that has been checked to be in the `Stake` state.
///
/// The only safe way to create this struct is via [`TryFrom<Valid<T>>`]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StakeStakeAccount<T>(StakeOrInitializedStakeAccount<T>);

impl<T> StakeStakeAccount<T> {
    pub fn as_stake_or_initialized(&self) -> &StakeOrInitializedStakeAccount<T> {
        &self.0
    }

    pub fn into_stake_or_initialized(self) -> StakeOrInitializedStakeAccount<T> {
        self.0
    }
}

impl<T: ReadonlyAccountData> StakeStakeAccount<T> {
    pub fn stake_stake(&self) -> stake::state::Stake {
        stake::state::Stake {
            delegation: self.stake_stake_delegation(),
            credits_observed: self.stake_stake_credits_observed(),
        }
    }

    #[allow(deprecated)]
    pub fn stake_stake_delegation(&self) -> Delegation {
        Delegation {
            voter_pubkey: self.stake_stake_delegation_voter_pubkey(),
            stake: self.stake_stake_delegation_stake(),
            activation_epoch: self.stake_stake_delegation_activation_epoch(),
            deactivation_epoch: self.stake_stake_delegation_deactivation_epoch(),
            warmup_cooldown_rate: self.stake_stake_delegation_warmup_cooldown_rate_deprecated(),
        }
    }

    pub fn stake_stake_delegation_voter_pubkey(&self) -> Pubkey {
        deser_pubkey_unchecked(
            self.0.as_valid().as_readonly().as_inner(),
            STAKE_STAKE_DELEGATION_VOTER_PUBKEY_OFFSET,
        )
    }

    pub fn stake_stake_delegation_stake(&self) -> u64 {
        deser_u64_le_unchecked(
            self.0.as_valid().as_readonly().as_inner(),
            STAKE_STAKE_DELEGATION_STAKE_OFFSET,
        )
    }

    pub fn stake_stake_delegation_activation_epoch(&self) -> u64 {
        deser_u64_le_unchecked(
            self.0.as_valid().as_readonly().as_inner(),
            STAKE_STAKE_DELEGATION_ACTIVATION_EPOCH_OFFSET,
        )
    }

    pub fn stake_stake_delegation_deactivation_epoch(&self) -> u64 {
        deser_u64_le_unchecked(
            self.0.as_valid().as_readonly().as_inner(),
            STAKE_STAKE_DELEGATION_DEACTIVATION_EPOCH_OFFSET,
        )
    }

    pub fn stake_stake_delegation_warmup_cooldown_rate_deprecated(&self) -> f64 {
        deser_f64_le_unchecked(
            self.0.as_valid().as_readonly().as_inner(),
            STAKE_STAKE_DELEGATION_WARMUP_COOLDOWN_RATE_DEPRECATED_OFFSET,
        )
    }

    pub fn stake_stake_credits_observed(&self) -> u64 {
        deser_u64_le_unchecked(
            self.0.as_valid().as_readonly().as_inner(),
            STAKE_STAKE_CREDITS_OBSERVED_OFFSET,
        )
    }

    // TODO: add tests for 1.17
    pub fn stake_stake_flags(&self) -> u8 {
        let d = self.0.as_valid().as_readonly().as_inner().data();
        d[STAKE_STAKE_FLAGS_OFFSET]
    }

    pub fn stake_is_bootstrap(&self) -> bool {
        self.stake_stake_delegation_activation_epoch() == u64::MAX
    }
}

impl<T: ReadonlyAccountData> TryFrom<ValidStakeAccount<T>> for StakeStakeAccount<T> {
    type Error = ProgramError;

    fn try_from(value: ValidStakeAccount<T>) -> Result<Self, Self::Error> {
        value.try_into_stake()
    }
}

impl<T: ReadonlyAccountData> TryFrom<StakeOrInitializedStakeAccount<T>> for StakeStakeAccount<T> {
    type Error = ProgramError;

    fn try_from(value: StakeOrInitializedStakeAccount<T>) -> Result<Self, Self::Error> {
        value.try_into_stake()
    }
}

impl<T> AsRef<StakeOrInitializedStakeAccount<T>> for StakeStakeAccount<T> {
    fn as_ref(&self) -> &StakeOrInitializedStakeAccount<T> {
        self.as_stake_or_initialized()
    }
}

impl<T> From<StakeStakeAccount<T>> for StakeOrInitializedStakeAccount<T> {
    fn from(value: StakeStakeAccount<T>) -> Self {
        value.into_stake_or_initialized()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum StakeStateMarker {
    Uninitialized,
    Initialized,
    Stake,
    RewardsPool,
}

impl TryFrom<[u8; 4]> for StakeStateMarker {
    type Error = ProgramError;

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        match value {
            STAKE_STATE_UNINITIALIZED_DISCM => Ok(Self::Uninitialized),
            STAKE_STATE_INITIALIZED_DISCM => Ok(Self::Initialized),
            STAKE_STATE_STAKE_DISCM => Ok(Self::Stake),
            STAKE_STATE_REWARDS_POOL_DISCM => Ok(Self::RewardsPool),
            _ => Err(ProgramError::InvalidAccountData),
        }
    }
}

fn deser_pubkey_unchecked<D: ReadonlyAccountData>(d: D, offset: usize) -> Pubkey {
    let d = d.data();
    let b: &[u8; 32] = d[offset..offset + PUBKEY_BYTES].try_into().unwrap();
    Pubkey::from(*b)
}

fn deser_u64_le_unchecked<D: ReadonlyAccountData>(d: D, offset: usize) -> u64 {
    let d = d.data();
    let b: &[u8; 8] = d[offset..offset + 8].try_into().unwrap();
    u64::from_le_bytes(*b)
}

fn deser_i64_le_unchecked<D: ReadonlyAccountData>(d: D, offset: usize) -> i64 {
    let d = d.data();
    let b: &[u8; 8] = d[offset..offset + 8].try_into().unwrap();
    i64::from_le_bytes(*b)
}

fn deser_f64_le_unchecked<D: ReadonlyAccountData>(d: D, offset: usize) -> f64 {
    let d = d.data();
    let b: &[u8; 8] = d[offset..offset + 8].try_into().unwrap();
    f64::from_le_bytes(*b)
}

#[cfg(test)]
mod tests {
    use borsh::{BorshDeserialize, BorshSerialize};
    use proptest::prelude::*;
    use sanctum_solana_test_utils::{proptest_utils::clock, stake::proptest_utils::stake_state};
    use solana_program::stake::state::StakeState;

    use super::*;

    struct AccountData<'a>(pub &'a [u8]);

    impl<'a> ReadonlyAccountData for AccountData<'a> {
        type SliceDeref<'s> = &'s [u8]
        where
            Self: 's;

        type DataDeref<'d> = &'d &'d [u8]
        where
            Self: 'd;

        fn data(&self) -> Self::DataDeref<'_> {
            &self.0
        }
    }

    proptest! {
        #[test]
        fn stake_readonly_matches_full_deser_invalid(data: [u8; STAKE_ACCOUNT_LEN]) {
            let account = ReadonlyStakeAccount(AccountData(&data));
            let unpack_res = StakeState::deserialize(&mut data.as_ref());
            if !account.stake_data_is_valid() {
                prop_assert!(unpack_res.is_err());
            }
        }
    }

    fn assert_meta_eq(
        actual: &StakeOrInitializedStakeAccount<AccountData>,
        expected: &Meta,
        clock: &Clock,
    ) -> Result<(), TestCaseError> {
        prop_assert_eq!(actual.stake_meta(), *expected);
        prop_assert_eq!(
            actual.stake_meta_rent_exempt_reserve(),
            expected.rent_exempt_reserve
        );
        // authorized
        prop_assert_eq!(actual.stake_meta_authorized(), expected.authorized);
        prop_assert_eq!(
            actual.stake_meta_authorized_staker(),
            expected.authorized.staker
        );
        prop_assert_eq!(
            actual.stake_meta_authorized_withdrawer(),
            expected.authorized.withdrawer
        );
        prop_assert_eq!(
            actual.stake_meta_authorized_staker(),
            expected.authorized.staker
        );
        // lockup
        prop_assert_eq!(actual.stake_meta_lockup(), expected.lockup);
        prop_assert_eq!(
            actual.stake_meta_lockup_unix_timestamp(),
            expected.lockup.unix_timestamp
        );
        prop_assert_eq!(actual.stake_meta_lockup_epoch(), expected.lockup.epoch);
        prop_assert_eq!(
            actual.stake_meta_lockup_custodian(),
            expected.lockup.custodian
        );
        prop_assert_eq!(
            actual.stake_lockup_is_in_force(clock),
            expected.lockup.is_in_force(clock, None)
        );
        Ok(())
    }

    proptest! {
        #[allow(deprecated)]
        #[test]
        fn stake_readonly_matches_full_deser_valid(stake_state in stake_state(), clock in clock()) {
            let mut data = vec![0u8; StakeState::size_of()];
            stake_state
                .serialize(&mut data.as_mut_slice())
                .unwrap();
            let account = ReadonlyStakeAccount(AccountData(&data));
            prop_assert!(account.stake_data_is_valid());
            let account = account.try_into_valid().unwrap();
            match stake_state {
                StakeState::Uninitialized => prop_assert_eq!(account.stake_state_marker(), StakeStateMarker::Uninitialized),
                StakeState::Initialized(meta) => assert_meta_eq(&account.try_into_stake_or_initialized().unwrap(), &meta, &clock).unwrap(),
                StakeState::Stake(meta, stake) => {
                    let account = account.try_into_stake_or_initialized().unwrap();
                    assert_meta_eq(&account, &meta, &clock).unwrap();
                    let account = account.try_into_stake().unwrap();
                    prop_assert_eq!(account.stake_stake(), stake);
                    // delegation
                    prop_assert_eq!(account.stake_stake_delegation(), stake.delegation);
                    prop_assert_eq!(account.stake_stake_delegation_voter_pubkey(), stake.delegation.voter_pubkey);
                    prop_assert_eq!(account.stake_stake_delegation_stake(), stake.delegation.stake);
                    prop_assert_eq!(account.stake_stake_delegation_activation_epoch(), stake.delegation.activation_epoch);
                    prop_assert_eq!(account.stake_stake_delegation_deactivation_epoch(), stake.delegation.deactivation_epoch);
                    prop_assert_eq!(
                        account.stake_stake_delegation_warmup_cooldown_rate_deprecated(),
                        stake.delegation.warmup_cooldown_rate
                    );

                    prop_assert_eq!(account.stake_stake_credits_observed(), stake.credits_observed);

                    prop_assert_eq!(account.stake_is_bootstrap(), stake.delegation.is_bootstrap());
                },
                StakeState::RewardsPool => prop_assert_eq!(account.stake_state_marker(), StakeStateMarker::RewardsPool),
            }
        }
    }
}
