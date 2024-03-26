use std::num::NonZeroU32;

use solana_program::{program_error::ProgramError, pubkey::Pubkey, stake, system_program, sysvar};
use solana_readonly_account::{ReadonlyAccountData, ReadonlyAccountPubkey};
use spl_stake_pool_interface::{
    DecreaseAdditionalValidatorStakeKeys, StakePool, ValidatorStakeInfo,
};

use crate::{
    deserialize_stake_pool_checked, FindEphemeralStakeAccount, FindEphemeralStakeAccountArgs,
    FindTransientStakeAccount, FindTransientStakeAccountArgs, FindValidatorStakeAccount,
    FindValidatorStakeAccountArgs, FindWithdrawAuthority,
};

use super::{AdditionalValidatorStakePdas, AdditionalValidatorStakeSeeds, ProgramIdAndVote};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DecreaseAdditionalValidatorStake<P> {
    pub stake_pool: P,
}

impl<P: ReadonlyAccountData + ReadonlyAccountPubkey> DecreaseAdditionalValidatorStake<P> {
    pub fn resolve_with_pdas(
        &self,
        AdditionalValidatorStakePdas {
            withdraw_authority,
            validator_stake_account,
            transient_stake_account,
            ephemeral_stake_account,
        }: AdditionalValidatorStakePdas,
    ) -> Result<DecreaseAdditionalValidatorStakeKeys, ProgramError> {
        let StakePool {
            staker,
            validator_list,
            reserve_stake,
            ..
        } = deserialize_stake_pool_checked(self.stake_pool.data().as_ref())?;
        Ok(DecreaseAdditionalValidatorStakeKeys {
            stake_pool: *self.stake_pool.pubkey(),
            staker,
            withdraw_authority,
            validator_list,
            reserve_stake,
            ephemeral_stake_account,
            transient_stake_account,
            validator_stake_account,
            clock: sysvar::clock::ID,
            stake_history: sysvar::stake_history::ID,
            system_program: system_program::ID,
            stake_program: stake::program::ID,
        })
    }

    pub fn resolve_for_prog_with_seeds(
        &self,
        ProgramIdAndVote {
            program_id,
            vote_account,
        }: ProgramIdAndVote,
        AdditionalValidatorStakeSeeds {
            validator,
            transient,
            ephemeral,
        }: AdditionalValidatorStakeSeeds,
    ) -> Result<DecreaseAdditionalValidatorStakeKeys, ProgramError> {
        let (withdraw_authority, _bump) = FindWithdrawAuthority {
            pool: *self.stake_pool.pubkey(),
        }
        .run_for_prog(&program_id);
        let (validator_stake_account, _bump) =
            FindValidatorStakeAccount::new(FindValidatorStakeAccountArgs {
                pool: *self.stake_pool.pubkey(),
                vote: vote_account,
                seed: validator,
            })
            .run_for_prog(&program_id);
        let (transient_stake_account, _bump) =
            FindTransientStakeAccount::new(FindTransientStakeAccountArgs {
                pool: *self.stake_pool.pubkey(),
                vote: vote_account,
                seed: transient,
            })
            .run_for_prog(&program_id);
        let (ephemeral_stake_account, _bump) =
            FindEphemeralStakeAccount::new(FindEphemeralStakeAccountArgs {
                pool: *self.stake_pool.pubkey(),
                seed: ephemeral,
            })
            .run_for_prog(&program_id);
        self.resolve_with_pdas(AdditionalValidatorStakePdas {
            withdraw_authority,
            validator_stake_account,
            transient_stake_account,
            ephemeral_stake_account,
        })
    }

    pub fn resolve_for_validator(
        &self,
        program_id: Pubkey,
        ValidatorStakeInfo {
            transient_seed_suffix,
            validator_seed_suffix,
            vote_account_address,
            ..
        }: &ValidatorStakeInfo,
        ephemeral_seed: u64,
    ) -> Result<DecreaseAdditionalValidatorStakeKeys, ProgramError> {
        self.resolve_for_prog_with_seeds(
            ProgramIdAndVote {
                program_id,
                vote_account: *vote_account_address,
            },
            AdditionalValidatorStakeSeeds {
                validator: NonZeroU32::new(*validator_seed_suffix),
                transient: *transient_seed_suffix,
                ephemeral: ephemeral_seed,
            },
        )
    }
}
