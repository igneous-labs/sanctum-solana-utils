use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AccountType {
    Uninitialized,
    StakePool,
    ValidatorList,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Lockup {
    pub unix_timestamp: i64,
    pub epoch: u64,
    pub custodian: Pubkey,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Fee {
    pub denominator: u64,
    pub numerator: u64,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FutureEpochFee {
    None,
    One { fee: Fee },
    Two { fee: Fee },
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum StakeStatus {
    Active,
    DeactivatingTransient,
    ReadyForRemoval,
    DeactivatingValidator,
    DeactivatingAll,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ValidatorListHeader {
    pub account_type: AccountType,
    pub max_validators: u32,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ValidatorStakeInfo {
    pub active_stake_lamports: u64,
    pub transient_stake_lamports: u64,
    pub last_update_epoch: u64,
    pub transient_seed_suffix: u64,
    pub unused: u32,
    pub validator_seed_suffix: u32,
    pub status: StakeStatus,
    pub vote_account_address: Pubkey,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FeeType {
    SolReferral { fee: u8 },
    StakeReferral { fee: u8 },
    Epoch { fee: Fee },
    StakeWithdrawal { fee: Fee },
    SolDeposit { fee: Fee },
    StakeDeposit { fee: Fee },
    SolWithdrawal { fee: Fee },
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FundingType {
    StakeDeposit,
    SolDeposit,
    SolWithdraw,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AdditionalValidatorStakeArgs {
    pub lamports: u64,
    pub transient_stake_seed: u64,
    pub ephemeral_stake_seed: u64,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PreferredValidatorType {
    Deposit,
    Withdraw,
}
