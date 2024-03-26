use std::num::NonZeroU32;

use borsh::BorshDeserialize;
use solana_program::{program_error::ProgramError, pubkey::Pubkey, stake, sysvar};
use solana_readonly_account::{ReadonlyAccountData, ReadonlyAccountPubkey};
use spl_stake_pool_interface::{AccountType, RemoveValidatorFromPoolKeys, StakePool};

use crate::{
    FindTransientStakeAccount, FindTransientStakeAccountArgs, FindValidatorStakeAccount,
    FindValidatorStakeAccountArgs, FindWithdrawAuthority,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RemoveValidatorFromPoolFreeArgs<P> {
    pub stake_pool: P,
    pub vote_account: Pubkey,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RemoveValidatorFromPoolPdas {
    pub withdraw_authority: Pubkey,
    pub validator_stake_account: Pubkey,
    pub transient_stake_account: Pubkey,
}

impl<P: ReadonlyAccountData + ReadonlyAccountPubkey> RemoveValidatorFromPoolFreeArgs<P> {
    pub fn resolve_with_pdas(
        &self,
        RemoveValidatorFromPoolPdas {
            withdraw_authority,
            validator_stake_account,
            transient_stake_account,
        }: RemoveValidatorFromPoolPdas,
    ) -> Result<RemoveValidatorFromPoolKeys, ProgramError> {
        let StakePool {
            account_type,
            staker,
            validator_list,
            ..
        } = StakePool::deserialize(&mut self.stake_pool.data().as_ref())?;
        if account_type != AccountType::StakePool {
            return Err(ProgramError::InvalidAccountData);
        }
        Ok(RemoveValidatorFromPoolKeys {
            stake_pool: *self.stake_pool.pubkey(),
            staker,
            withdraw_authority,
            validator_list,
            validator_stake_account,
            clock: sysvar::clock::ID,
            stake_program: stake::program::ID,
            transient_stake_account,
        })
    }

    pub fn resolve_for_prog(
        &self,
        program_id: &Pubkey,
        vsa_seed: Option<NonZeroU32>,
        transient_seed: u64,
    ) -> Result<RemoveValidatorFromPoolKeys, ProgramError> {
        let (withdraw_authority, _bump) = FindWithdrawAuthority {
            pool: *self.stake_pool.pubkey(),
        }
        .run_for_prog(program_id);
        let (validator_stake_account, _bump) =
            FindValidatorStakeAccount::new(FindValidatorStakeAccountArgs {
                pool: *self.stake_pool.pubkey(),
                vote: self.vote_account,
                seed: vsa_seed,
            })
            .run_for_prog(program_id);
        let (transient_stake_account, _bump) =
            FindTransientStakeAccount::new(FindTransientStakeAccountArgs {
                pool: *self.stake_pool.pubkey(),
                vote: self.vote_account,
                seed: transient_seed,
            })
            .run_for_prog(program_id);
        self.resolve_with_pdas(RemoveValidatorFromPoolPdas {
            withdraw_authority,
            validator_stake_account,
            transient_stake_account,
        })
    }
}
