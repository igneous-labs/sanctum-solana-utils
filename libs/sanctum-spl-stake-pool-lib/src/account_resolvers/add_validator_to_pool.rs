use std::num::NonZeroU32;

use solana_program::{
    borsh0_10::try_from_slice_unchecked, program_error::ProgramError, pubkey::Pubkey, stake,
    system_program, sysvar,
};
use solana_readonly_account::{ReadonlyAccountData, ReadonlyAccountPubkey};
use spl_stake_pool_interface::{AccountType, AddValidatorToPoolKeys, StakePool};

use crate::{FindValidatorStakeAccount, FindValidatorStakeAccountArgs, FindWithdrawAuthority};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AddValidatorToPoolPdas {
    pub withdraw_authority: Pubkey,
    pub validator_stake_account: Pubkey,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AddValidatorToPoolFreeArgs<P> {
    pub stake_pool: P,
    pub vote_account: Pubkey,
}

impl<P: ReadonlyAccountData + ReadonlyAccountPubkey> AddValidatorToPoolFreeArgs<P> {
    pub fn resolve_with_pdas(
        &self,
        AddValidatorToPoolPdas {
            withdraw_authority,
            validator_stake_account,
        }: AddValidatorToPoolPdas,
    ) -> Result<AddValidatorToPoolKeys, ProgramError> {
        let StakePool {
            account_type,
            staker,
            validator_list,
            reserve_stake,
            ..
        } = try_from_slice_unchecked(&self.stake_pool.data())?;
        if account_type != AccountType::StakePool {
            return Err(ProgramError::InvalidAccountData);
        }
        Ok(AddValidatorToPoolKeys {
            stake_pool: *self.stake_pool.pubkey(),
            staker,
            reserve_stake,
            withdraw_authority,
            validator_list,
            validator_stake_account,
            vote_account: self.vote_account,
            rent: sysvar::rent::ID,
            clock: sysvar::clock::ID,
            stake_history: sysvar::stake_history::ID,
            stake_config: stake::config::ID,
            system_program: system_program::ID,
            stake_program: stake::program::ID,
        })
    }

    pub fn resolve_for_prog(
        &self,
        program_id: &Pubkey,
        seed: Option<NonZeroU32>,
    ) -> Result<AddValidatorToPoolKeys, ProgramError> {
        let withdraw_authority = FindWithdrawAuthority {
            pool: *self.stake_pool.pubkey(),
        }
        .run_for_prog(program_id)
        .0;
        let validator_stake_account =
            FindValidatorStakeAccount::new(FindValidatorStakeAccountArgs {
                pool: *self.stake_pool.pubkey(),
                vote: self.vote_account,
                seed,
            })
            .run_for_prog(program_id)
            .0;
        self.resolve_with_pdas(AddValidatorToPoolPdas {
            withdraw_authority,
            validator_stake_account,
        })
    }
}
